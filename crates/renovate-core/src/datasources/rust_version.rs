//! Rust toolchain version datasource.
//!
//! Fetches available Rust toolchain releases from the `manifests.txt` index
//! at `https://static.rust-lang.org/manifests.txt`.  Each line in the file is
//! a URL of the form:
//!   `static.rust-lang.org/dist/YYYY-MM-DD/channel-rust-{identifier}.toml`
//!
//! The identifier is one of:
//! - A semver release (e.g. `1.82.0`)
//! - A beta build (e.g. `1.83.0-beta.5`)
//! - `nightly`
//! - `stable` or `beta` — channel aliases (filtered out in the final result)
//!
//! Renovate reference: `lib/modules/datasource/rust-version/index.ts`
//! Renovate reference: `lib/modules/datasource/rust-version/parse.ts`
//! Registry:           `https://static.rust-lang.org`

use std::collections::HashMap;

use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://static.rust-lang.org";
pub const DATASOURCE_ID: &str = "rust-version";

/// Errors from the Rust version datasource.
#[derive(Debug, Error)]
pub enum RustVersionError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

/// Parsed content of a single manifest URL line.
///
/// Mirrors TypeScript's `ParsedManifestUrl` in `parse.ts`.
/// The `version` field is the raw channel identifier (e.g. `"nightly"`,
/// `"stable"`, `"1.82.0"`).  Callers are responsible for transforming
/// `"nightly"` to `"nightly-YYYY-MM-DD"` and filtering channel aliases.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedManifestUrl {
    pub date: String,
    pub version: String,
}

/// Parse a single manifest URL line into its date and version components.
///
/// Accepts lines like:
///   `static.rust-lang.org/dist/2024-10-17/channel-rust-1.82.0.toml`
///   `https://static.rust-lang.org/dist/2025-11-24/channel-rust-nightly.toml`
///
/// Returns `None` for blank lines and URLs that do not match the expected
/// `channel-rust-{identifier}.toml` pattern.  Does **not** filter channel
/// aliases (`stable`, `beta`) — that is the caller's responsibility.
///
/// Mirrors TypeScript's `parseManifestUrl` in `parse.ts`.
pub fn parse_manifest_url(line: &str) -> Option<ParsedManifestUrl> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    // Locate the `channel-rust-` prefix.
    let channel_prefix = "channel-rust-";
    let channel_pos = line.find(channel_prefix)?;
    let after_channel = &line[channel_pos + channel_prefix.len()..];

    // Strip the trailing `.toml` extension.
    let version = after_channel.strip_suffix(".toml")?.to_owned();

    // Extract the date segment immediately before `/channel-rust-...`.
    let before_channel = &line[..channel_pos];
    // Trim the separating '/' so we can find the date component.
    let before_channel_trimmed = before_channel.trim_end_matches('/');
    let slash_pos = before_channel_trimmed.rfind('/')?;
    let date = &before_channel_trimmed[slash_pos + 1..];

    // Basic format check: must look like YYYY-MM-DD (10 chars, hyphens at 4,7).
    if date.len() != 10
        || !date.chars().enumerate().all(|(i, c)| {
            if i == 4 || i == 7 {
                c == '-'
            } else {
                c.is_ascii_digit()
            }
        })
    {
        return None;
    }

    Some(ParsedManifestUrl {
        date: date.to_owned(),
        version,
    })
}

/// One release entry returned by `fetch_releases`.
#[derive(Debug, Clone)]
pub struct RustVersionRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct RustVersionResult {
    pub releases: Vec<RustVersionRelease>,
    pub homepage: &'static str,
    pub source_url: &'static str,
    pub changelog_url: &'static str,
}

