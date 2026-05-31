//! Local preset fetching.
//!
//! Renovate reference: `lib/config/presets/local/index.ts`.

use serde_json::Value;

/// Fetch a preset from a local file or platform repo.
pub async fn fetch_local_preset(
    _repo: &str,
    _file_name: &str,
    _endpoint: &str,
    _tag: Option<&str>,
) -> Result<Value, String> {
    Err("Local preset fetching requires platform integration".to_owned())
}

/// Get a preset from a local source.
pub async fn get_preset(
    repo: &str,
    preset_name: &str,
    _preset_path: Option<&str>,
    tag: Option<&str>,
) -> Result<Value, String> {
    fetch_local_preset(repo, preset_name, "", tag).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn local_module_exists() {}
}
