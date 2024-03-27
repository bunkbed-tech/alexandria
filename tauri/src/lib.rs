use std::collections::HashSet;
use std::env::var;

use quick_xml::de::from_str;
use reqwest::get;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tauri::{command, State};

use models::Resource;

#[command]
async fn search_bgg(query: String) -> Result<Vec<Resource>, String> {
    let search_xml = get(format!(
        "https://boardgamegeek.com/xmlapi2/search?query={}",
        query
    ))
    .await
    .map_err(|err| err.to_string())?
    .text()
    .await
    .map_err(|err| err.to_string())?;
    let search_items: SearchItems = from_str(&search_xml).map_err(|err| err.to_string())?;
    let ids = search_items
        .item
        .clone()
        .into_iter()
        .map(|item| item.id)
        .collect::<Vec<String>>();
    let thing_xml = get(format!(
        "https://boardgamegeek.com/xmlapi2/thing?id={}",
        ids.join(",")
    ))
    .await
    .map_err(|err| err.to_string())?
    .text()
    .await
    .map_err(|err| err.to_string())?;
    let thing_items: ThingItems = from_str(&thing_xml).map_err(|err| err.to_string())?;
    let resources = search_items
        .item
        .into_iter()
        .zip(thing_items.item.into_iter())
        .map(|(search, thing)| Resource {
            id: search.id.parse::<i64>().expect("Not a valid ID"),
            title: search.name.value,
            description: "".to_string(),
            year_published: search
                .yearpublished
                .map(|year| year.value.parse::<i32>().expect("Not a valid year")),
            owned: false,
            want_to_own: false,
            want_to_try: false,
            thumbnail: thing.thumbnail.value,
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    Ok(resources)
}

#[command]
async fn list_resources(state: State<'_, PgPoolWrapper>) -> Result<String, String> {
    let rows: Vec<Resource> = {
        sqlx::query_as!(Resource, r#"SELECT * FROM resource"#)
            .fetch_all(&state.pool)
            .await
            .expect("Unable to list resources")
    };
    to_string_pretty(&rows).map_err(|err| err.to_string())
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
        .invoke_handler(tauri::generate_handler![list_resources, search_bgg,])
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
    name: Attribute,
    yearpublished: Option<Attribute>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct SearchItems {
    item: Vec<SearchItem>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Thumbnail {
    #[serde(rename = "$text")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct ThingItem {
    thumbnail: Thumbnail,
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
                    thumbnail: Thumbnail {
                        value: String::from("https://cf.geekdo-images.com/hHZWXnUTMYDd_KTAM6Jwlw__thumb/img/O5XHaPOALYquS058qcXWVm5b_k4=/fit-in/200x150/filters:strip_icc()/pic3759421.jpg"),
                    },
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
                id: 6420,
                title: String::from("Cranium Cadoo"),
                description: String::from(""),
                year_published: Some(2001),
                owned: false,
                want_to_own: false,
                want_to_try: false,
                thumbnail: String::from("https://cf.geekdo-images.com/hQI6W-7HwKty4c5yLFP-Aw__thumb/img/_IyE4nIyGh7_PVfGCarLoNmDMGc=/fit-in/200x150/filters:strip_icc()/pic3335930.jpg"),
            },
            Resource {
                id: 14454,
                title: String::from("Cranium Cadoo Booster Box"),
                description: String::from(""),
                year_published: Some(2001),
                owned: false,
                want_to_own: false,
                want_to_try: false,
                thumbnail: String::from("https://cf.geekdo-images.com/jboSqbHm5jcQp7XJZPM-vw__thumb/img/v6dQ2IqIdGJIX19AVEZDSaQ5Nms=/fit-in/200x150/filters:strip_icc()/pic58689.jpg"),
            },
        ];
        assert_eq!(resources, rresources);
    }
}
