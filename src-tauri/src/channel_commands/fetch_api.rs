use reqwest;
use serde_json::Value;
use std::collections::HashMap;

use crate::channel_commands::extract_channels::extract_channels;
use crate::models::Error;

pub async fn fetch_api_data(
    server_url: String,
    username: String,
    password: String,
) -> Result<
    (
        Value,
        HashMap<String, String>,
        HashMap<String, String>,
        HashMap<String, String>,
    ),
    Error,
> {
    let mut _api_data = Value::Null;
    let client = reqwest::Client::new();
    let mut live_categories: HashMap<String, String> = HashMap::new();
    let mut vod_categories: HashMap<String, String> = HashMap::new();
    let mut series_categories: HashMap<String, String> = HashMap::new();
    let mut all_channels: Vec<Value> = Vec::new();

    // Fetch live categories
    let live_categories_endpoint = format!(
        "{}/player_api.php?username={}&password={}&action=get_live_categories",
        server_url, username, password
    );

    println!(
        "Trying live categories endpoint: {}",
        live_categories_endpoint
    );

    match client.get(&live_categories_endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!(
                    "Successfully connected to live categories endpoint: {}",
                    live_categories_endpoint
                );

                match response.json::<Value>().await {
                    Ok(data) => {
                        println!("Successfully parsed live categories JSON data");

                        if let Some(categories_array) = data.as_array() {
                            for category in categories_array {
                                if let Some(category_id) =
                                    category.get("category_id").and_then(|v| v.as_str())
                                {
                                    if let Some(category_name) =
                                        category.get("category_name").and_then(|v| v.as_str())
                                    {
                                        live_categories.insert(
                                            category_id.to_string(),
                                            category_name.to_string(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!(
                            "Failed to parse live categories JSON from {}: {}",
                            live_categories_endpoint, e
                        );
                    }
                }
            } else {
                println!(
                    "Failed to connect to live categories endpoint {}: {}",
                    live_categories_endpoint,
                    response.status()
                );
            }
        }
        Err(e) => {
            println!(
                "Error connecting to live categories endpoint {}: {}",
                live_categories_endpoint, e
            );
        }
    }

    // Fetch VOD categories
    let vod_categories_endpoint = format!(
        "{}/player_api.php?username={}&password={}&action=get_vod_categories",
        server_url, username, password
    );

    println!(
        "Trying VOD categories endpoint: {}",
        vod_categories_endpoint
    );

    match client.get(&vod_categories_endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!(
                    "Successfully connected to VOD categories endpoint: {}",
                    vod_categories_endpoint
                );

                match response.json::<Value>().await {
                    Ok(data) => {
                        println!("Successfully parsed VOD categories JSON data");

                        if let Some(categories_array) = data.as_array() {
                            for category in categories_array {
                                if let Some(category_id) =
                                    category.get("category_id").and_then(|v| v.as_str())
                                {
                                    if let Some(category_name) =
                                        category.get("category_name").and_then(|v| v.as_str())
                                    {
                                        vod_categories.insert(
                                            category_id.to_string(),
                                            category_name.to_string(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!(
                            "Failed to parse VOD categories JSON from {}: {}",
                            vod_categories_endpoint, e
                        );
                    }
                }
            } else {
                println!(
                    "Failed to connect to VOD categories endpoint {}: {}",
                    vod_categories_endpoint,
                    response.status()
                );
            }
        }
        Err(e) => {
            println!(
                "Error connecting to VOD categories endpoint {}: {}",
                vod_categories_endpoint, e
            );
        }
    }

    // Fetch series categories
    let series_categories_endpoint = format!(
        "{}/player_api.php?username={}&password={}&action=get_series_categories",
        server_url, username, password
    );

    println!(
        "Trying series categories endpoint: {}",
        series_categories_endpoint
    );

    match client.get(&series_categories_endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!(
                    "Successfully connected to series categories endpoint: {}",
                    series_categories_endpoint
                );

                match response.json::<Value>().await {
                    Ok(data) => {
                        println!("Successfully parsed series categories JSON data");

                        if let Some(categories_array) = data.as_array() {
                            for category in categories_array {
                                if let Some(category_id) =
                                    category.get("category_id").and_then(|v| v.as_str())
                                {
                                    if let Some(category_name) =
                                        category.get("category_name").and_then(|v| v.as_str())
                                    {
                                        series_categories.insert(
                                            category_id.to_string(),
                                            category_name.to_string(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!(
                            "Failed to parse series categories JSON from {}: {}",
                            series_categories_endpoint, e
                        );
                    }
                }
            } else {
                println!(
                    "Failed to connect to series categories endpoint {}: {}",
                    series_categories_endpoint,
                    response.status()
                );
            }
        }
        Err(e) => {
            println!(
                "Error connecting to series categories endpoint {}: {}",
                series_categories_endpoint, e
            );
        }
    }

    // Fetch live streams
    let live_streams_endpoint = format!(
        "{}/player_api.php?username={}&password={}&action=get_live_streams",
        server_url, username, password
    );

    println!("Trying live streams endpoint: {}", live_streams_endpoint);

    match client.get(&live_streams_endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!(
                    "Successfully connected to live streams endpoint: {}",
                    live_streams_endpoint
                );

                match response.json::<Value>().await {
                    Ok(data) => {
                        println!("Successfully parsed live streams JSON data");

                        let live_channels = extract_channels(&data, "live".to_string());
                        all_channels.extend(live_channels);
                    }
                    Err(e) => {
                        println!("Failed to parse JSON from {}: {}", live_streams_endpoint, e);
                    }
                }
            } else {
                println!(
                    "Failed to connect to live streams endpoint {}: {}",
                    live_streams_endpoint,
                    response.status()
                );
            }
        }
        Err(e) => {
            println!(
                "Error connecting to live streams endpoint {}: {}",
                live_streams_endpoint, e
            );
        }
    }

    // Fetch VOD streams
    let vod_streams_endpoint = format!(
        "{}/player_api.php?username={}&password={}&action=get_vod_streams",
        server_url, username, password
    );

    println!("Trying VOD streams endpoint: {}", vod_streams_endpoint);

    match client.get(&vod_streams_endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!(
                    "Successfully connected to VOD streams endpoint: {}",
                    vod_streams_endpoint
                );

                match response.json::<Value>().await {
                    Ok(data) => {
                        println!("Successfully parsed VOD streams JSON data");

                        let vod_channels = extract_channels(&data, "vod".to_string());
                        all_channels.extend(vod_channels);
                    }
                    Err(e) => {
                        println!("Failed to parse JSON from {}: {}", vod_streams_endpoint, e);
                    }
                }
            } else {
                println!(
                    "Failed to connect to VOD streams endpoint {}: {}",
                    vod_streams_endpoint,
                    response.status()
                );
            }
        }
        Err(e) => {
            println!(
                "Error connecting to VOD streams endpoint {}: {}",
                vod_streams_endpoint, e
            );
        }
    }

    // Fetch series streams
    let series_streams_endpoint = format!(
        "{}/player_api.php?username={}&password={}&action=get_series",
        server_url, username, password
    );

    println!(
        "Trying series streams endpoint: {}",
        series_streams_endpoint
    );

    match client.get(&series_streams_endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!(
                    "Successfully connected to series streams endpoint: {}",
                    series_streams_endpoint
                );

                match response.json::<Value>().await {
                    Ok(data) => {
                        println!("Successfully parsed series streams JSON data");

                        let series_channels = extract_channels(&data, "series".to_string());
                        all_channels.extend(series_channels);
                    }
                    Err(e) => {
                        println!(
                            "Failed to parse JSON from {}: {}",
                            series_streams_endpoint, e
                        );
                    }
                }
            } else {
                println!(
                    "Failed to connect to series streams endpoint {}: {}",
                    series_streams_endpoint,
                    response.status()
                );
            }
        }
        Err(e) => {
            println!(
                "Error connecting to series streams endpoint {}: {}",
                series_streams_endpoint, e
            );
        }
    }

    Ok((Value::Array(all_channels), live_categories, vod_categories, series_categories))
}
