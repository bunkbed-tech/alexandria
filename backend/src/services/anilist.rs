use actix_web::{get, web::Query, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{http::respond, models::Resource};

#[get("/search")]
pub async fn anilist_search(Query(params): Query<QueryParams>) -> impl Responder {
    respond(search_anilist(params.search, params.media_format).await)
}

async fn search_anilist(
    search: String,
    media_format: MediaFormat,
) -> Result<Vec<Resource>, String> {
    let media_type = match media_format {
        MediaFormat::TV | MediaFormat::MOVIE => MediaType::ANIME,
        MediaFormat::MANGA | MediaFormat::NOVEL => MediaType::MANGA,
    };
    let json = json!({"query": QUERY, "variables": {"search": search, "type": media_type, "format": media_format}});
    Client::new()
        .post("https://graphql.anilist.co")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send()
        .await
        .map_err(|err| String::from("[search_anilist:_:post] ") + &err.to_string())?
        .json::<SearchResults>()
        .await
        .map_err(|err| String::from("[search_anilist:_:json] ") + &err.to_string())
        .map(|results| {
            results
                .data
                .page
                .media
                .iter()
                .map(|result| Resource {
                    id: None,
                    title: result.title.english.clone(),
                    description: Some(result.description.clone()),
                    year_published: Some(result.season_year.clone()),
                    thumbnail: Some(result.cover_image.medium.clone()),
                    api_id: result.id,
                })
                .collect::<Vec<Resource>>()
        })
}

const QUERY: &str = "
query ($search: String!, $type: MediaType!, $format: MediaFormat!) {
  Page {
    media (search: $search, type: $type, format: $format) {
      id
      description
      seasonYear
      title {
        english
      }
      coverImage {
        medium
      }
    }
  }
}
";

#[derive(Deserialize, Serialize)]
enum MediaType {
    ANIME,
    MANGA,
}

#[derive(Deserialize, Serialize)]
enum MediaFormat {
    TV,
    MOVIE,
    MANGA,
    NOVEL,
}

#[derive(Deserialize)]
struct QueryParams {
    search: String,
    media_format: MediaFormat,
}

#[derive(Deserialize)]
struct MediaTitle {
    english: String,
}

#[derive(Deserialize)]
struct CoverImage {
    medium: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Media {
    id: i32,
    description: String,
    season_year: i32,
    title: MediaTitle,
    cover_image: CoverImage,
}

#[derive(Deserialize)]
struct Page {
    media: Vec<Media>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Data {
    page: Page,
}

#[derive(Deserialize)]
struct SearchResults {
    data: Data,
}

#[cfg(test)]
mod tests {
    use actix_web::test;

    use super::*;

    #[test]
    async fn test_search_anilist() {
        let search = String::from("Buddy Daddies");
        let resources = search_anilist(search, MediaFormat::TV).await.unwrap();
        let expected_resources = vec![Resource {
            id: None,
            api_id: 155907,
            title: String::from("Buddy Daddies"),
            description: Some(String::from("Assassins Kazuki Kurusu and Rei Suwa meet Miri, a girl looking for her father on Christmas Day. Kazuki, Rei, and Miri unexpectedly end up living together.<br>\n<br>\nFollows Kazuki Kurusu, a criminal contractor/coordinator who lives with his best friend, Rei Suwa, a professional assassin who has been raised from childhood to be a contract killer. Kazuki is outgoing and loves gambling and women, while Rei is a man of few words who spends his off time playing video games. One day, the two buddies end up caring for Miri Unasaka, a four year old girl whose father is a mafia boss, after Miri accidentally wanders into a firefight in a hotel while looking for her father.")),
            year_published: Some(2023),
            thumbnail: Some(String::from("https://s4.anilist.co/file/anilistcdn/media/anime/cover/small/bx155907-gR7aRwVHwrjc.jpg")),
        }];
        assert_eq!(resources, expected_resources);
    }
}
