// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[async_std::main]
async fn main() {
    alexandria_tauri_lib::run().await
}
