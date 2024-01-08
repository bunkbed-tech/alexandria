use std::env::var;

use serde_json::to_string_pretty;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tauri::{State, command};

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(Serialize, Deserialize, Debug)]
struct Resource {
    id: i64,
    title: String,
    description: String,
    year_published: i64,
    owned: bool,
    want_to_own: bool,
    want_to_try: bool,
}

#[command]
fn list_resources(state: State<'_, PgPoolWrapper>) -> Result<String, String> {
    let rows: Vec<Resources> = {
        sqlx::query_as!(Resource, r#"SELECT * FROM resource"#)
            .fetch_all(&state.pool)
            .await
            .expect("Unable to list resources")
    };
    to_string_pretty(&rows).map_err(|err| err.to_string())
}

struct PgPoolWrapper {
  pub pool: PgPool,
}

#[async_std::main]
fn main() {
    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set to connect to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a postgres connection pool");

    tauri::Builder::default()
        .manage(PgPoolWrapper { pool })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
