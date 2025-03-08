use serde::Serialize;
use tauri::{AppHandle, Manager, Runtime, Emitter};

#[derive(Debug, Clone, Serialize)]
pub enum PlayerEvent {
    StateChanged {
        state: String,
    },
    BufferingProgress {
        percent: i32,
    },
    QualityChanged {
        width: i32,
        height: i32,
        framerate: Option<f64>,
        bitrate: Option<u32>,
    },
    Error {
        code: i32,
        message: String,
    },
    EndOfStream,
}

pub fn emit_player_event<R: Runtime>(app: &AppHandle<R>, event: PlayerEvent) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.emit("player-event", event);
    }
}
