use crate::db::DbConnection;
use crate::models::{Error, Playlist};
use rusqlite::params;
use tauri::State;

#[tauri::command]
pub async fn add_playlist(db: State<'_, DbConnection>, playlist: Playlist) -> Result<i64, Error> {
    println!("Adding playlist: {:?}", playlist);
    let conn = db.0.lock().unwrap();

    let result = conn.execute(
        "INSERT INTO playlists (name, server_url, username, password, epg_url, created_at, updated_at, last_updated, is_active) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![playlist.name, playlist.server_url, playlist.username, playlist.password, playlist.epg_url, playlist.created_at, playlist.updated_at, playlist.last_updated, playlist.is_active],
    );

    match result {
        Ok(_) => {
            let id = conn.last_insert_rowid();
            println!("Successfully added playlist with ID: {}", id);
            Ok(id)
        }
        Err(e) => {
            println!("Error adding playlist: {:?}", e);
            Err(Error::Database(e))
        }
    }
}

#[tauri::command]
pub async fn get_playlists(db: State<'_, DbConnection>) -> Result<Vec<Playlist>, Error> {
    let conn = db.0.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM playlists")?;
    let playlists = stmt
        .query_map([], |row| {
            Ok(Playlist {
                id: row.get(0)?,
                name: row.get(1)?,
                server_url: row.get(2)?,
                username: row.get(3)?,
                password: row.get(4)?,
                epg_url: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                last_updated: row.get(8)?,
                is_active: row.get(9)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(playlists)
}

#[tauri::command]
pub async fn delete_playlist(db: State<'_, DbConnection>, id: i64) -> Result<(), Error> {
    let conn = db.0.lock().unwrap();
    conn.execute("DELETE FROM playlists WHERE id = ?", [id])?;
    Ok(())
}

#[tauri::command]
pub async fn update_playlist(db: State<'_, DbConnection>, playlist: Playlist) -> Result<(), Error> {
    let conn = db.0.lock().unwrap();
    conn.execute(
        "UPDATE playlists SET name = ?1, server_url = ?2, username = ?3, password = ?4, epg_url = ?5, updated_at = ?6, last_updated = ?7, is_active = ?8 WHERE id = ?9",
        params![playlist.name, playlist.server_url, playlist.username, playlist.password, playlist.epg_url, playlist.updated_at, playlist.last_updated, playlist.is_active, playlist.id],
    )?;
    Ok(())
}
