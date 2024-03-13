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
    let xml = get(format!(
        "https://boardgamegeek.com/xmlapi2/search?query={}",
        query
    ))
    .await
    .map_err(|err| err.to_string())?
    .text()
    .await
    .map_err(|err| err.to_string())?;
    let items: Items = from_str(&xml).map_err(|err| err.to_string())?;
    let resources: Vec<Resource> = items.into();
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
struct Item {
    name: Attribute,
    yearpublished: Option<Attribute>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Items {
    item: Vec<Item>,
}

impl Into<Resource> for Item {
    fn into(self) -> Resource {
        Resource {
            id: 0,
            title: self.name.value,
            description: "".to_string(),
            year_published: self
                .yearpublished
                .map(|year| year.value.parse::<i32>().expect("Not a valid year")),
            owned: false,
            want_to_own: false,
            want_to_try: false,
        }
    }
}

impl Into<Vec<Resource>> for Items {
    fn into(self) -> Vec<Resource> {
        self.item.into_iter().map(Into::into).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?><items total="75" termsofuse="https://boardgamegeek.com/xmlapi/termsofuse"> <item type="boardgame" id="398158"> <name type="primary" value="Grind House: Scythes Out"/> <yearpublished value="2023" /> </item> <item type="boardgame" id="226320"> <name type="primary" value="My Little Scythe"/> <yearpublished value="2017" /> </item></items>"#;
        let items: Items = from_str(&xml).unwrap();
        let iitems = Items {
            item: vec![
                Item {
                    name: Attribute {
                        value: String::from("Grind House: Scythes Out"),
                    },
                    yearpublished: Some(Attribute {
                        value: String::from("2023"),
                    }),
                },
                Item {
                    name: Attribute {
                        value: String::from("My Little Scythe"),
                    },
                    yearpublished: Some(Attribute {
                        value: String::from("2017"),
                    }),
                },
            ],
        };
        let resources: Vec<Resource> = items.clone().into();
        let rresources = vec![
            Resource {
                id: 0,
                title: String::from("Grind House: Scythes Out"),
                description: "".to_string(),
                year_published: Some(2023),
                owned: false,
                want_to_own: false,
                want_to_try: false,
            },
            Resource {
                id: 0,
                title: String::from("My Little Scythe"),
                description: "".to_string(),
                year_published: Some(2017),
                owned: false,
                want_to_own: false,
                want_to_try: false,
            },
        ];
        assert_eq!(items, iitems);
        assert_eq!(resources, rresources);
    }
}
