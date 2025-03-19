use std::collections::HashMap;

use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse, Responder,
};
use serde::Deserialize;
use sqlx::postgres::PgPool;

use models::{Resource, SearchResults};

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
    HttpResponse::Ok().json(SearchResults { resources, errors })
}

// TODO avoid cloning the query for every call
// TODO implement smarter fuzzy search to mix results from different APIs
async fn _search(pool: &PgPool, query: String) -> (Vec<Resource>, Vec<String>) {
    let ((mut bgg_resources, mut bgg_errors), anilist_result, vndb_result, igdb_result) = tokio::join!(
        search_bgg_things(query.clone()),
        search_anilist(
            query.clone(),
            vec![
                MediaFormat::TV,
                MediaFormat::MOVIE,
                MediaFormat::MANGA,
                MediaFormat::NOVEL,
            ]
        ),
        search_vndb(query.clone()),
        search_igdb(query.clone()),
    );
    let mut combined = SearchResults::default();
    combined.resources.append(&mut bgg_resources);
    combined.errors.append(&mut bgg_errors);
    for result in [anilist_result, vndb_result, igdb_result] {
        match result {
            Ok(mut _resources) => combined.resources.append(&mut _resources),
            Err(err) => combined.errors.push(err.to_string()),
        }
    }

    let ids = combined
        .resources
        .iter()
        .map(|resource| resource.api_id)
        .collect::<Vec<_>>();
    let db_resources = match list_resources(pool, Some(ids)).await {
        Ok(_resources) => _resources,
        Err(db_error) => {
            combined.errors.push(db_error);
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
    let matched_resources: Vec<Resource> = combined
        .resources
        .into_iter()
        .map(|mut resource| {
            resource.id = api_to_db_id.get(&resource.api_id).copied();
            resource
        })
        .collect();

    (matched_resources, combined.errors)
}

#[derive(Deserialize)]
struct QueryParams {
    query: String,
}
