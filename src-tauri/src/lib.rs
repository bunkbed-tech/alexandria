use std::collections::HashSet;
use std::env::var;

use quick_xml::de::from_str;
use reqwest::get;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tauri::{command, State};

mod models;
use crate::models::Resource;

async fn bgg_thing_xml(ids: &Vec<i32>) -> Result<String, String> {
    get(format!(
        "https://boardgamegeek.com/xmlapi2/thing?id={}",
        ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",")
    ))
    .await
    .map_err(|err| String::from("[bgg_thing_xml:_:get] ") + &err.to_string())?
    .text()
    .await
    .map_err(|err| String::from("[bgg_thing_xml:_:text] ") + &err.to_string())
}

async fn bgg_search_xml(query: &String) -> Result<String, String> {
    get(format!(
        "https://boardgamegeek.com/xmlapi2/search?query={}",
        query
    ))
    .await
    .map_err(|err| String::from("[bgg_search_xml:_:get] ") + &err.to_string())?
    .text()
    .await
    .map_err(|err| String::from("[bgg_search_xml:_:text] ") + &err.to_string())
}

#[command]
async fn search_bgg(query: String) -> Result<Vec<i32>, String> {
    let search_xml = bgg_search_xml(&query).await?;
    let search_items: SearchItems = from_str(&search_xml).map_err(|err| String::from("[search_bgg:search_items:from_str] ") + &err.to_string() + &search_xml)?;
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

#[command]
async fn list_bgg_things(ids: Vec<i32>) -> Result<Vec<Resource>, String> {
    let thing_xml = bgg_thing_xml(&ids).await?;
    let thing_items: ThingItems = from_str(&thing_xml).map_err(|err| String::from("[list_bgg_things:thing_items:from_str] ") + &err.to_string() + &thing_xml)?;
    let mut resources: Vec<_> = thing_items
        .item
        .unwrap_or_else(Vec::new)
        .into_iter()
        .filter(|thing| thing.name.is_some())
        .zip(ids)
        .map(|(thing, id)| Resource {
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
            bgg_id: id,
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    // BGG doesn't return things in a reliable order, so we sort by bgg_id
    resources.sort_by_key(|resource| resource.bgg_id);
    Ok(resources)
}

// We couldn't include "impl Future" outside of a function signature, so we had to write this -_-
fn list_bgg_things_chunks(chunks: std::slice::Chunks<'_, i32>) -> Vec<impl futures::Future<Output = Result<Vec<Resource>, String>>> {
    chunks.map(|chunk| list_bgg_things(chunk.to_vec())).collect()
}

#[command]
async fn search_bgg_things(query: String) -> (Vec<Resource>, Vec<String>) {
    match search_bgg(query).await {
        Ok(resource_ids) => {
            // BGG /thing API has a limit of 20 IDs, so we chunk the IDs
            let chunk_results: Vec<Result<Vec<Resource>, String>> = futures::future::join_all(list_bgg_things_chunks(resource_ids.chunks(20))).await;
            let (successes, errors): (Vec<Result<Vec<Resource>, String>>, Vec<Result<Vec<Resource>, String>>) = chunk_results.into_iter().partition(|result| result.is_ok());
            let successes: Vec<Resource> = successes.into_iter().map(|r| r.unwrap()).flatten().collect();
            let errors: Vec<String> = errors.into_iter().map(|r| r.unwrap_err()).collect();
            (successes, errors)
        },
        Err(error) => {
            let empty_results: Vec<Resource> = Vec::new();
            (empty_results, vec![error])
        },
    }
}

async fn _list_resources(
    pool: &PgPool,
    ids: Option<Vec<i32>>,
) -> Result<Vec<Resource>, String> {
    let rows: Vec<Resource>;
    if let Some(bgg_ids) = ids {
        rows = sqlx::query_as!(Resource, r#"SELECT * FROM resource WHERE bgg_id = ANY($1)"#, &bgg_ids)
            .fetch_all(pool)
            .await
            .expect("Unable to list resources")
    } else {
        rows = sqlx::query_as!(Resource, r#"SELECT * FROM resource"#)
            .fetch_all(pool)
            .await
            .expect("Unable to list resources")
    }
    Ok(rows)
}

#[command]
async fn list_resources(
    state: State<'_, PgPoolWrapper>,
    ids: Option<Vec<i32>>,
) -> Result<Vec<Resource>, String> {
    _list_resources(&state.pool, ids).await
}

async fn _track_resource(
    pool: &PgPool,
    resource: Resource,
) -> Result<Resource, String> {
    if let Some(_) = resource.id {
        return Err(format!("Resource {} is already tracked.", resource))
    }
    let db_resource = {
        sqlx::query_as!(
            Resource,
            r#"INSERT INTO resource (title, description, year_published, thumbnail, bgg_id) VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
            resource.title,
            resource.description,
            resource.year_published,
            resource.thumbnail,
            resource.bgg_id,
        ).fetch_one(pool)
        .await
        .map_err(|err| String::from("[track_resource:db_resource:fetch_one] ") + &err.to_string())?
    };
    Ok(db_resource)
}

#[command]
async fn track_resource(
    state: State<'_, PgPoolWrapper>,
    resource: Resource,
) -> Result<Resource, String> {
    _track_resource(&state.pool, resource).await
}

async fn _untrack_resource(
    pool: &PgPool,
    mut resource: Resource,
) -> Result<Resource, String> {
    if resource.id == None {
        return Err(format!("Resource {} is already untracked.", resource))
    }
    sqlx::query_as!(
        Resource,
        r#"DELETE FROM resource WHERE id = $1"#,
        resource.id,
    )
    .execute(pool)
    .await
    .map_err(|err| String::from("[untrack_resource:_:execute] ") + &err.to_string())?;
    resource.id = None;
    Ok(resource)
}

#[command]
async fn untrack_resource(
    state: State<'_, PgPoolWrapper>,
    resource: Resource,
) -> Result<Resource, String> {
    _untrack_resource(&state.pool, resource).await
}

struct PgPoolWrapper {
    pub pool: PgPool,
}

async fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>, pool: PgPool) {
    builder
        .manage(PgPoolWrapper { pool })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            list_resources,
            search_bgg,
            list_bgg_things,
            search_bgg_things,
            track_resource,
            untrack_resource,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let database_url =
        var("DATABASE_URL").expect("DATABASE_URL must be set to connect to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a postgres connection pool");

    create_app(tauri::Builder::default(), pool).await;
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
    use regex::Regex;

    use super::*;

    fn trim_xml(xml: &String) -> String {
        let re1 = Regex::new(r">\s+<").unwrap();
        let trimmed_xml1 = re1.replace_all(&xml, "><");
        let re2 = Regex::new(r"\s/>").unwrap();
        let trimmed_xml2 = re2.replace_all(&trimmed_xml1, "/>");
        trimmed_xml2.to_string()
    }

    #[async_std::test]
    async fn test_bgg_search_xml() {
        let xml = bgg_search_xml(&String::from("scythe")).await.unwrap();
        let trimmed_xml = trim_xml(&xml);
        let items = [
            r#"<item type="boardgame" id="398158"><name type="primary" value="Grind House: Scythes Out"/><yearpublished value="2023"/></item>"#,
            r#"<item type="boardgame" id="226320"><name type="primary" value="My Little Scythe"/><yearpublished value="2017"/></item>"#,
        ];
        assert!(items.iter().all(|item| trimmed_xml.contains(item)));
    }

    #[async_std::test]
    async fn test_bgg_thing_xml() {
        let xml = bgg_thing_xml(&vec![398158]).await.unwrap();
        let trimmed_xml = trim_xml(&xml);
        let items = [
            r#"<name type="primary" sortindex="1" value="Grind House: Scythes Out"/>"#,
            r#"<description>The secrets of Grind House deepen in this new expansion that promises greater rewards&amp;hellip;if you can stand the risk.&amp;#10;The Host has opened a new wing of the mansion for you to come explore, and your invitation included a little something extra this time around. But did the other players receive another message too? You&amp;rsquo;d better stay sharp if you want to win Scythes Out.&amp;#10;&amp;#10;New Thematic Mechanic: Promised Inheritance tokens. The Host has invited you all with a promise, but you&amp;rsquo;ll soon find, that some promises are better than others. Rooms offer the ability to swap tokens between yourselves and other players in hopes of ending as the favored player.&amp;#10;&amp;#10;&amp;mdash;description from the designer&amp;#10;&amp;#10;</description>"#,
            r#"<thumbnail>https://cf.geekdo-images.com/jDB5KAU3JF2YYpBM4mHnKw__thumb/img/_2iVW-Dht0AIbnwXslMJEcl7mA8=/fit-in/200x150/filters:strip_icc()/pic7678523.png</thumbnail>"#,
            r#"<yearpublished value="2023"/>"#,
        ];
        assert!(items.iter().all(|item| trimmed_xml.contains(item)));
    }

    #[async_std::test]
    async fn test_search_bgg() {
        let query = String::from("Cranium Cadoo");
        let mut resource_ids = search_bgg(query).await.unwrap();
        // BGG /search seems to return resources in a non-deterministic order, so we sort to ensure the test works
        resource_ids.sort();
        let expected_resource_ids = vec![6420, 14454];
        assert_eq!(resource_ids, expected_resource_ids);
    }

    #[async_std::test]
    async fn test_search_bgg_empty() {
        let query = String::from("sdlkajlslkshlddk");
        let resource_ids = search_bgg(query).await.unwrap();
        let expected_resource_ids = Vec::<i32>::new();
        assert_eq!(resource_ids, expected_resource_ids);
    }

    #[async_std::test]
    async fn test_list_bgg_things() {
        let ids = vec![14454, 6420];
        let resources = list_bgg_things(ids).await.unwrap();
        let expected_resources = vec![
            Resource {
                id: None,
                bgg_id: 6420,
                title: String::from("Cranium Cadoo"),
                description: Some(String::from("A version of Cranium &quot;scaled down&quot; for kids, although the game should still appeal to adults who like Cranium.  Here's the manufacturer's information:&#10;&#10;&quot;With a variety of hilarious activities, Cranium Cadoo gets kids thinking, creating, giggling, grinning, and laughing like crazy as they try to get four in a row to win. With so many different activities, there is something in Cranium Cadoo that will make every kid hoot and high-five. They might even discover a talent they never knew they had!&#10;&#10;And kids just love the cool Cranium Clay, funky tokens, and especially the Secret Decoder Mask. Whether kids love to act, puzzle, sketch, sculpt, or even crack secret codes, Cranium Cadoo has something for everyone&hellip;including you!&quot;&#10;&#10;")),
                year_published: Some(2001),
                thumbnail: Some(String::from("https://cf.geekdo-images.com/hQI6W-7HwKty4c5yLFP-Aw__thumb/img/_IyE4nIyGh7_PVfGCarLoNmDMGc=/fit-in/200x150/filters:strip_icc()/pic3335930.jpg")),
            },
            Resource {
                id: None,
                bgg_id: 14454,
                title: String::from("Cranium Cadoo Booster Box"),
                description: Some(String::from("Booster box with 300 new cards, Clay, secret decoder mask and drawing pad.&#10;&#10;Expands:&#10;&#10;    Cranium Cadoo&#10;&#10;&#10;")),
                year_published: Some(2001),
                thumbnail: Some(String::from("https://cf.geekdo-images.com/jboSqbHm5jcQp7XJZPM-vw__thumb/img/v6dQ2IqIdGJIX19AVEZDSaQ5Nms=/fit-in/200x150/filters:strip_icc()/pic58689.jpg")),
            },
        ];
        assert_eq!(resources, expected_resources);
    }

    #[async_std::test]
    async fn test_list_bgg_things_invalid() {
        let ids = vec![0];
        let resources = list_bgg_things(ids).await.unwrap();
        let expected_resources = Vec::<Resource>::new();
        assert_eq!(resources, expected_resources);
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("resources")))]
    async fn test_list_resources_all(pool: PgPool) {
        let ids: Option<Vec<i32>> = None;
        let resources = _list_resources(&pool, ids).await.unwrap();
        let expected_resources = vec![
          Resource {
              id: Some(1),
              title: String::from("Scythe"),
              description: Some(String::from("Really good game")),
              year_published: Some(2015),
              thumbnail: Some(String::from("https://google.com")),
              bgg_id: 9000,
          },
          Resource {
              id: Some(2),
              title: String::from("Cranium Cadoo"),
              description: None,
              year_published: None,
              thumbnail: None,
              bgg_id: 420,
          },
        ];
        assert_eq!(resources, expected_resources);
    }

    #[sqlx::test]
    async fn test_list_resources_missing(pool: PgPool) {
        let ids = Some(vec![1]);
        let resources = _list_resources(&pool, ids).await.unwrap();
        let expected_resources = Vec::<Resource>::new();
        assert_eq!(resources, expected_resources);
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("resources")))]
    async fn test_list_resources_some(pool: PgPool) {
        let ids = Some(vec![9000]);
        let resources = _list_resources(&pool, ids).await.unwrap();
        let expected_resources = vec![
          Resource {
              id: Some(1),
              title: String::from("Scythe"),
              description: Some(String::from("Really good game")),
              year_published: Some(2015),
              thumbnail: Some(String::from("https://google.com")),
              bgg_id: 9000,
          },
        ];
        assert_eq!(resources, expected_resources);
    }

    #[sqlx::test]
    async fn test_track_resource_untracked(pool: PgPool) {
        let untracked_resource = Resource {
            id: None,
            title: String::from("Scythe"),
            description: Some(String::from("Really good game")),
            year_published: Some(2015),
            thumbnail: Some(String::from("https://google.com")),
            bgg_id: 9000,
        };
        let tracked_resource = _track_resource(&pool, untracked_resource).await.unwrap();
        let expected_tracked_resource = Resource {
            id: Some(1),
            title: String::from("Scythe"),
            description: Some(String::from("Really good game")),
            year_published: Some(2015),
            thumbnail: Some(String::from("https://google.com")),
            bgg_id: 9000,
        };
        assert_eq!(tracked_resource, expected_tracked_resource);
    }

    #[sqlx::test]
    async fn test_track_resource_tracked(pool: PgPool) {
        let tracked_resource = Resource {
            id: Some(1),
            title: String::from("Scythe"),
            description: Some(String::from("Really good game")),
            year_published: Some(2015),
            thumbnail: Some(String::from("https://google.com")),
            bgg_id: 9000,
        };
        assert!(_track_resource(&pool, tracked_resource).await.is_err());
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("resources")))]
    async fn test_track_resource_invalid(pool: PgPool) {
        // This is invalid because the resource is already in the database, but id is None
        // This should probably only happen if we make a mistake in developing the app
        let untracked_resource = Resource {
            id: None,
            title: String::from("Scythe"),
            description: Some(String::from("Really good game")),
            year_published: Some(2015),
            thumbnail: Some(String::from("https://google.com")),
            bgg_id: 9000,
        };
        assert!(_track_resource(&pool, untracked_resource).await.is_err());
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("resources")))]
    async fn test_untrack_resource_tracked(pool: PgPool) {
        let tracked_resource = Resource {
            id: Some(1),
            title: String::from("Scythe"),
            description: Some(String::from("Really good game")),
            year_published: Some(2015),
            thumbnail: Some(String::from("https://google.com")),
            bgg_id: 9000,
        };
        let untracked_resource = _untrack_resource(&pool, tracked_resource).await.unwrap();
        let expected_untracked_resource = Resource {
            id: None,
            title: String::from("Scythe"),
            description: Some(String::from("Really good game")),
            year_published: Some(2015),
            thumbnail: Some(String::from("https://google.com")),
            bgg_id: 9000,
        };
        assert_eq!(untracked_resource, expected_untracked_resource);
    }

    #[sqlx::test]
    async fn test_untrack_resource_untracked(pool: PgPool) {
        let untracked_resource = Resource {
            id: None,
            title: String::from("Scythe"),
            description: Some(String::from("Really good game")),
            year_published: Some(2015),
            thumbnail: Some(String::from("https://google.com")),
            bgg_id: 9000,
        };
        assert!(_untrack_resource(&pool, untracked_resource).await.is_err());
    }
}
