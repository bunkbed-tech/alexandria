use futures::Future;
use std::{collections::HashSet, slice::Chunks, str::FromStr};

use actix_web::{get, web::Query, HttpResponse, Responder};
use quick_xml::de::from_str;
use reqwest::get as rget;
use serde::{Deserialize, Deserializer, Serialize};

use models::AlexandriaResource;

use crate::http::respond;

#[get("/search")]
pub async fn bgg_search(Query(params): Query<QueryParams>) -> impl Responder {
    respond(search_bgg(params.query).await)
}

#[get("/things")]
pub async fn bgg_things_list(Query(params): Query<IdsParams>) -> impl Responder {
    respond(list_bgg_things(params.ids).await)
}

#[get("/things/search")]
pub async fn bgg_things_search(Query(params): Query<QueryParams>) -> impl Responder {
    let (resources, errors) = search_bgg_things(params.query).await;
    HttpResponse::Ok().json(SearchThingsResults { resources, errors })
}

async fn search_bgg(query: String) -> Result<Vec<i32>, String> {
    let search_xml = bgg_search_xml(query).await?;
    let search_items: SearchItems = from_str(&search_xml).map_err(|err| {
        String::from("[search_bgg:search_items:from_str] ") + &err.to_string() + &search_xml
    })?;
    // The search API returns duplicates (no idea why), so we deduplicate with a HashSet and sort for a guaranteed order
    Ok(search_items
        .item
        .unwrap_or_else(Vec::new)
        .into_iter()
        .map(|item| item.id.parse::<i32>().expect("Not a valid ID"))
        .collect::<HashSet<i32>>()
        .into_iter()
        .collect::<Vec<_>>())
}

async fn list_bgg_things(ids: Vec<i32>) -> Result<Vec<AlexandriaResource>, String> {
    let thing_xml = bgg_thing_xml(ids.clone()).await?;
    let thing_items: ThingItems = from_str(&thing_xml).map_err(|err| {
        String::from("[list_bgg_things:thing_items:from_str] ") + &err.to_string() + &thing_xml
    })?;
    let mut resources: Vec<_> = thing_items
        .item
        .unwrap_or_else(Vec::new)
        .into_iter()
        .filter(|thing| thing.name.is_some())
        .zip(ids)
        .map(|(thing, id)| AlexandriaResource {
            id: None,
            title: thing
                .name
                .expect("")
                .iter()
                .filter(|name| name.name_type == NameType::Primary)
                .next()
                .map(|name| name.value.clone())
                .expect("Primary name does not exist"),
            description: thing.description.map(|description| description.value),
            year_published: thing
                .yearpublished
                .map(|year| year.value.parse::<i32>().expect("Not a valid year")),
            thumbnail: thing.thumbnail.map(|thumbnail| thumbnail.value),
            api_id: id,
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    // BGG doesn't return things in a reliable order, so we sort by api_id
    resources.sort_by_key(|resource| resource.api_id);
    Ok(resources)
}

pub async fn search_bgg_things(query: String) -> (Vec<AlexandriaResource>, Vec<String>) {
    match search_bgg(query).await {
        Ok(resource_ids) => {
            // BGG /thing API has a limit of 20 IDs, so we chunk the IDs
            // We also limit the number of resources to 100 so as not to take too long
            let maximum_resource_ids = &resource_ids[..100.min(resource_ids.len())];
            let chunk_results: Vec<Result<Vec<AlexandriaResource>, String>> =
                futures::future::join_all(list_bgg_things_chunks(maximum_resource_ids.chunks(20)))
                    .await;
            let (successes, errors): (
                Vec<Result<Vec<AlexandriaResource>, String>>,
                Vec<Result<Vec<AlexandriaResource>, String>>,
            ) = chunk_results.into_iter().partition(|result| result.is_ok());
            let successes: Vec<AlexandriaResource> = successes
                .into_iter()
                .map(|r| r.unwrap())
                .flatten()
                .collect();
            let errors: Vec<String> = errors.into_iter().map(|r| r.unwrap_err()).collect();
            (successes, errors)
        }
        Err(error) => {
            let empty_results: Vec<AlexandriaResource> = Vec::new();
            (empty_results, vec![error])
        }
    }
}

async fn bgg_thing_xml(ids: Vec<i32>) -> Result<String, String> {
    rget(format!(
        "https://boardgamegeek.com/xmlapi2/thing?id={}",
        ids.iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",")
    ))
    .await
    .map_err(|err| String::from("[bgg_thing_xml:_:get] ") + &err.to_string())?
    .text()
    .await
    .map_err(|err| String::from("[bgg_thing_xml:_:text] ") + &err.to_string())
}

async fn bgg_search_xml(query: String) -> Result<String, String> {
    rget(format!(
        "https://boardgamegeek.com/xmlapi2/search?query={}",
        query
    ))
    .await
    .map_err(|err| String::from("[bgg_search_xml:_:get] ") + &err.to_string())?
    .text()
    .await
    .map_err(|err| String::from("[bgg_search_xml:_:text] ") + &err.to_string())
}

// We couldn't include "impl Future" outside of a function signature, so we had to write this -_-
fn list_bgg_things_chunks(
    chunks: Chunks<'_, i32>,
) -> Vec<impl Future<Output = Result<Vec<AlexandriaResource>, String>>> {
    chunks
        .map(|chunk| list_bgg_things(chunk.to_vec()))
        .collect()
}

fn csv_ids<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let string: &str = Deserialize::deserialize(deserializer)?;
    let result: Result<Vec<i32>, _> = string.split(",").map(|v| i32::from_str(v.trim())).collect();
    result.map_err(serde::de::Error::custom)
}

