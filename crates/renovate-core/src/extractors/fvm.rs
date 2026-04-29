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

    // Ported: "returns a result for .fvmrc" — fvm/extract.spec.ts line 41
    #[test]
    fn extracts_flutter_key() {
        let dep = extract(r#"{"flutter": "3.16.5"}"#).unwrap();
        assert_eq!(dep.version, "3.16.5");
    }

    // Ported: "returns a result for .fvm/fvm_config.json" — fvm/extract.spec.ts line 26
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

    // Ported: "returns null for empty flutter sdk version" — fvm/extract.spec.ts line 13
    #[test]
    fn missing_version_returns_none() {
        assert!(extract(r#"{"channel": "stable"}"#).is_none());
    }

    // Ported: "returns null for invalid json" — fvm/extract.spec.ts line 7
    #[test]
    fn invalid_json_returns_none() {
        assert!(extract("not json").is_none());
    }

    // Ported: "returns null for non string flutter sdk version" — fvm/extract.spec.ts line 17
    #[test]
    fn non_string_flutter_sdk_version_returns_none() {
        assert!(extract(r#"{"flutterSdkVersion": 2.1, "flavors": {}}"#).is_none());
    }

    // Ported: "supports non range for .fvm/fvm_config.json" — fvm/extract.spec.ts line 53
    #[test]
    fn flutter_sdk_version_channel_extracted() {
        let dep = extract(r#"{"flutterSdkVersion": "stable", "flavors": {}}"#).unwrap();
        assert_eq!(dep.version, "stable");
    }

    // Ported: "supports non range for .fvmrc" — fvm/extract.spec.ts line 68
    #[test]
    fn flutter_channel_extracted() {
        let dep = extract(r#"{"flutter": "stable"}"#).unwrap();
        assert_eq!(dep.version, "stable");
    }
}
