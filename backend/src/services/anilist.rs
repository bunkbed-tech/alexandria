use std::collections::HashSet;

use actix_web::{get, web::Query, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_value};

use models::Resource;

use crate::http::respond;

#[get("/search")]
pub async fn anilist_search(Query(params): Query<QueryParams>) -> impl Responder {
    respond(search_anilist(params.search, params.media_format).await)
}

// TODO collect seasons under umbrella series
// NOTE [MediaRelation](https://docs.anilist.co/reference/enum/mediarelation)

pub async fn search_anilist<Format>(
    search: String,
    media_format: Format,
) -> Result<Vec<Resource>, String>
where
    Format: Into<Vec<MediaFormat>>,
{
    let media_formats: Vec<MediaFormat> = media_format.into();
    let media_types_unique = media_formats
        .iter()
        .map(|format| match format {
            MediaFormat::TV | MediaFormat::MOVIE => MediaType::ANIME,
            MediaFormat::MANGA | MediaFormat::NOVEL => MediaType::MANGA,
        })
        .collect::<HashSet<_>>();
    let media_types = media_types_unique.iter().collect::<Vec<_>>();

    let mut variables = json!({"search": search, "formats": media_formats});
    if media_types.len() == 1 {
        variables["type"] = to_value(**media_types.first().unwrap()).unwrap();
    }
    let json = json!({"query": QUERY, "variables": variables});

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
                    year_published: Some(result.start_date.year.clone()),
                    thumbnail: Some(result.cover_image.medium.clone()),
                    api_id: result.id,
                })
                .collect::<Vec<Resource>>()
        })
}

