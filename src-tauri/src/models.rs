use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: Option<i64>,
    pub playlist_id: i64,
    pub category_id: String,
    pub name: String,
    pub content_type: String,
    pub parent_id: Option<i64>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Channel {
    pub id: Option<i64>,
    pub playlist_id: i64,
    pub category_id: Option<String>,
    pub stream_id: String,
    pub name: String,
    pub stream_type: String,
    pub type_name: Option<String>,
    pub stream_url: String,
    pub stream_icon: Option<String>,
    pub epg_channel_id: Option<String>,
    pub added: Option<String>,
    pub series_no: Option<String>,
    pub live: Option<String>,
    pub container_extension: Option<String>,
    pub custom_sid: Option<String>,
    pub tv_archive: Option<i64>,
    pub direct_source: Option<String>,
    pub tv_archive_duration: Option<i64>,
    pub num: Option<String>,
    pub plot: Option<String>,
    pub cast: Option<String>,
    pub director: Option<String>,
    pub genre: Option<String>,
    pub release_date: Option<String>,
    pub rating: Option<String>,
    pub rating_5based: Option<f64>,
    pub backdrop_path: Option<Vec<String>>,
    pub youtube_trailer: Option<String>,
    pub episode_run_time: Option<String>,
    pub cover: Option<String>,
    pub created_at: Option<String>,
    pub category_name: Option<String>,
    pub content_type: Option<String>,
    pub authenticated_stream_url: Option<String>,
    pub is_selected: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Playlist {
    pub id: Option<i64>,
    pub name: String,
    pub server_url: String,
    pub username: String,
    pub password: String,
    pub epg_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub last_updated: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    Io(String),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
