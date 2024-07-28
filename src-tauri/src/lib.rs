use std::collections::HashSet;
use std::env::var;

use quick_xml::de::from_str;
use reqwest::get;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tauri::{command, State};

mod models;
use crate::models::Resource;

#[command]
async fn search_bgg(query: String) -> Result<Vec<i32>, String> {
    let search_xml = get(format!(
        "https://boardgamegeek.com/xmlapi2/search?query={}",
        query
    ))
    .await
    .map_err(|err| String::from("[search_bgg:search_xml:get] ") + &err.to_string())?
    .text()
    .await
    .map_err(|err| String::from("[search_bgg:search_xml:text] ") + &err.to_string())?;
    let search_items: SearchItems = from_str(&search_xml).map_err(|err| String::from("[search_bgg:search_items:from_str] ") + &err.to_string())?;
    let ids = search_items
        .item
        .clone()
        .into_iter()
        .map(|item| item.id.parse::<i32>().expect("Not a valid ID"))
        .collect::<Vec<i32>>();
    Ok(ids)
}

#[command]
async fn list_bgg_things(ids: Vec<i32>) -> Result<Vec<Resource>, String> {
    let thing_xml = get(format!(
        "https://boardgamegeek.com/xmlapi2/thing?id={}",
        ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",")
    ))
    .await
    .map_err(|err| String::from("[list_bgg_things:thing_xml:get] ") + &err.to_string())?
    .text()
    .await
    .map_err(|err| String::from("[list_bgg_things:thing_xml:text] ") + &err.to_string())?;
    let thing_items: ThingItems = from_str(&thing_xml).map_err(|err| String::from("[list_bgg_things:thing_items:from_str] ") + &err.to_string() + &thing_xml)?;
    let resources = thing_items
        .item
        .into_iter()
        .zip(ids)
        .map(|(thing, id)| Resource {
            id: None,
            title: thing
                .name
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
    Ok(resources)
}

#[command]
async fn list_resources(
    state: State<'_, PgPoolWrapper>,
    ids: Option<Vec<i32>>,
) -> Result<Vec<Resource>, String> {
    let rows: Vec<Resource>;
    if let Some(bgg_ids) = ids {
        rows = sqlx::query_as!(Resource, r#"SELECT * FROM resource WHERE bgg_id = ANY($1)"#, &bgg_ids)
            .fetch_all(&state.pool)
            .await
            .expect("Unable to list resources")
    } else {
        rows = sqlx::query_as!(Resource, r#"SELECT * FROM resource"#)
            .fetch_all(&state.pool)
            .await
            .expect("Unable to list resources")
    }
    Ok(rows)
}

#[command]
async fn untrack_resource(
    mut resource: Resource,
    state: State<'_, PgPoolWrapper>,
) -> Result<Resource, String> {
    sqlx::query_as!(
        Resource,
        r#"DELETE FROM resource WHERE id = $1"#,
        resource.id,
    )
    .execute(&state.pool)
    .await
    .map_err(|err| String::from("[untrack_resource:_:execute] ") + &err.to_string())?;
    resource.id = None;
    Ok(resource)
}

#[command]
async fn track_resource(
    resource: Resource,
    state: State<'_, PgPoolWrapper>,
) -> Result<Resource, String> {
    let db_resource = {
        sqlx::query_as!(
            Resource,
            r#"INSERT INTO resource (title, description, year_published, thumbnail, bgg_id) VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
            resource.title,
            resource.description,
            resource.year_published,
            resource.thumbnail,
            resource.bgg_id,
        ).fetch_one(&state.pool)
        .await
        .map_err(|err| String::from("[track_resource:db_resource:fetch_one] ") + &err.to_string())?
    };
    Ok(db_resource)
}

struct PgPoolWrapper {
    pub pool: PgPool,
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

    tauri::Builder::default()
        .manage(PgPoolWrapper { pool })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            list_resources,
            search_bgg,
            list_bgg_things,
            track_resource,
            untrack_resource,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
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
    item: Vec<SearchItem>,
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
    name: Vec<NameAttribute>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct ThingItems {
    item: Vec<ThingItem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_xml() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?><items total="75" termsofuse="https://boardgamegeek.com/xmlapi/termsofuse"> <item type="boardgame" id="398158"> <name type="primary" value="Grind House: Scythes Out"/> <yearpublished value="2023" /> </item> <item type="boardgame" id="226320"> <name type="primary" value="My Little Scythe"/> <yearpublished value="2017" /> </item></items>"#;
        let items: SearchItems = from_str(&xml).unwrap();
        let iitems = SearchItems {
            item: vec![
                SearchItem {
                    id: String::from("398158"),
                    name: Attribute {
                        value: String::from("Grind House: Scythes Out"),
                    },
                    yearpublished: Some(Attribute {
                        value: String::from("2023"),
                    }),
                },
                SearchItem {
                    id: String::from("226320"),
                    name: Attribute {
                        value: String::from("My Little Scythe"),
                    },
                    yearpublished: Some(Attribute {
                        value: String::from("2017"),
                    }),
                },
            ],
        };
        assert_eq!(items, iitems);
    }

    #[test]
    fn test_thing_xml() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?><items termsofuse="https://boardgamegeek.com/xmlapi/termsofuse"><item type="boardgame" id="225694"><thumbnail>https://cf.geekdo-images.com/hHZWXnUTMYDd_KTAM6Jwlw__thumb/img/O5XHaPOALYquS058qcXWVm5b_k4=/fit-in/200x150/filters:strip_icc()/pic3759421.jpg</thumbnail></item></items>"#;
        let items: ThingItems = from_str(&xml).unwrap();
        let iitems = ThingItems {
            item: vec![
                ThingItem {
                    thumbnail: Some(Thumbnail {
                        value: String::from("https://cf.geekdo-images.com/hHZWXnUTMYDd_KTAM6Jwlw__thumb/img/O5XHaPOALYquS058qcXWVm5b_k4=/fit-in/200x150/filters:strip_icc()/pic3759421.jpg"),
                    }),
                },
            ],
        };
        assert_eq!(items, iitems);
    }

    #[async_std::test]
    async fn test_search_bgg() {
        let query = String::from("Cranium Cadoo");
        let resources = search_bgg(query).await.unwrap();
        let rresources = vec![
            Resource {
                id: None,
                bgg_id: 6420,
                title: String::from("Cranium Cadoo"),
                description: String::from(""),
                year_published: Some(2001),
                thumbnail: Some(String::from("https://cf.geekdo-images.com/hQI6W-7HwKty4c5yLFP-Aw__thumb/img/_IyE4nIyGh7_PVfGCarLoNmDMGc=/fit-in/200x150/filters:strip_icc()/pic3335930.jpg")),
            },
            Resource {
                id: None,
                bgg_id: 14454,
                title: String::from("Cranium Cadoo Booster Box"),
                description: String::from(""),
                year_published: Some(2001),
                thumbnail: Some(String::from("https://cf.geekdo-images.com/jboSqbHm5jcQp7XJZPM-vw__thumb/img/v6dQ2IqIdGJIX19AVEZDSaQ5Nms=/fit-in/200x150/filters:strip_icc()/pic58689.jpg")),
            },
        ];
        assert_eq!(resources, rresources);
    }
}