const QUERY: &str = "
query ($search: String!, $type: MediaType, $formats: [MediaFormat!]!) {
  Page {
    media (search: $search, type: $type, format_in: $formats) {
      id
      description
      startDate {
        year
      }
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

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
enum MediaType {
    ANIME,
    MANGA,
}

#[derive(Deserialize, Serialize)]
pub enum MediaFormat {
    TV,
    MOVIE,
    MANGA,
    NOVEL,
}

impl From<MediaFormat> for Vec<MediaFormat> {
    fn from(f: MediaFormat) -> Self {
        vec![f]
    }
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
struct FuzzyDate {
    year: i32,
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
    start_date: FuzzyDate,
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
    async fn test_search_anilist_movie() {
        let search = String::from("Paprika");
        let resources = search_anilist(search, MediaFormat::MOVIE).await.unwrap();
        let expected_resources = vec![Resource {
            id: None,
            api_id: 1943,
            title: String::from("Paprika"),
            description: Some(String::from("Prepare to enter the realm of fantasy and imagination where reality and dreams collide in a kaleidoscopic mindscape of sheer visual genius. The magical tale centers on a revolutionary machine that allows scientists to enter and record a subject's dream. After being stolen, a fearless detective and brilliant therapist join forces to recover the device before it falls into the hands of a dream terrorist.<br>\n<br>\n(Source: Sony Pictures Home Entertainment)<br>\n<br>\n<i>Note: The film received an early premiere at the 63rd Venice International Film Festival on September 2, 2006.</i>")),
            year_published: Some(2006),
            thumbnail: Some(String::from("https://s4.anilist.co/file/anilistcdn/media/anime/cover/small/b1943-TBNhMVA9VwdI.png")),
        }];
        assert_eq!(resources, expected_resources);
    }

    #[test]
    async fn test_search_anilist_tv() {
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

    #[test]
    async fn test_search_anilist_novel() {
        let search = String::from("All You Need Is Kill");
        let resources = search_anilist(search, MediaFormat::NOVEL).await.unwrap();
        let expected_resources = vec![Resource {
            id: None,
            api_id: 48511,
            title: String::from("All You Need is Kill"),
            description: Some(String::from("When the alien Mimics invade, Keiji Kiriya is just one of many recruits shoved into a suit of battle armor called a Jacket and sent out to kill. Keiji dies on the battlefield, only to be reborn each morning to fight and die again and again. On his 158th iteration, he gets a message from a mysterious ally--the female soldier known as the Full Metal Bitch. Is she the key to Keiji's escape or his final death?\n<br><br>\n(Source: Viz Media)")),
            year_published: Some(2004),
            thumbnail: Some(String::from("https://s4.anilist.co/file/anilistcdn/media/manga/cover/small/bx48511-HJpsLXWtjHTz.jpg"))
        }];
        assert_eq!(resources, expected_resources);
    }

    #[test]
    async fn test_search_anilist_manga() {
        let search = String::from("All You Need Is Kill");
        let resources = search_anilist(search, MediaFormat::MANGA).await.unwrap();
        let expected_resources = vec![Resource {
            id: None,
            api_id: 85215,
            title: String::from("All You Need Is Kill"),
            description: Some(String::from("When the alien Mimics invade, Keiji Kiriya is just one of many recruits shoved into a suit of battle armor called a Jacket and sent out to kill. Keiji dies on the battlefield, only to be reborn each morning to fight and die again and again. On his 158th iteration, he gets a message from a mysterious ally--the female soldier known as the Full Metal Bitch. Is she the key to Keiji's escape or his final death?\n<br><br>\n(Source: Viz Media)")),
            year_published: Some(2014),
            thumbnail: Some(String::from("https://s4.anilist.co/file/anilistcdn/media/manga/cover/small/bx85215-oHqG7fkrpas9.png")),
        }];
        assert_eq!(resources, expected_resources);
    }

    #[test]
    async fn test_search_anilist_multiple_formats() {
        let search = String::from("All You Need Is Kill");
        let resources = search_anilist(search, vec![MediaFormat::MANGA, MediaFormat::NOVEL])
            .await
            .unwrap();
        let expected_resources = vec![
            Resource {
                id: None,
                api_id: 85215,
                title: String::from("All You Need Is Kill"),
                description: Some(String::from("When the alien Mimics invade, Keiji Kiriya is just one of many recruits shoved into a suit of battle armor called a Jacket and sent out to kill. Keiji dies on the battlefield, only to be reborn each morning to fight and die again and again. On his 158th iteration, he gets a message from a mysterious ally--the female soldier known as the Full Metal Bitch. Is she the key to Keiji's escape or his final death?\n<br><br>\n(Source: Viz Media)")),
                year_published: Some(2014),
                thumbnail: Some(String::from("https://s4.anilist.co/file/anilistcdn/media/manga/cover/small/bx85215-oHqG7fkrpas9.png")),
            },
            Resource {
              id: None,
              api_id: 48511,
              title: String::from("All You Need is Kill"),
              description: Some(String::from("When the alien Mimics invade, Keiji Kiriya is just one of many recruits shoved into a suit of battle armor called a Jacket and sent out to kill. Keiji dies on the battlefield, only to be reborn each morning to fight and die again and again. On his 158th iteration, he gets a message from a mysterious ally--the female soldier known as the Full Metal Bitch. Is she the key to Keiji's escape or his final death?\n<br><br>\n(Source: Viz Media)")),
              year_published: Some(2004),
              thumbnail: Some(String::from("https://s4.anilist.co/file/anilistcdn/media/manga/cover/small/bx48511-HJpsLXWtjHTz.jpg"))
            }
        ];
        assert_eq!(resources, expected_resources);
    }

    #[test]
    async fn test_search_anilist_multiple_types() {
        let search = String::from("Samurai Champloo");
        let resources = search_anilist(search, vec![MediaFormat::TV, MediaFormat::MANGA])
            .await
            .unwrap();
        let expected_resources = vec![
            Resource {
                id: None,
                api_id: 205,
                title: String::from("Samurai Champloo"),
                description: Some(String::from("Let's break it down. Mugen's a reckless sword-slinger with a style that's more b-boy than Shaolin. He's got a nasty streak that makes people want to stick a knife in his throat. Then there's Jin, a deadbeat ronin who speaks softly but carries a big blade. He runs game old-school style, but he can make your blood spray with the quickness. When these roughnecks bring the ruckus, it ain't good for anybody, especially them. Enter Fuu, the ditzy waitress who springs her new friends from a deadly jam. All she wants in return is help solving a riddle from her past. She and the boys are tracking the scent, but there's 99 ways to die between them and the sunflower samurai.<br>\n<br>\n(Source: Funimation)")),
                year_published: Some(2004),
                thumbnail: Some(String::from("https://s4.anilist.co/file/anilistcdn/media/anime/cover/small/bx205-xxonQKyJtVcw.png")),
            },
            Resource {
              id: None,
              api_id: 30512,
              title: String::from("Samurai Champloo"),
              description: Some(String::from("Mugen is a rough-around-the-edges mercenary with a killer technique and nothing left to lose. Jin is a disciplined samurai who's as deadly as he is reserved. Fuu is a young waitress with a good heart and a resourcefulness that emerges when you least expect it. These three unlikely companions are about to begin a journey that will change all of their lives.<br><br>\nIt's a dangerous quest for a mysterious samurai that will see our squabbling group of heroes get into and out of trouble more times than they can count (which admittedly, isn't very high). From the cynical gentility of the nobles to the backstabbing of the Japanese underworld, Mugen, Jin and Fuu will face threats from without and within as they hurl insults and throwing stars alike. Ancient Japan is about to get a lethal dose of street justice -- Champloo style. And it will never be the same.<br><br>\n(Source: Tokyopop)")),
              year_published: Some(2004),
              thumbnail: Some(String::from("https://s4.anilist.co/file/anilistcdn/media/manga/cover/small/bx30512-L7FWQ9Dj6dHj.png")),
            }
        ];
        assert_eq!(resources, expected_resources);
    }
}
