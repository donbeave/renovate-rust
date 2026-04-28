//! TFLint plugin (`.tflint.hcl`) dependency extractor.
//!
//! Parses TFLint plugin blocks and extracts GitHub Releases deps.
//!
//! Renovate reference:
//! - `lib/modules/manager/tflint-plugin/extract.ts`
//! - Pattern: `/\.tflint\.hcl$/`
//! - Datasource: GitHub Releases
//!
//! ## File format
//!
//! ```hcl
//! plugin "aws" {
//!   enabled = true
//!   version = "0.21.0"
//!   source  = "github.com/terraform-linters/tflint-ruleset-aws"
//! }
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// Why a TFLint plugin dep is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TflintSkipReason {
    /// Source is not a recognized GitHub URL.
    UnsupportedDatasource,
    /// No version is specified in the plugin block.
    UnspecifiedVersion,
    /// No source URL in the plugin block.
    MissingSource,
}

/// A single extracted TFLint plugin dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TflintPluginDep {
    /// The plugin name from the block header.
    pub name: String,
    /// `owner/repo` path extracted from the github.com source URL.
    pub dep_name: String,
    /// Version string.
    pub current_value: String,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<TflintSkipReason>,
}

/// Matches `plugin "name" {` — starts a plugin block.
static PLUGIN_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s*plugin\s+"([^"]+)"\s+\{"#).unwrap());

/// Matches `key = "value"` or `key = value` inside an HCL block.
static KV: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"^\s*(\w+)\s*=\s*"?([^"#\s]+)"?\s*$"##).unwrap());

/// Extract TFLint plugin deps from a `.tflint.hcl` file.
pub fn extract(content: &str) -> Vec<TflintPluginDep> {
    let mut deps = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        if let Some(cap) = PLUGIN_BLOCK.captures(lines[i]) {
            let plugin_name = cap[1].to_owned();

            // Parse the block by counting braces.
            let mut source: Option<String> = None;
            let mut version: Option<String> = None;
            let mut brace_count: i32 = 0;
            let start = i;

            while i < lines.len() {
                let line = lines[i];
                let open = line.chars().filter(|&c| c == '{').count() as i32;
                let close = line.chars().filter(|&c| c == '}').count() as i32;
                brace_count += open - close;

                if i > start
                    && brace_count == 1
                    && let Some(kv) = KV.captures(line)
                {
                    match &kv[1] {
                        "version" => version = Some(kv[2].to_owned()),
                        "source" => source = Some(kv[2].to_owned()),
                        _ => {}
                    }
                }

                if brace_count == 0 {
                    break;
                }
                i += 1;
            }

            deps.push(build_dep(plugin_name, source, version));
        }
        i += 1;
    }

    deps
}

fn build_dep(name: String, source: Option<String>, version: Option<String>) -> TflintPluginDep {
    let Some(src) = source else {
        return TflintPluginDep {
            name,
            dep_name: String::new(),
            current_value: String::new(),
            skip_reason: Some(TflintSkipReason::MissingSource),
        };
    };

    let parts: Vec<&str> = src.splitn(3, '/').collect();
    if parts.first() != Some(&"github.com") || parts.len() < 3 {
        return TflintPluginDep {
            name,
            dep_name: src,
            current_value: String::new(),
            skip_reason: Some(TflintSkipReason::UnsupportedDatasource),
        };
    }

    let dep_name = parts[1..].join("/");
    let Some(current_value) = version else {
        return TflintPluginDep {
            name,
            dep_name,
            current_value: String::new(),
            skip_reason: Some(TflintSkipReason::UnspecifiedVersion),
        };
    };

    TflintPluginDep {
        name,
        dep_name,
        current_value,
        skip_reason: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_github_plugin() {
        let content = r#"
plugin "aws" {
  enabled = true
  version = "0.21.0"
  source  = "github.com/terraform-linters/tflint-ruleset-aws"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.name, "aws");
        assert_eq!(d.dep_name, "terraform-linters/tflint-ruleset-aws");
        assert_eq!(d.current_value, "0.21.0");
        assert!(d.skip_reason.is_none());
    }

    #[test]
    fn extracts_multiple_plugins() {
        let content = r#"
plugin "aws" {
  enabled = true
  version = "0.21.0"
  source  = "github.com/terraform-linters/tflint-ruleset-aws"
}

plugin "google" {
  enabled = true
  version = "0.20.0"
  source  = "github.com/terraform-linters/tflint-ruleset-google"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_name, "terraform-linters/tflint-ruleset-aws");
        assert_eq!(deps[1].dep_name, "terraform-linters/tflint-ruleset-google");
    }

    #[test]
    fn non_github_source_skipped() {
        let content = r#"
plugin "custom" {
  version = "1.0.0"
  source  = "registry.example.com/myorg/myplugin"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(TflintSkipReason::UnsupportedDatasource)
        );
    }

    #[test]
    fn missing_version_sets_skip_reason() {
        let content = r#"
plugin "aws" {
  enabled = true
  source  = "github.com/terraform-linters/tflint-ruleset-aws"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(TflintSkipReason::UnspecifiedVersion)
        );
    }

    #[test]
    fn no_plugins_returns_empty() {
        let content = r#"
config {
  module = true
}
"#;
        assert!(extract(content).is_empty());
    }
}
