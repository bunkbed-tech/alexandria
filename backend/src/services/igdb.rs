use std::{collections::HashMap, env::var};

use actix_web::{get, web::Query, Responder};
use chrono::{Datelike, TimeZone, Utc};
use reqwest::Client;
use serde::Deserialize;

use crate::{http::respond, models::Resource};

#[get("/search")]
pub async fn igdb_search(Query(params): Query<QueryParams>) -> impl Responder {
    respond(search_igdb(params.query).await)
}

async fn search_igdb(query: String) -> Result<Vec<Resource>, String> {
    let client_id =
        var("ALEXANDRIA_IGDB_CLIENT_ID").expect("ALEXANDRIA_IGDB_CLIENT_ID must be set");
    let client_id_str = client_id.as_str();
    let client = Client::new();
    let access_token = client
        .post("https://id.twitch.tv/oauth2/token")
        .query(&[
            ("client_id", client_id_str),
            (
                "client_secret",
                var("ALEXANDRIA_IGDB_CLIENT_SECRET")
                    .expect("ALEXANDRIA_IGDB_CLIENT_SECRET must be set")
                    .as_str(),
            ),
            ("grant_type", "client_credentials"),
        ])
        .send()
        .await
        .map_err(|err| String::from("[search_igdb:auth:post] ") + &err.to_string())?
        .json::<AuthResult>()
        .await
        .map_err(|err| String::from("[search_igdb:auth:json] ") + &err.to_string())?
        .access_token;
    let games = client
        .post("https://api.igdb.com/v4/games")
        .header("Client-ID", client_id_str)
        .header("Authorization", format!("Bearer {}", access_token))
        .body(format!(
            "fields name, summary, cover, first_release_date; search \"{}\"; limit 500;",
            query
        ))
        .send()
        .await
        .map_err(|err| String::from("[search_igdb:games:post] ") + &err.to_string())?
        .json::<Vec<SearchResult>>()
        .await
        .map_err(|err| String::from("[search_igdb:games:json] ") + &err.to_string())?;
    let covers = client
        .post("https://api.igdb.com/v4/covers")
        .header("Client-ID", client_id_str)
        .header("Authorization", format!("Bearer {}", access_token))
        .body(format!(
            "fields url; where id = ({}); limit 500;",
            games
                .iter()
                .map(|result| result.cover)
                .filter_map(|id| id)
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .join(","),
        ))
        .send()
        .await
        .map_err(|err| String::from("[search_igdb:covers:post] ") + &err.to_string())?
        .json::<Vec<CoverResult>>()
        .await
        .map_err(|err| String::from("[search_igdb:covers:json] ") + &err.to_string())?
        .into_iter()
        .map(|result| (result.id, result.url))
        .collect::<HashMap<i32, String>>();
    Ok(games
        .into_iter()
        .map(|game| Resource {
            id: None,
            title: game.name,
            description: game.summary,
            year_published: game
                .first_release_date
                .map(|date| Utc.timestamp_opt(date, 0).unwrap().year()),
            thumbnail: game
                .cover
                .as_ref()
                .map(|id| format!("https:{}", covers[id].clone())),
            api_id: game.id,
        })
        .collect())
}

#[derive(Deserialize)]
struct QueryParams {
    query: String,
}

#[derive(Deserialize)]
struct AuthResult {
    access_token: String,
}

#[derive(Deserialize)]
struct SearchResult {
    id: i32,
    name: String,
    summary: Option<String>,
    cover: Option<i32>,
    first_release_date: Option<i64>,
}

#[derive(Deserialize)]
struct CoverResult {
    id: i32,
    url: String,
}

#[cfg(test)]
mod tests {
    use actix_web::test;

    use super::*;

    #[test]
    async fn test_search_igdb() {
        let query = String::from("fretless");
        let resources = search_igdb(query).await.unwrap();
        let expected_resources = vec![Resource {
            id: None,
            api_id: 252794,
            title: String::from("Fretless"),
            year_published: Some(2025),
            description: Some(String::from("In this turned-based RPG, wield powerful legendary instruments, gather mighty riff attacks and save the land from Rick Riffson\u{0027}s devilish goons and musical hybrid monsters!")),
            thumbnail: Some(String::from("https://images.igdb.com/igdb/image/upload/t_thumb/co6lw2.jpg")),
        }];
        assert_eq!(resources, expected_resources);
    }
}
