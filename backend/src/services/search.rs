use std::collections::HashMap;

use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

use models::Resource;

use crate::{
    services::{bgg::search_bgg_things, resource::list_resources},
    state::AppState,
};

#[get("")]
pub async fn search(data: Data<AppState>, Query(params): Query<QueryParams>) -> impl Responder {
    let (resources, errors) = _search(&data.db, params.query).await;
    HttpResponse::Ok().json(Results { resources, errors })
}

async fn _search(pool: &PgPool, query: String) -> (Vec<Resource>, Vec<String>) {
    let mut resources: Vec<Resource> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    let (mut bgg_resources, mut bgg_errors) = search_bgg_things(query).await;
    resources.append(&mut bgg_resources);
    errors.append(&mut bgg_errors);

    let ids = resources
        .iter()
        .map(|resource| resource.api_id)
        .collect::<Vec<_>>();
    let db_resources = match list_resources(pool, Some(ids)).await {
        Ok(_resources) => _resources,
        Err(db_error) => {
            errors.push(db_error);
            Vec::<Resource>::new()
        },
    };

    let api_to_db_id: HashMap<i32, i32> = db_resources
        .into_iter()
        .map(|r| {
            (
                r.api_id,
                r.id.expect("Database resources always have IDs..."),
            )
        })
        .collect();
    let matched_resources: Vec<Resource> = resources
        .into_iter()
        .map(|mut resource| {
            resource.id = api_to_db_id.get(&resource.api_id).copied();
            resource
        })
        .collect();

    (matched_resources, errors)
}

#[derive(Deserialize)]
struct QueryParams {
    query: String,
}

#[derive(Serialize)]
struct Results {
    resources: Vec<Resource>,
    errors: Vec<String>,
}
