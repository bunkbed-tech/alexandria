use actix_web::{get, web::Query, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use models::AlexandriaResource;

use crate::http::respond;

#[get("/search")]
pub async fn vndb_search(Query(params): Query<QueryParams>) -> impl Responder {
    respond(search_vndb(params.query).await)
}

pub async fn search_vndb(query: String) -> Result<Vec<AlexandriaResource>, String> {
    Client::new()
        .post("https://api.vndb.org/kana/vn")
        .json(&RequestData {
            filters: vec![String::from("search"), String::from("="), query],
            fields: String::from("title, description, released, image.thumbnail"),
            count: true,
            results: 100,
        })
        .send()
        .await
        .map_err(|err| String::from("[search_vndb:_:post] ") + &err.to_string())?
        .json::<SearchResults>()
        .await
        .map_err(|err| String::from("[search_vndb:_:json] ") + &err.to_string())
        .map(|results| {
            results
                .results
                .iter()
                .map(|result| AlexandriaResource {
                    id: None,
                    title: result.title.clone(),
                    description: result.description.clone(),
                    year_published: result
                        .released
                        .clone()
                        .and_then(|r| r.get(0..4).and_then(|year| year.parse().ok())),
                    thumbnail: result.image.thumbnail.clone(),
                    api_id: result
                        .id
                        .get(1..)
                        .and_then(|id| id.parse().ok())
                        .expect("What the hell VNDB"),
                })
                .collect::<Vec<AlexandriaResource>>()
        })
}

#[derive(Deserialize)]
struct QueryParams {
    query: String,
}

#[derive(Deserialize)]
struct SearchImage {
    thumbnail: Option<String>,
}

#[derive(Deserialize)]
struct SearchResult {
    description: Option<String>,
    image: SearchImage,
    released: Option<String>,
    id: String,
    title: String,
}

#[derive(Deserialize)]
struct SearchResults {
    results: Vec<SearchResult>,
    more: bool,
    count: i32,
}

#[derive(Serialize)]
struct RequestData {
    filters: Vec<String>,
    fields: String,
    count: bool,
    results: i32,
}

#[cfg(test)]
mod tests {
    use actix_web::test;

    use super::*;

    #[test]
    async fn test_search_vndb() {
        let query = String::from("steins divergence");
        let resources = search_vndb(query).await.unwrap();
        let expected_resources = vec![AlexandriaResource {
            id: None,
            api_id: 15695,
            title: String::from("Steins;Gate Divergence"),
            description: None,
            year_published: Some(2014),
            thumbnail: Some(String::from("https://t.vndb.org/cv/83/21383.jpg")),
        }];
        assert_eq!(resources, expected_resources);
    }
}
