use reqwest;
use serde_json::Value;

use crate::models::Error;

pub async fn fetch_api_data(
    server_url: String,
    username: String,
    password: String,
) -> Result<Value, Error> {
    let mut api_data = Value::Null;
    let client = reqwest::Client::new();

    // Try different API endpoint formats commonly used by IPTV providers
    let endpoints = vec![
        format!(
            "{}/api/panel_api.php?username={}&password={}",
            server_url, username, password
        ),
        format!(
            "{}/player_api.php?username={}&password={}&action=get_live_streams",
            server_url, username, password
        ),
        format!(
            "{}/player_api.php?username={}&password={}&action=get_live_categories",
            server_url, username, password
        ),
        format!(
            "{}/get.php?username={}&password={}&type=m3u_plus",
            server_url, username, password
        ),
    ];

    let mut success = false;
    for endpoint in endpoints {
        println!("Trying API endpoint: {}", endpoint);
        match client.get(&endpoint).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Successfully connected to: {}", endpoint);

                    // For M3U format, handle differently
                    if endpoint.contains("m3u_plus") {
                        let m3u_content = response.text().await?;
                        println!("Received M3U content, processing...");
                        api_data = Value::String(m3u_content);
                        success = true;
                        break;
                    } else {
                        // For JSON API formats
                        match response.json::<Value>().await {
                            Ok(data) => {
                                println!("Successfully parsed JSON data");
                                // Print the top-level structure of the JSON
                                if let Some(obj) = data.as_object() {
                                    println!("JSON structure has the following top-level keys:");
                                    for (key, value) in obj {
                                        let type_str = match value {
                                            Value::Null => "null",
                                            Value::Bool(_) => "boolean",
                                            Value::Number(_) => "number",
                                            Value::String(_) => "string",
                                            Value::Array(_) => "array",
                                            Value::Object(_) => "object",
                                        };
                                        println!("  - {}: {}", key, type_str);
                                    }
                                } else {
                                    println!("JSON data is not an object, it's a: {:?}", data);
                                }
                                api_data = data;
                                success = true;
                                break;
                            }
                            Err(e) => {
                                println!("Failed to parse JSON from {}: {}", endpoint, e);
                                // Continue to next endpoint
                            }
                        }
                    }
                } else {
                    println!("Failed to connect to {}: {}", endpoint, response.status());
                }
            }
            Err(e) => {
                println!("Error connecting to {}: {}", endpoint, e);
                // Continue to next endpoint
            }
        }
    }

    if !success && api_data == Value::Null {
        return Err(Error::Internal(
            "Failed to fetch data from any API endpoint".to_string(),
        ));
    }

    Ok(api_data)
}
