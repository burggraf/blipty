use tauri::Manager;

pub mod channel_commands;
pub mod commands;
pub mod db;
pub mod models;
pub mod playlist_commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            println!("Starting database initialization...");

            // Use platform-specific path for Android
            let db_path = if cfg!(target_os = "android") {
                println!("Running on Android, getting app_local_data_dir...");
                let dir = app_handle
                    .path()
                    .app_local_data_dir()
                    .expect("Failed to get Android local data directory");

                println!("Creating directory at {:?}", dir);
                std::fs::create_dir_all(&dir).expect("Failed to create database directory");
                let path = dir.join("blipty.db");
                println!("Final database path: {:?}", path);
                path
            } else {
                let app_data_dir = app_handle
                    .path()
                    .app_data_dir()
                    .expect("Failed to get app data directory");
                std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");
                app_data_dir.join("blipty.db")
            };

            println!("Attempting to open database at {:?}", db_path);

            // Try to open or create the database file
            let conn = rusqlite::Connection::open(&db_path).expect("Failed to open database");
            println!("Successfully opened database connection");

            // Initialize the database schema
            println!("Initializing database schema...");
            db::init_db(&conn).expect("Failed to initialize database schema");
            println!("Schema initialized successfully");

            // Initialize tables
            db::check_and_create_channels_table(&conn)
                .expect("Failed to check/create channels table");
            db::migrate_db_v1(&conn).expect("Failed to migrate database");

            // Create and manage the database connection state
            let db_connection = db::DbConnection(std::sync::Mutex::new(conn));
            app.manage(db_connection);
            println!("Database connection managed successfully");

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
            commands::initialize_database
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
