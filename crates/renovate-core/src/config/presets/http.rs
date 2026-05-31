//! HTTP preset fetching.
//!
//! Renovate reference: `lib/config/presets/http/index.ts`.

use serde_json::Value;

/// Fetch a preset from an HTTP URL.
pub async fn fetch_http_preset(_url: &str) -> Result<Value, String> {
    Err("HTTP preset fetching requires HTTP client".to_owned())
}

/// Get a preset from an HTTP URL.
pub async fn get_preset(url: &str) -> Result<Value, String> {
    fetch_http_preset(url).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn http_module_exists() {}
}
