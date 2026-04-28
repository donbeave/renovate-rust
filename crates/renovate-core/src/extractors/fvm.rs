//! Flutter Version Manager (`fvm`) config extractor.
//!
//! Extracts the Flutter SDK version from FVM config files:
//! - `.fvm/fvm_config.json` — JSON with `"flutter": "3.16.5"` or `"flutterSdkVersion": "3.16.5"`
//! - `.fvmrc` — same JSON format
//!
//! Renovate reference:
//! - `lib/modules/manager/fvm/extract.ts`
//! - Patterns: `/(^|/)\.fvm/fvm_config\.json$/`, `/(^|/)\.fvmrc$/`
//! - Datasource: flutter-version (we use GitHub Tags `flutter/flutter`)

use serde::Deserialize;

/// The extracted Flutter SDK version from an FVM config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FvmDep {
    pub version: String,
}

#[derive(Debug, Deserialize)]
struct FvmConfig {
    flutter: Option<String>,
    #[serde(rename = "flutterSdkVersion")]
    flutter_sdk_version: Option<String>,
}

/// Extract the Flutter version from an FVM config JSON file.
pub fn extract(content: &str) -> Option<FvmDep> {
    let config: FvmConfig = serde_json::from_str(content.trim()).ok()?;
    let version = config.flutter.or(config.flutter_sdk_version)?;
    if version.is_empty() {
        return None;
    }
    Some(FvmDep { version })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_flutter_key() {
        let dep = extract(r#"{"flutter": "3.16.5"}"#).unwrap();
        assert_eq!(dep.version, "3.16.5");
    }

    #[test]
    fn extracts_flutter_sdk_version_key() {
        let dep = extract(r#"{"flutterSdkVersion": "3.19.0"}"#).unwrap();
        assert_eq!(dep.version, "3.19.0");
    }

    #[test]
    fn flutter_key_takes_precedence() {
        let dep = extract(r#"{"flutter": "3.16.5", "flutterSdkVersion": "3.19.0"}"#).unwrap();
        assert_eq!(dep.version, "3.16.5");
    }

    #[test]
    fn missing_version_returns_none() {
        assert!(extract(r#"{"channel": "stable"}"#).is_none());
    }

    #[test]
    fn invalid_json_returns_none() {
        assert!(extract("not json").is_none());
    }
}
