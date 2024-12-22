async fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) {
    builder
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    create_app(tauri::Builder::default()).await;
}
