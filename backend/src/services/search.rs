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
    services::{
        anilist::{search_anilist, MediaFormat},
        bgg::search_bgg_things,
        igdb::search_igdb,
        resource::list_resources,
        vndb::search_vndb,
    },
    state::AppState,
};

#[get("")]
pub async fn search(data: Data<AppState>, Query(params): Query<QueryParams>) -> impl Responder {
    let (resources, errors) = _search(&data.db, params.query).await;
    HttpResponse::Ok().json(Results { resources, errors })
}

// TODO parallelize all of these external calls
// TODO avoid cloning the query for every call
// TODO implement smarter fuzzy search to mix results from different APIs
async fn _search(pool: &PgPool, query: String) -> (Vec<Resource>, Vec<String>) {
    let mut resources: Vec<Resource> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    let (mut bgg_resources, mut bgg_errors) = search_bgg_things(query.clone()).await;
    resources.append(&mut bgg_resources);
    errors.append(&mut bgg_errors);

    let anilist_formats = vec![
        MediaFormat::TV,
        MediaFormat::MOVIE,
        MediaFormat::MANGA,
        MediaFormat::NOVEL,
    ];
    match search_anilist(query.clone(), anilist_formats).await {
        Ok(mut anilist_resources) => resources.append(&mut anilist_resources),
        Err(anilist_error) => errors.push(anilist_error),
    };

    match search_vndb(query.clone()).await {
        Ok(mut vndb_resources) => resources.append(&mut vndb_resources),
        Err(vndb_error) => errors.push(vndb_error),
    };

    match search_igdb(query.clone()).await {
        Ok(mut igdb_resources) => resources.append(&mut igdb_resources),
        Err(igdb_error) => errors.push(igdb_error),
    };

    let ids = resources
        .iter()
        .map(|resource| resource.api_id)
        .collect::<Vec<_>>();
    let db_resources = match list_resources(pool, Some(ids)).await {
        Ok(_resources) => _resources,
        Err(db_error) => {
            errors.push(db_error);
            Vec::<Resource>::new()
        }
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