/// Fetch all Rust toolchain releases from `registry_url/manifests.txt`.
///
/// Deduplicates versions, keeping the latest date when a version appears
/// more than once.  Filters out the `stable` and `beta` channel aliases
/// and transforms `nightly` to `nightly-YYYY-MM-DD`.
///
/// Returns `None` only when no parseable, non-alias entries exist.
pub async fn fetch_releases(
    registry_url: &str,
    http: &HttpClient,
) -> Result<Option<RustVersionResult>, RustVersionError> {
    let url = format!("{registry_url}/manifests.txt");
    let text = http.get_raw_with_accept(&url, "text/plain").await?;

    // Map version → latest date seen.
    let mut version_map: HashMap<String, String> = HashMap::new();

    for line in text.lines() {
        if let Some(parsed) = parse_manifest_url(line) {
            // Filter out the `stable` and `beta` channel aliases: they are
            // not pinnable version strings.
            if parsed.version == "stable" || parsed.version == "beta" {
                continue;
            }

            // Encode nightly as `nightly-YYYY-MM-DD` for uniqueness.
            let version = if parsed.version == "nightly" {
                format!("nightly-{}", parsed.date)
            } else {
                parsed.version
            };

            // Keep the latest date for each version.
            version_map
                .entry(version)
                .and_modify(|d| {
                    if parsed.date > *d {
                        *d = parsed.date.clone();
                    }
                })
                .or_insert(parsed.date);
        }
    }

    if version_map.is_empty() {
        return Ok(None);
    }

    let releases: Vec<RustVersionRelease> = version_map
        .into_iter()
        .map(|(version, date)| RustVersionRelease {
            version,
            release_timestamp: Some(format!("{date}T00:00:00.000Z")),
        })
        .collect();

    Ok(Some(RustVersionResult {
        releases,
        homepage: "https://rust-lang.org/",
        source_url: "https://github.com/rust-lang/rust",
        changelog_url: "https://github.com/rust-lang/rust/blob/main/RELEASES.md",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "parses nightly URL" — rust-version/parse.spec.ts line 5
    #[test]
    fn parse_nightly_url() {
        let result =
            parse_manifest_url("static.rust-lang.org/dist/2025-11-24/channel-rust-nightly.toml");
        assert_eq!(
            result,
            Some(ParsedManifestUrl {
                date: "2025-11-24".into(),
                version: "nightly".into(),
            })
        );
    }

    // Ported: "parses versioned release URL" — rust-version/parse.spec.ts line 15
    #[test]
    fn parse_versioned_release_url() {
        let result =
            parse_manifest_url("static.rust-lang.org/dist/2024-10-17/channel-rust-1.82.0.toml");
        assert_eq!(
            result,
            Some(ParsedManifestUrl {
                date: "2024-10-17".into(),
                version: "1.82.0".into(),
            })
        );
    }

    // Ported: "parses beta versioned URL" — rust-version/parse.spec.ts line 25
    #[test]
    fn parse_beta_versioned_url() {
        let result = parse_manifest_url(
            "static.rust-lang.org/dist/2025-01-15/channel-rust-1.83.0-beta.5.toml",
        );
        assert_eq!(
            result,
            Some(ParsedManifestUrl {
                date: "2025-01-15".into(),
                version: "1.83.0-beta.5".into(),
            })
        );
    }

    // Ported: "parses stable channel URL" — rust-version/parse.spec.ts line 35
    #[test]
    fn parse_stable_channel_url() {
        let result =
            parse_manifest_url("static.rust-lang.org/dist/2025-11-24/channel-rust-stable.toml");
        assert_eq!(
            result,
            Some(ParsedManifestUrl {
                date: "2025-11-24".into(),
                version: "stable".into(),
            })
        );
    }

    // Ported: "parses beta channel URL" — rust-version/parse.spec.ts line 45
    #[test]
    fn parse_beta_channel_url() {
        let result =
            parse_manifest_url("static.rust-lang.org/dist/2025-11-24/channel-rust-beta.toml");
        assert_eq!(
            result,
            Some(ParsedManifestUrl {
                date: "2025-11-24".into(),
                version: "beta".into(),
            })
        );
    }

    // Ported: "parses URL with https protocol" — rust-version/parse.spec.ts line 55
    #[test]
    fn parse_url_with_https_protocol() {
        let result = parse_manifest_url(
            "https://static.rust-lang.org/dist/2025-11-24/channel-rust-nightly.toml",
        );
        assert_eq!(
            result,
            Some(ParsedManifestUrl {
                date: "2025-11-24".into(),
                version: "nightly".into(),
            })
        );
    }

    // Ported: "parses URL with http protocol" — rust-version/parse.spec.ts line 65
    #[test]
    fn parse_url_with_http_protocol() {
        let result = parse_manifest_url(
            "http://static.rust-lang.org/dist/2025-11-24/channel-rust-nightly.toml",
        );
        assert_eq!(
            result,
            Some(ParsedManifestUrl {
                date: "2025-11-24".into(),
                version: "nightly".into(),
            })
        );
    }

    // Ported: "returns null for URL without date" — rust-version/parse.spec.ts line 75
    #[test]
    fn returns_none_without_date() {
        let result = parse_manifest_url("static.rust-lang.org/dist/channel-rust-nightly.toml");
        assert!(result.is_none());
    }

    // Ported: "returns null for URL without channel-rust pattern" — rust-version/parse.spec.ts line 82
    #[test]
    fn returns_none_without_channel_rust() {
        let result = parse_manifest_url("static.rust-lang.org/dist/2025-11-24/something-else.toml");
        assert!(result.is_none());
    }

    // Ported: "returns null for empty string" — rust-version/parse.spec.ts line 89
    #[test]
    fn returns_none_for_empty_string() {
        assert!(parse_manifest_url("").is_none());
    }

    // Ported: "returns null for malformed date" — rust-version/parse.spec.ts line 94
    // Note: upstream TypeScript parses successfully even for out-of-range dates.
    #[test]
    fn accepts_out_of_range_date() {
        // 2025-13-45 has an invalid month/day but the correct YYYY-MM-DD format,
        // so parseManifestUrl returns Some (matching TypeScript behaviour).
        let result =
            parse_manifest_url("static.rust-lang.org/dist/2025-13-45/channel-rust-nightly.toml");
        assert_eq!(
            result,
            Some(ParsedManifestUrl {
                date: "2025-13-45".into(),
                version: "nightly".into(),
            })
        );
    }

    // Ported: "parses URL with different domain" — rust-version/parse.spec.ts line 104
    #[test]
    fn parse_url_with_different_domain() {
        let result = parse_manifest_url("example.com/archives/2025-11-24/channel-rust-1.82.0.toml");
        assert_eq!(
            result,
            Some(ParsedManifestUrl {
                date: "2025-11-24".into(),
                version: "1.82.0".into(),
            })
        );
    }

    // Ported: "parses URL with complex version" — rust-version/parse.spec.ts line 114
    #[test]
    fn parse_url_with_complex_version() {
        let result =
            parse_manifest_url("static.rust-lang.org/dist/2020-06-18/channel-rust-1.44.1.toml");
        assert_eq!(
            result,
            Some(ParsedManifestUrl {
                date: "2020-06-18".into(),
                version: "1.44.1".into(),
            })
        );
    }

    // Ported: "ignores blank lines silently (no spurious warning)" — rust-version/index.spec.ts line 92
    #[test]
    fn skip_blank_lines() {
        assert!(parse_manifest_url("").is_none());
        assert!(parse_manifest_url("   ").is_none());
    }

    // Ported: "ignores unexpected URLs" — rust-version/index.spec.ts line 69
    #[test]
    fn skip_invalid_url() {
        assert!(parse_manifest_url("static.rust-lang.org/dist/invalid.toml").is_none());
    }

    // Ported: "deduplicates versions with latest date" — rust-version/index.spec.ts line 46
    #[test]
    fn deduplication_keeps_latest_date() {
        let lines = [
            "static.rust-lang.org/dist/2024-10-17/channel-rust-1.82.0.toml",
            "static.rust-lang.org/dist/2024-10-18/channel-rust-1.82.0.toml",
            "static.rust-lang.org/dist/2024-10-19/channel-rust-1.82.0.toml",
        ];

        let mut version_map: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for line in &lines {
            if let Some(parsed) = parse_manifest_url(line) {
                if parsed.version == "stable" || parsed.version == "beta" {
                    continue;
                }
                let version = if parsed.version == "nightly" {
                    format!("nightly-{}", parsed.date)
                } else {
                    parsed.version.clone()
                };
                version_map
                    .entry(version)
                    .and_modify(|d| {
                        if parsed.date > *d {
                            *d = parsed.date.clone();
                        }
                    })
                    .or_insert(parsed.date);
            }
        }

        assert_eq!(version_map.len(), 1);
        assert_eq!(version_map["1.82.0"], "2024-10-19");
    }

    // Verify stable/beta are filtered in the pipeline (index.ts behaviour).
    #[test]
    fn fetch_pipeline_filters_channel_aliases() {
        // stable/beta are excluded from the final release list.
        let inputs = [
            "static.rust-lang.org/dist/2025-11-24/channel-rust-stable.toml",
            "static.rust-lang.org/dist/2025-11-24/channel-rust-beta.toml",
            "static.rust-lang.org/dist/2024-10-17/channel-rust-1.82.0.toml",
        ];

        let mut version_map: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for line in &inputs {
            if let Some(parsed) = parse_manifest_url(line) {
                if parsed.version == "stable" || parsed.version == "beta" {
                    continue;
                }
                let version = if parsed.version == "nightly" {
                    format!("nightly-{}", parsed.date)
                } else {
                    parsed.version.clone()
                };
                version_map.entry(version).or_insert(parsed.date);
            }
        }

        assert_eq!(version_map.len(), 1);
        assert!(version_map.contains_key("1.82.0"));
    }
}
