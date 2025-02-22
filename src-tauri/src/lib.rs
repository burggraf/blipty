mod commands;
use commands::*;
use log::error;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Error)
        .init()
        .unwrap();
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            if let Err(e) = std::fs::create_dir_all(&app_dir) {
                error!("Failed to create app data dir: {}", e);
                return Err(e.into());
            }
            let db_path = app_dir.join("iptv.db");

            let conn = match Connection::open(&db_path) {
                Ok(conn) => conn,
                Err(e) => {
                    error!("Failed to open database: {}", e);
                    return Err(e.into());
                }
            };
            if let Err(e) = init_db(&conn) {
                error!("Failed to initialize database: {}", e);
                return Err(e.into());
            }
            let db = DbConnection(Mutex::new(conn));

            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            initialize_database,
            add_playlist,
            get_playlists,
            delete_playlist
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
