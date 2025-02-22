mod commands;
use commands::*;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
            let db_path = app_dir.join("iptv.db");

            let conn = Connection::open(&db_path).expect("Failed to open database");
            commands::init_db(&conn).expect("Failed to initialize database");
            let db = DbConnection(Mutex::new(conn));

            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            initialize_database,
            add_playlist,
            get_playlists
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
