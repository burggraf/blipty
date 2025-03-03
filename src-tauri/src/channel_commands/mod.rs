use tauri::State;

pub mod category_commands;
pub mod import_commands;

mod fetch_data;
mod process_m3u;

use crate::{db::DbConnection, models::Channel, models::Error};

#[tauri::command(rename_all = "camelCase")]
pub async fn fetch_channels(
    db: State<'_, DbConnection>,
    playlist_id: i64,
) -> Result<Vec<Channel>, Error> {
    println!("fetch_channels called with playlist_id: {}", playlist_id);
    let conn = db.0.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM channels WHERE playlist_id = ?")?;
    let channels = stmt
        .query_map([playlist_id], |row| {
            Ok(Channel {
                id: row.get(0)?,
                playlist_id: row.get(1)?,
                category_id: row.get(2)?,
                category_name: row.get(3)?,
                stream_id: row.get(4)?,
                name: row.get(5)?,
                stream_type: row.get(6)?,
                stream_url: row.get(7)?,
                authenticated_stream_url: row.get(8)?,
                created_at: row.get(9)?,
                is_selected: row.get(10)?,
                // Set default values for other fields that aren't in the database
                type_name: None,
                stream_icon: None,
                epg_channel_id: None,
                added: None,
                series_no: None,
                live: None,
                container_extension: None,
                custom_sid: None,
                tv_archive: None,
                direct_source: None,
                tv_archive_duration: None,
                num: None,
                plot: None,
                cast: None,
                director: None,
                genre: None,
                release_date: None,
                rating: None,
                rating_5based: None,
                backdrop_path: None,
                youtube_trailer: None,
                episode_run_time: None,
                cover: None,
                content_type: None,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(channels)
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_selected_channel(db: State<'_, DbConnection>) -> Result<Option<Channel>, Error> {
    let conn = db.0.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM channels WHERE is_selected = 1 LIMIT 1")?;
    let channel = stmt
        .query_map([], |row| {
            Ok(Channel {
                id: row.get(0)?,
                playlist_id: row.get(1)?,
                category_id: row.get(2)?,
                category_name: row.get(3)?,
                stream_id: row.get(4)?,
                name: row.get(5)?,
                stream_type: row.get(6)?,
                stream_url: row.get(7)?,
                authenticated_stream_url: row.get(8)?,
                created_at: row.get(9)?,
                is_selected: row.get(10)?,
                // Set default values for other fields that aren't in the database
                type_name: None,
                stream_icon: None,
                epg_channel_id: None,
                added: None,
                series_no: None,
                live: None,
                container_extension: None,
                custom_sid: None,
                tv_archive: None,
                direct_source: None,
                tv_archive_duration: None,
                num: None,
                plot: None,
                cast: None,
                director: None,
                genre: None,
                release_date: None,
                rating: None,
                rating_5based: None,
                backdrop_path: None,
                youtube_trailer: None,
                episode_run_time: None,
                cover: None,
                content_type: None,
            })
        })?
        .next()
        .transpose()?;
    Ok(channel)
}

#[tauri::command(rename_all = "camelCase")]
pub async fn set_selected_channel(
    db: State<'_, DbConnection>,
    channel_id: i64,
) -> Result<(), Error> {
    let conn = db.0.lock().unwrap();
    // First reset all selected channels
    conn.execute("UPDATE channels SET is_selected = 0", [])?;
    // Then set the new selected channel
    conn.execute(
        "UPDATE channels SET is_selected = 1 WHERE id = ?",
        [channel_id],
    )?;
    Ok(())
}
