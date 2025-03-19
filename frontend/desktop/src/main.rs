// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macro_rules_attribute::apply;
use smol_macros::main;

#[apply(main!)]
async fn main() {
    alexandria_tauri_lib::run().await
}
