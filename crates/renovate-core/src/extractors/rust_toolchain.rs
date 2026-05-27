//! Rust toolchain file dependency extractor.
//!
//! Handles `rust-toolchain.toml` (TOML format with `[toolchain].channel`)
//! and legacy `rust-toolchain` files (single-line channel value).
//!
//! Renovate reference:
//! - `lib/modules/manager/rust-toolchain/extract.ts`
//! - `lib/modules/manager/rust-toolchain/schema.ts`

use regex::Regex;
use std::sync::LazyLock;

const DATASOURCE: &str = "rust-version";

/// A single dependency extracted from a rust-toolchain file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustToolchainDep {
    pub dep_name: &'static str,
    pub dep_type: &'static str,
    pub current_value: String,
    pub datasource: &'static str,
}

static CHANNEL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:\d+\.\d+(?:\.\d+)?|stable|beta|nightly(?:-\d{4}-\d{2}-\d{2})?)$").unwrap()
});

fn is_valid_channel(channel: &str) -> bool {
    CHANNEL_RE.is_match(channel)
}

fn make_dep(channel: String) -> Option<RustToolchainDep> {
    if !is_valid_channel(&channel) {
        return None;
    }
    Some(RustToolchainDep {
        dep_name: "rust",
        dep_type: "toolchain",
        current_value: channel,
        datasource: DATASOURCE,
    })
}

/// Extract rust-toolchain dependency from file content.
///
/// Returns `Some(vec)` with one dep, or `None` when the file is unrecognised.
pub fn extract(content: &str, package_file: &str) -> Option<Vec<RustToolchainDep>> {
    // Try TOML parse first.
    if let Some(channel) = parse_toml_channel(content) {
        return make_dep(channel).map(|d| vec![d]);
    }

    // For .toml files TOML parsing must succeed.
    if package_file.ends_with(".toml") {
        return None;
    }

    // Legacy single-line format.
    let lines: Vec<&str> = content
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();

    if lines.len() != 1 {
        return None;
    }

    make_dep(lines[0].to_owned()).map(|d| vec![d])
}

