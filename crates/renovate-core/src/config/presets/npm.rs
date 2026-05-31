//! npm preset fetching.
//!
//! Renovate reference: `lib/config/presets/npm/index.ts`.

use serde_json::Value;

/// Fetch a preset from an npm package's `renovate-config` field.
pub async fn fetch_npm_preset(
    _package: &str,
    _preset_name: &str,
) -> Result<Value, String> {
    Err("npm preset fetching requires HTTP client".to_owned())
}

/// Get a preset from an npm package.
pub async fn get_preset(
    repo: &str,
    preset_name: &str,
) -> Result<Value, String> {
    fetch_npm_preset(repo, preset_name).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn npm_module_exists() {}
}
