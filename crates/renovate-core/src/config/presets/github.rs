//! GitHub preset fetching.
//!
//! Renovate reference: `lib/config/presets/github/index.ts`.

use serde_json::Value;

pub const ENDPOINT: &str = "https://api.github.com/";

/// Fetch a preset from a GitHub repository.
///
/// In the full implementation, this would use the GitHub API to fetch
/// a JSON file from a repository's contents. For now, returns an error
/// indicating the preset could not be found.
pub async fn fetch_github_preset(
    _repo: &str,
    _file_name: &str,
    _endpoint: &str,
    _tag: Option<&str>,
) -> Result<Value, String> {
    Err("GitHub preset fetching requires HTTP client".to_owned())
}

/// Get a preset from the default GitHub endpoint.
pub async fn get_preset(
    repo: &str,
    preset_name: &str,
    _preset_path: Option<&str>,
    tag: Option<&str>,
) -> Result<Value, String> {
    fetch_github_preset(repo, preset_name, ENDPOINT, tag).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_is_github_api() {
        assert_eq!(ENDPOINT, "https://api.github.com/");
    }
}
