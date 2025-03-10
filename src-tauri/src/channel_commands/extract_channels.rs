use serde_json::Value;
// use std::collections::HashMap;

pub fn extract_channels(api_data: &Value, stream_type: String) -> Vec<Value> {
    let mut all_channels = Vec::new();
    println!("Attempting to extract channels from JSON data...");

    // Structure 1: panel_api.php format with available_channels object
    if let Some(available_channels) = api_data["available_channels"].as_object() {
        println!("Found panel_api.php style channels structure");
        for (stream_id, channel_data) in available_channels {
            let mut channel = channel_data.clone();
            channel["stream_id"] = serde_json::Value::String(stream_id.clone());
            if channel.get("stream_type").is_none() {
                channel["stream_type"] = serde_json::Value::String(stream_type.clone());
            }
            all_channels.push(channel);
        }
    }
    // Structure 2: player_api.php format with direct array
    else if let Some(channels) = api_data.as_array() {
        println!("Found player_api.php style array structure for channels");
        for channel in channels {
            if channel.is_object() {
                let mut chan = channel.clone();
                if chan.get("stream_type").is_none() {
                    chan["stream_type"] = serde_json::Value::String(stream_type.clone());
                }
                all_channels.push(chan);
            }
        }
    }

    println!("Extracted {} channels", all_channels.len());

    all_channels
}
