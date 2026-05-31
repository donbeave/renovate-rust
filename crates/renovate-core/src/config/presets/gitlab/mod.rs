//! GitLab preset fetching.
//!
//! Renovate reference: `lib/config/presets/gitlab/index.ts`.

use serde_json::Value;

pub const ENDPOINT: &str = "https://gitlab.com/api/v4/";

/// Fetch a preset from a GitLab repository.
pub async fn fetch_gitlab_preset(
    _repo: &str,
    _file_name: &str,
    _endpoint: &str,
    _tag: Option<&str>,
) -> Result<Value, String> {
    Err("GitLab preset fetching requires HTTP client".to_owned())
}

/// Get a preset from the default GitLab endpoint.
pub async fn get_preset(
    repo: &str,
    preset_name: &str,
    _preset_path: Option<&str>,
    tag: Option<&str>,
) -> Result<Value, String> {
    fetch_gitlab_preset(repo, preset_name, ENDPOINT, tag).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_is_gitlab_api() {
        assert_eq!(ENDPOINT, "https://gitlab.com/api/v4/");
    }
}