/// Parse the `[toolchain].channel` field from a TOML string.
/// Returns `None` on any parse or schema error.
fn parse_toml_channel(content: &str) -> Option<String> {
    let root = toml::from_str::<toml::Value>(content).ok()?;
    let channel = root.get("toolchain")?.get("channel")?.as_str()?;
    if channel.is_empty() {
        return None;
    }
    Some(channel.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dep(channel: &str) -> Option<Vec<RustToolchainDep>> {
        Some(vec![RustToolchainDep {
            dep_name: "rust",
            dep_type: "toolchain",
            current_value: channel.into(),
            datasource: DATASOURCE,
        }])
    }

    // ── schema.spec.ts tests ──────────────────────────────────────────────────

    // Ported: "parses valid TOML with channel" — manager/rust-toolchain/schema.spec.ts line 6
    #[test]
    fn schema_parses_valid_toml_with_channel() {
        assert_eq!(
            parse_toml_channel("[toolchain]\nchannel = \"1.89.1\"\n"),
            Some("1.89.1".into())
        );
    }

    // Ported: "parses TOML with additional fields" — manager/rust-toolchain/schema.spec.ts line 21
    #[test]
    fn schema_parses_toml_with_additional_fields() {
        let toml = "[toolchain]\nchannel = \"1.89.1\"\ncomponents = [\"rustfmt\", \"clippy\"]\n";
        assert_eq!(parse_toml_channel(toml), Some("1.89.1".into()));
    }

    // Ported: "throws error for invalid TOML" — manager/rust-toolchain/schema.spec.ts line 38
    #[test]
    fn schema_rejects_invalid_toml() {
        assert!(parse_toml_channel("this is not valid toml [").is_none());
    }

    // Ported: "throws error for missing toolchain section" — manager/rust-toolchain/schema.spec.ts line 44
    #[test]
    fn schema_rejects_missing_toolchain_section() {
        assert!(parse_toml_channel("[other]\nchannel = \"1.89.1\"\n").is_none());
    }

    // Ported: "throws error for missing channel field" — manager/rust-toolchain/schema.spec.ts line 53
    #[test]
    fn schema_rejects_missing_channel_field() {
        assert!(parse_toml_channel("[toolchain]\ncomponents = [\"rustfmt\"]\n").is_none());
    }

    // Ported: "throws error for non-string channel" — manager/rust-toolchain/schema.spec.ts line 62
    #[test]
    fn schema_rejects_non_string_channel() {
        assert!(parse_toml_channel("[toolchain]\nchannel = 123\n").is_none());
    }

    // Ported: "throws error for empty channel" — manager/rust-toolchain/schema.spec.ts line 71
    #[test]
    fn schema_rejects_empty_channel() {
        assert!(parse_toml_channel("[toolchain]\nchannel = \"\"\n").is_none());
    }

    // Ported: "parses nightly channel" — manager/rust-toolchain/schema.spec.ts line 80
    #[test]
    fn schema_parses_nightly_channel() {
        assert_eq!(
            parse_toml_channel("[toolchain]\nchannel = \"nightly-2025-10-12\"\n"),
            Some("nightly-2025-10-12".into())
        );
    }

    // Ported: "parses stable keyword" — manager/rust-toolchain/schema.spec.ts line 95
    #[test]
    fn schema_parses_stable_keyword() {
        assert_eq!(
            parse_toml_channel("[toolchain]\nchannel = \"stable\"\n"),
            Some("stable".into())
        );
    }

    // ── extract.spec.ts tests ─────────────────────────────────────────────────

    // Ported: "extracts major.minor.patch versions" — manager/rust-toolchain/extract.spec.ts line 7
    #[test]
    fn extract_major_minor_patch_version() {
        assert_eq!(
            extract("[toolchain]\nchannel = \"1.89.1\"\n", "rust-toolchain.toml"),
            dep("1.89.1")
        );
    }

    // Ported: "extracts major.minor ranges" — manager/rust-toolchain/extract.spec.ts line 27
    #[test]
    fn extract_major_minor_range() {
        assert_eq!(
            extract("[toolchain]\nchannel = \"1.89\"\n", "rust-toolchain.toml"),
            dep("1.89")
        );
    }

    // Ported: "extracts beta channel" — manager/rust-toolchain/extract.spec.ts line 47
    #[test]
    fn extract_beta_channel() {
        assert_eq!(
            extract("[toolchain]\nchannel = \"beta\"\n", "rust-toolchain.toml"),
            dep("beta")
        );
    }

    // Ported: "extracts nightly channel" — manager/rust-toolchain/extract.spec.ts line 67
    #[test]
    fn extract_nightly_channel() {
        assert_eq!(
            extract(
                "[toolchain]\nchannel = \"nightly\"\n",
                "rust-toolchain.toml"
            ),
            dep("nightly")
        );
    }

    // Ported: "extracts dated nightly channel" — manager/rust-toolchain/extract.spec.ts line 87
    #[test]
    fn extract_dated_nightly_channel() {
        assert_eq!(
            extract(
                "[toolchain]\nchannel = \"nightly-2025-10-12\"\n",
                "rust-toolchain.toml"
            ),
            dep("nightly-2025-10-12")
        );
    }

    // Ported: "returns null for invalid TOML" — manager/rust-toolchain/extract.spec.ts line 107
    #[test]
    fn extract_returns_none_for_invalid_toml() {
        assert_eq!(
            extract("this is not valid toml [", "rust-toolchain.toml"),
            None
        );
    }

    // Ported: "returns null when [toolchain] section is absent" — manager/rust-toolchain/extract.spec.ts line 115
    #[test]
    fn extract_returns_none_when_no_toolchain_section() {
        assert_eq!(
            extract("channel = \"1.89.1\"\n", "rust-toolchain.toml"),
            None
        );
    }

    // Ported: "returns null when channel is absent" — manager/rust-toolchain/extract.spec.ts line 123
    #[test]
    fn extract_returns_none_when_channel_absent() {
        assert_eq!(
            extract(
                "[toolchain]\ncomponents = [\"rustfmt\"]\n",
                "rust-toolchain.toml"
            ),
            None
        );
    }

    // Ported: "returns null for unparseable channel value" — manager/rust-toolchain/extract.spec.ts line 134
    #[test]
    fn extract_returns_none_for_invalid_channel() {
        assert_eq!(
            extract(
                "[toolchain]\nchannel = \"not-a-rust-channel\"\n",
                "rust-toolchain.toml"
            ),
            None
        );
    }

    // Ported: "can handle additional fields" — manager/rust-toolchain/extract.spec.ts line 145
    #[test]
    fn extract_handles_additional_fields() {
        let toml = "[toolchain]\nchannel = \"1.89.1\"\ncomponents = [\"rustfmt\", \"clippy\"]\n";
        assert_eq!(extract(toml, "rust-toolchain.toml"), dep("1.89.1"));
    }

    // Ported: "can read from legacy filename" — manager/rust-toolchain/extract.spec.ts line 167
    #[test]
    fn extract_reads_from_legacy_filename() {
        let toml = "[toolchain]\nchannel = \"1.89.1\"\n";
        assert_eq!(extract(toml, "rust-toolchain"), dep("1.89.1"));
    }

    // Ported: "returns null for empty legacy file" — manager/rust-toolchain/extract.spec.ts line 187
    #[test]
    fn extract_returns_none_for_empty_legacy_file() {
        assert_eq!(extract("", "rust-toolchain"), None);
    }

    // Ported: "extracts from legacy format" — manager/rust-toolchain/extract.spec.ts line 192
    #[test]
    fn extract_from_legacy_format() {
        assert_eq!(extract("1.89.1\n", "rust-toolchain"), dep("1.89.1"));
    }

    // Ported: "returns null for multiline legacy files" — manager/rust-toolchain/extract.spec.ts line 206
    #[test]
    fn extract_returns_none_for_multiline_legacy() {
        assert_eq!(
            extract("1.89.1\nextra line\nanother line\n", "rust-toolchain"),
            None
        );
    }
}
