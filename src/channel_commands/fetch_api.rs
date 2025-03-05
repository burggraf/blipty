// For Android, we use a minimal reqwest client without native TLS
#[cfg(target_os = "android")]
use reqwest::{Client, ClientBuilder};

// For other platforms, use the default reqwest
#[cfg(not(target_os = "android"))]
use reqwest;

// Rest of your file continues here
#[cfg(target_os = "android")]

// For other platforms, use the default reqwest
#[cfg(not(target_os = "android"))]

// Rest of your file continues here
#[cfg(target_os = "android")]

// For other platforms, use the default reqwest
#[cfg(not(target_os = "android"))]

// Rest of your file continues here
// ... (without any duplicate reqwest imports)
