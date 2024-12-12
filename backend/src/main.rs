use std::env::var;

use actix_web::{web::Data, App, HttpServer};
use sqlx::{postgres::PgPoolOptions};

use alexandria::{
    state::AppState,
    services::{
        bgg::bgg_resource,
        resource::resource_resource,
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set to connect to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(bgg_resource)
            .service(resource_resource)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
