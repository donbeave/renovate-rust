//! Forgejo preset fetching.
//!
//! Renovate reference: `lib/config/presets/forgejo/index.ts`.

use serde_json::Value;

pub const ENDPOINT: &str = "https://code.forgejo.org/";

/// Fetch a preset from a Forgejo repository.
pub async fn fetch_forgejo_preset(
    _repo: &str,
    _file_name: &str,
    _endpoint: &str,
    _tag: Option<&str>,
) -> Result<Value, String> {
    Err("Forgejo preset fetching requires HTTP client".to_owned())
}

/// Get a preset from the default Forgejo endpoint.
pub async fn get_preset(
    repo: &str,
    preset_name: &str,
    _preset_path: Option<&str>,
    tag: Option<&str>,
) -> Result<Value, String> {
    fetch_forgejo_preset(repo, preset_name, ENDPOINT, tag).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_is_forgejo() {
        assert_eq!(ENDPOINT, "https://code.forgejo.org/");
    }
}
