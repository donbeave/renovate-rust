//! OpenTelemetry Collector Builder (`ocb`) YAML extractor.
//!
//! Extracts Go module dependencies from OCB builder config files.
//!
//! Renovate reference:
//! - `lib/modules/manager/ocb/extract.ts`
//! - Default patterns: `[]` (user-configured). We add `otelcol-builder.yaml` convention.
//! - Datasource: `go` (Go module proxy)
//!
//! ## File format
//!
//! ```yaml
//! dist:
//!   otelcol_version: 0.40.0
//!   version: 1.0.0
//! exporters:
//!   - gomod: github.com/org/repo/exporter v0.86.0
//! receivers:
//!   - gomod: go.opentelemetry.io/collector/receiver/otlpreceiver v0.86.0
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// Component section names that contain `gomod` entries.
const MODULE_SECTIONS: &[&str] = &[
    "connectors",
    "exporters",
    "extensions",
    "processors",
    "providers",
    "receivers",
];

/// Dep type for the collector itself.
const COLLECTOR_DEP_TYPE: &str = "collector";
/// Canonical collector module path.
const COLLECTOR_MODULE: &str = "go.opentelemetry.io/collector";

/// A single OCB dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OcbDep {
    /// Go module path.
    pub dep_name: String,
    /// Version string (e.g. `v0.86.0` or bare `0.40.0`).
    pub current_value: String,
    /// Component type (`collector`, `exporters`, `extensions`, etc.).
    pub dep_type: String,
    /// When set the gomod string had no version.
    pub skip_reason: Option<OcbSkipReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OcbSkipReason {
    /// `gomod:` entry had no version field.
    MissingVersion,
}

// ── Regexes ───────────────────────────────────────────────────────────────────

static SECTION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([a-z_]+):\s*$").unwrap());

static OTELCOL_VERSION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s+otelcol_version:\s+(\S+)").unwrap());

static GOMOD_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s+-\s+gomod:\s+(\S+)(?:\s+(\S+))?").unwrap());

// ── Public API ────────────────────────────────────────────────────────────────

/// Extract OCB Go module deps from a builder YAML config.
pub fn extract(content: &str) -> Vec<OcbDep> {
    if !content.contains("gomod:") && !content.contains("otelcol_version:") {
        return Vec::new();
    }

    let mut deps = Vec::new();
    let mut current_section: Option<&'static str> = None;
    let mut in_dist = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Detect top-level section.
        if let Some(cap) = SECTION_RE.captures(line) {
            let sec = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            in_dist = sec == "dist";
            current_section = MODULE_SECTIONS.iter().find(|&&s| s == sec).copied();
            continue;
        }

        // Inside `dist:` look for otelcol_version.
        if in_dist {
            if let Some(cap) = OTELCOL_VERSION_RE.captures(line) {
                deps.push(OcbDep {
                    dep_name: COLLECTOR_MODULE.to_owned(),
                    current_value: cap[1].to_owned(),
                    dep_type: COLLECTOR_DEP_TYPE.to_owned(),
                    skip_reason: None,
                });
            }
            continue;
        }

        // Inside a module section look for `- gomod: module version`.
        if let Some(section) = current_section
            && let Some(cap) = GOMOD_RE.captures(line)
        {
            let module_path = cap[1].to_owned();
            let version = cap.get(2).map(|m| m.as_str().to_owned());
            deps.push(OcbDep {
                dep_name: module_path,
                current_value: version.clone().unwrap_or_default(),
                dep_type: section.to_owned(),
                skip_reason: if version.is_none() {
                    Some(OcbSkipReason::MissingVersion)
                } else {
                    None
                },
            });
        }
    }

    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL_EXAMPLE: &str = r#"
dist:
  name: otelcol-custom
  description: Local OpenTelemetry Collector binary
  module: github.com/open-telemetry/opentelemetry-collector
  otelcol_version: 0.40.0
  version: 1.0.0
  output_path: /tmp/dist
exporters:
  - gomod: github.com/open-telemetry/opentelemetry-collector-contrib/exporter/alibabacloudlogserviceexporter v0.86.0
  - gomod: go.opentelemetry.io/collector/exporter/debugexporter v0.86.0

extensions:
  - gomod: github.com/open-telemetry/opentelemetry-collector-contrib/extension/healthcheckextension v0.86.0

receivers:
  - gomod: go.opentelemetry.io/collector/receiver/otlpreceiver v0.86.0

processors:
  - gomod: go.opentelemetry.io/collector/processor/batchprocessor v0.86.0

providers:
  - gomod: go.opentelemetry.io/collector/confmap/provider/envprovider v1.0.0-rcv0015
"#;

    #[test]
    fn extracts_full_example() {
        let deps = extract(FULL_EXAMPLE);
        // collector + 2 exporters + 1 extension + 1 receiver + 1 processor + 1 provider = 7
        assert_eq!(deps.len(), 7);

        let collector = &deps[0];
        assert_eq!(collector.dep_name, "go.opentelemetry.io/collector");
        assert_eq!(collector.current_value, "0.40.0");
        assert_eq!(collector.dep_type, "collector");
        assert!(collector.skip_reason.is_none());

        let exporter = &deps[1];
        assert_eq!(
            exporter.dep_name,
            "github.com/open-telemetry/opentelemetry-collector-contrib/exporter/alibabacloudlogserviceexporter"
        );
        assert_eq!(exporter.current_value, "v0.86.0");
        assert_eq!(exporter.dep_type, "exporters");
    }

    #[test]
    fn extracts_module_sections() {
        let deps = extract(FULL_EXAMPLE);
        let types: Vec<&str> = deps.iter().map(|d| d.dep_type.as_str()).collect();
        assert!(types.contains(&"collector"));
        assert!(types.contains(&"exporters"));
        assert!(types.contains(&"extensions"));
        assert!(types.contains(&"receivers"));
        assert!(types.contains(&"processors"));
        assert!(types.contains(&"providers"));
    }

    #[test]
    fn skips_unknown_content() {
        assert!(extract("foo: bar\nbaz: qux\n").is_empty());
    }

    #[test]
    fn handles_missing_version() {
        let content = "receivers:\n  - gomod: go.opentelemetry.io/collector/receiver/foo\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(OcbSkipReason::MissingVersion));
    }
}
