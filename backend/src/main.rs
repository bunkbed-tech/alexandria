use std::env::var;

use actix_web::{
    web::{scope, Data},
    App, HttpServer,
};
use sqlx::postgres::PgPoolOptions;

use alexandria::{
    services::{bgg, resource},
    state::AppState,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url =
        var("DATABASE_URL").expect("DATABASE_URL must be set to connect to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(
                scope("/bgg")
                    .service(bgg::bgg_search)
                    .service(bgg::bgg_things_list)
                    .service(bgg::bgg_things_search),
            )
            .service(
                scope("/resource")
                    .service(resource::resource_list)
                    .service(resource::resource_track)
                    .service(resource::resource_untrack),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
