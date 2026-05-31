//! Gitea preset fetching.
//!
//! Renovate reference: `lib/config/presets/gitea/index.ts`.

use serde_json::Value;

pub const ENDPOINT: &str = "https://gitea.com/";

/// Fetch a preset from a Gitea repository.
pub async fn fetch_gitea_preset(
    _repo: &str,
    _file_name: &str,
    _endpoint: &str,
    _tag: Option<&str>,
) -> Result<Value, String> {
    Err("Gitea preset fetching requires HTTP client".to_owned())
}

/// Get a preset from the default Gitea endpoint.
pub async fn get_preset(
    repo: &str,
    preset_name: &str,
    _preset_path: Option<&str>,
    tag: Option<&str>,
) -> Result<Value, String> {
    fetch_gitea_preset(repo, preset_name, ENDPOINT, tag).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_is_gitea() {
        assert_eq!(ENDPOINT, "https://gitea.com/");
    }
}
