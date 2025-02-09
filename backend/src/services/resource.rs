use std::str::FromStr;

use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, Query},
    Responder,
};
use serde::{Deserialize, Deserializer};
use sqlx::postgres::PgPool;

use models::Resource;

use crate::{http::respond, state::AppState};

#[get("/")]
pub async fn resource_list(
    data: Data<AppState>,
    Query(params): Query<IdsParams>,
) -> impl Responder {
    respond(list_resources(&data.db, params.ids).await)
}

#[post("/")]
pub async fn resource_track(
    data: Data<AppState>,
    Json(json): Json<ResourceData>,
) -> impl Responder {
    respond(track_resource(&data.db, json.resource).await)
}

#[delete("/{id}")]
pub async fn resource_untrack(data: Data<AppState>, path: Path<i32>) -> impl Responder {
    respond(untrack_resource(&data.db, path.into_inner()).await)
}

async fn list_resources(pool: &PgPool, ids: Option<Vec<i32>>) -> Result<Vec<Resource>, String> {
    let rows: Vec<Resource>;
    if let Some(api_ids) = ids {
        rows = sqlx::query_as!(
            Resource,
            r#"SELECT * FROM resource WHERE api_id = ANY($1)"#,
            &api_ids
        )
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

async fn track_resource(pool: &PgPool, resource: Resource) -> Result<Resource, String> {
    if let Some(_) = resource.id {
        return Err(format!("Resource {} is already tracked.", resource));
    }
    let db_resource = {
        sqlx::query_as!(
            Resource,
            r#"INSERT INTO resource (title, description, year_published, thumbnail, api_id) VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
            resource.title,
            resource.description,
            resource.year_published,
            resource.thumbnail,
            resource.api_id,
        ).fetch_one(pool)
        .await
        .map_err(|err| String::from("[track_resource:db_resource:fetch_one] ") + &err.to_string())?
    };
    Ok(db_resource)
}

async fn untrack_resource(pool: &PgPool, id: i32) -> Result<Resource, String> {
    let mut resource = {
        sqlx::query_as!(
            Resource,
            r#"DELETE FROM resource WHERE id = $1 RETURNING *"#,
            id,
        )
        .fetch_one(pool)
        .await
        .map_err(|err| String::from("[untrack_resource:_:execute] ") + &err.to_string())?
    };
    resource.id = None;
    Ok(resource)
}

fn csv_ids<'de, D>(deserializer: D) -> Result<Option<Vec<i32>>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<&str>::deserialize(deserializer)? {
        Some(string) => {
            let result: Result<Vec<i32>, _> =
                string.split(",").map(|v| i32::from_str(v.trim())).collect();
            result.map(Some).map_err(serde::de::Error::custom)
        }
        None => Ok(None),
    }
}

#[derive(Deserialize)]
struct ResourceData {
    resource: Resource,
}

#[derive(Deserialize)]
struct IdsParams {
    #[serde(deserialize_with = "csv_ids")]
    ids: Option<Vec<i32>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::test;

    #[test(fixtures("resources"))]
    async fn test_list_resources_all(pool: PgPool) {
        let ids: Option<Vec<i32>> = None;
        let resources = list_resources(&pool, ids).await.unwrap();
        let expected_resources = vec![
            Resource {
                id: Some(1),
                title: String::from("Scythe"),
                description: Some(String::from("Really good game")),
                year_published: Some(2015),
                thumbnail: Some(String::from("https://google.com")),
                api_id: 9000,
            },
            Resource {
                id: Some(2),
                title: String::from("Cranium Cadoo"),
                description: None,
                year_published: None,
                thumbnail: None,
                api_id: 420,
            },
        ];
        assert_eq!(resources, expected_resources);
    }

    #[test]
    async fn test_list_resources_missing(pool: PgPool) {
        let ids = Some(vec![1]);
        let resources = list_resources(&pool, ids).await.unwrap();
        let expected_resources = Vec::<Resource>::new();
        assert_eq!(resources, expected_resources);
    }

    #[test(fixtures("resources"))]
    async fn test_list_resources_some(pool: PgPool) {
        let ids = Some(vec![9000]);
        let resources = list_resources(&pool, ids).await.unwrap();
        let expected_resources = vec![Resource {
            id: Some(1),
            title: String::from("Scythe"),
            description: Some(String::from("Really good game")),
            year_published: Some(2015),
            thumbnail: Some(String::from("https://google.com")),
            api_id: 9000,
        }];
        assert_eq!(resources, expected_resources);
    }

    #[test]
    async fn test_track_resource_untracked(pool: PgPool) {
        let untracked_resource = Resource {
            id: None,
            title: String::from("Scythe"),
            description: Some(String::from("Really good game")),
            year_published: Some(2015),
            thumbnail: Some(String::from("https://google.com")),
            api_id: 9000,
        };
        let tracked_resource = track_resource(&pool, untracked_resource).await.unwrap();
        let expected_tracked_resource = Resource {
            id: Some(1),
            title: String::from("Scythe"),
            description: Some(String::from("Really good game")),
            year_published: Some(2015),
            thumbnail: Some(String::from("https://google.com")),
            api_id: 9000,
        };
        assert_eq!(tracked_resource, expected_tracked_resource);
    }

    #[test]
    async fn test_track_resource_tracked(pool: PgPool) {
        let tracked_resource = Resource {
            id: Some(1),
            title: String::from("Scythe"),
            description: Some(String::from("Really good game")),
            year_published: Some(2015),
            thumbnail: Some(String::from("https://google.com")),
            api_id: 9000,
        };
        assert!(track_resource(&pool, tracked_resource).await.is_err());
    }

    #[test(fixtures("resources"))]
    async fn test_track_resource_invalid(pool: PgPool) {
        // This is invalid because the resource is already in the database, but id is None
        // This should probably only happen if we make a mistake in developing the app
        let untracked_resource = Resource {
            id: None,
            title: String::from("Scythe"),
            description: Some(String::from("Really good game")),
            year_published: Some(2015),
            thumbnail: Some(String::from("https://google.com")),
            api_id: 9000,
        };
        assert!(track_resource(&pool, untracked_resource).await.is_err());
    }

    #[test(fixtures("resources"))]
    async fn test_untrack_resource(pool: PgPool) {
        let tracked_resource = Resource {
            id: Some(1),
            title: String::from("Scythe"),
            description: Some(String::from("Really good game")),
            year_published: Some(2015),
            thumbnail: Some(String::from("https://google.com")),
            api_id: 9000,
        };
        let untracked_resource = untrack_resource(&pool, tracked_resource.id.unwrap())
            .await
            .unwrap();
        let expected_untracked_resource = Resource {
            id: None,
            title: String::from("Scythe"),
            description: Some(String::from("Really good game")),
            year_published: Some(2015),
            thumbnail: Some(String::from("https://google.com")),
            api_id: 9000,
        };
        assert_eq!(untracked_resource, expected_untracked_resource);
    }
}