#[derive(Deserialize)]
struct IdsParams {
    #[serde(deserialize_with = "csv_ids")]
    ids: Vec<i32>,
}

#[derive(Deserialize)]
struct QueryParams {
    query: String,
}

#[derive(Serialize)]
struct SearchThingsResults {
    resources: Vec<AlexandriaResource>,
    errors: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Attribute {
    #[serde(rename = "@value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct SearchItem {
    #[serde(rename = "@id")]
    id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct SearchItems {
    item: Option<Vec<SearchItem>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct InnerText {
    #[serde(rename = "$text")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
enum NameType {
    Primary,
    Alternate,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct NameAttribute {
    #[serde(rename = "@value")]
    value: String,
    #[serde(rename = "@type")]
    name_type: NameType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct ThingItem {
    description: Option<InnerText>,
    thumbnail: Option<InnerText>,
    yearpublished: Option<Attribute>,
    name: Option<Vec<NameAttribute>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct ThingItems {
    item: Option<Vec<ThingItem>>,
}

#[cfg(test)]
mod tests {
    use actix_web::test;
    use regex::Regex;

    use super::*;

    fn trim_xml(xml: &String) -> String {
        let re1 = Regex::new(r">\s+<").unwrap();
        let trimmed_xml1 = re1.replace_all(&xml, "><");
        let re2 = Regex::new(r"\s/>").unwrap();
        let trimmed_xml2 = re2.replace_all(&trimmed_xml1, "/>");
        trimmed_xml2.to_string()
    }

    #[test]
    async fn test_bgg_search_xml() {
        let xml = bgg_search_xml(String::from("scythe")).await.unwrap();
        let trimmed_xml = trim_xml(&xml);
        let items = [
            r#"<item type="boardgame" id="398158"><name type="primary" value="Grind House: Scythes Out"/><yearpublished value="2023"/></item>"#,
            r#"<item type="boardgame" id="226320"><name type="primary" value="My Little Scythe"/><yearpublished value="2017"/></item>"#,
        ];
        assert!(items.iter().all(|item| trimmed_xml.contains(item)));
    }

    #[test]
    async fn test_bgg_thing_xml() {
        let xml = bgg_thing_xml(vec![398158]).await.unwrap();
        let trimmed_xml = trim_xml(&xml);
        let items = [
            r#"<name type="primary" sortindex="1" value="Grind House: Scythes Out"/>"#,
            r#"<description>The secrets of Grind House deepen in this new expansion that promises greater rewards&amp;hellip;if you can stand the risk.&amp;#10;The Host has opened a new wing of the mansion for you to come explore, and your invitation included a little something extra this time around. But did the other players receive another message too? You&amp;rsquo;d better stay sharp if you want to win Scythes Out.&amp;#10;&amp;#10;New Thematic Mechanic: Promised Inheritance tokens. The Host has invited you all with a promise, but you&amp;rsquo;ll soon find, that some promises are better than others. Rooms offer the ability to swap tokens between yourselves and other players in hopes of ending as the favored player.&amp;#10;&amp;#10;&amp;mdash;description from the designer&amp;#10;&amp;#10;</description>"#,
            r#"<thumbnail>https://cf.geekdo-images.com/jDB5KAU3JF2YYpBM4mHnKw__thumb/img/_2iVW-Dht0AIbnwXslMJEcl7mA8=/fit-in/200x150/filters:strip_icc()/pic7678523.png</thumbnail>"#,
            r#"<yearpublished value="2023"/>"#,
        ];
        assert!(items.iter().all(|item| trimmed_xml.contains(item)));
    }

    #[test]
    async fn test_search_bgg() {
        let query = String::from("Cranium Cadoo");
        let mut resource_ids = search_bgg(query).await.unwrap();
        // BGG /search seems to return resources in a non-deterministic order, so we sort to ensure the test works
        resource_ids.sort();
        let expected_resource_ids = vec![6420, 14454];
        assert_eq!(resource_ids, expected_resource_ids);
    }

    #[test]
    async fn test_search_bgg_empty() {
        let query = String::from("sdlkajlslkshlddk");
        let resource_ids = search_bgg(query).await.unwrap();
        let expected_resource_ids = Vec::<i32>::new();
        assert_eq!(resource_ids, expected_resource_ids);
    }

    #[test]
    async fn test_list_bgg_things() {
        let ids = vec![14454, 6420];
        let resources = list_bgg_things(ids).await.unwrap();
        let expected_resources = vec![
            AlexandriaResource {
                id: None,
                api_id: 6420,
                title: String::from("Cranium Cadoo"),
                description: Some(String::from("A version of Cranium &quot;scaled down&quot; for kids, although the game should still appeal to adults who like Cranium.  Here's the manufacturer's information:&#10;&#10;&quot;With a variety of hilarious activities, Cranium Cadoo gets kids thinking, creating, giggling, grinning, and laughing like crazy as they try to get four in a row to win. With so many different activities, there is something in Cranium Cadoo that will make every kid hoot and high-five. They might even discover a talent they never knew they had!&#10;&#10;And kids just love the cool Cranium Clay, funky tokens, and especially the Secret Decoder Mask. Whether kids love to act, puzzle, sketch, sculpt, or even crack secret codes, Cranium Cadoo has something for everyone&hellip;including you!&quot;&#10;&#10;")),
                year_published: Some(2001),
                thumbnail: Some(String::from("https://cf.geekdo-images.com/hQI6W-7HwKty4c5yLFP-Aw__thumb/img/_IyE4nIyGh7_PVfGCarLoNmDMGc=/fit-in/200x150/filters:strip_icc()/pic3335930.jpg")),
            },
            AlexandriaResource {
                id: None,
                api_id: 14454,
                title: String::from("Cranium Cadoo Booster Box"),
                description: Some(String::from("Booster box with 300 new cards, Clay, secret decoder mask and drawing pad.&#10;&#10;Expands:&#10;&#10;    Cranium Cadoo&#10;&#10;&#10;")),
                year_published: Some(2001),
                thumbnail: Some(String::from("https://cf.geekdo-images.com/jboSqbHm5jcQp7XJZPM-vw__thumb/img/v6dQ2IqIdGJIX19AVEZDSaQ5Nms=/fit-in/200x150/filters:strip_icc()/pic58689.jpg")),
            },
        ];
        assert_eq!(resources, expected_resources);
    }

    #[test]
    async fn test_list_bgg_things_invalid() {
        let ids = vec![0];
        let resources = list_bgg_things(ids).await.unwrap();
        let expected_resources = Vec::<AlexandriaResource>::new();
        assert_eq!(resources, expected_resources);
    }
}
