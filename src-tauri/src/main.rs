// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::Manager;

mod channel_commands;
mod commands;
mod db;
mod models;
mod playlist_commands;
mod video_player;

#[cfg(not(mobile))]
fn main() {
    tauri::Builder::default()
        .manage(db::DbConnection::new())
        .setup(|app| {
            let app_handle = app.handle();
            let db_path = app_handle
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir")
                .join("blipty.db");

            println!("Database path: {:?}", db_path);

            std::fs::create_dir_all(db_path.parent().unwrap())
                .expect("Failed to create app data dir");

            // Try to open or create the database file
            let conn = rusqlite::Connection::open(&db_path).expect("Failed to open database");

            // Initialize the database schema
            db::init_db(&conn).expect("Failed to initialize database schema");
            db::check_and_create_channels_table(&conn)
                .expect("Failed to check/create channels table");
            db::migrate_db_v1(&conn).expect("Failed to migrate database");

            // Initialize video player before managing it
            let video_player = video_player::VideoPlayer::<tauri::Wry>::new(app_handle.clone())
                .expect("Failed to initialize video player");
            app.manage(video_player);

            // Create and manage the database connection state
            let db_connection = db::DbConnection(std::sync::Mutex::new(conn));
            app.manage(db_connection);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            playlist_commands::add_playlist,
            playlist_commands::get_playlists,
            playlist_commands::delete_playlist,
            playlist_commands::update_playlist,
            channel_commands::fetch_channels,
            channel_commands::get_selected_channel,
            channel_commands::set_selected_channel,
            channel_commands::category_commands::get_categories,
            channel_commands::import_commands::fetch_and_populate_data,
            channel_commands::import_commands::process_m3u_content,
            commands::get_db_path,
            commands::initialize_database,
            video_player::commands::initialize_player,
            video_player::commands::play_video,
            video_player::commands::pause_video,
            video_player::commands::resume_video,
            video_player::commands::stop_video,
            video_player::commands::get_player_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
