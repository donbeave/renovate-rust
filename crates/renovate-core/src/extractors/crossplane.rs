//! Crossplane package manifest extractor.
//!
//! Extracts `spec.package` image references from Crossplane `Provider`,
//! `Configuration`, and `Function` custom resources.
//!
//! Renovate reference:
//! - `lib/modules/manager/crossplane/extract.ts`
//! - Default patterns: `[]` (user-configured). We add common `crossplane/` directory.
//! - Datasource: `docker` (OCI packages from `xpkg.upbound.io`)
//!
//! ## Supported resource kinds
//!
//! ```yaml
//! apiVersion: pkg.crossplane.io/v1
//! kind: Provider
//! metadata:
//!   name: provider-aws
//! spec:
//!   package: xpkg.upbound.io/upbound/provider-aws:v0.27.0
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// Skip reason for a Crossplane dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrossplaneSkipReason {
    /// OCI image from `xpkg.upbound.io` — registry not yet supported.
    UnsupportedRegistry,
    /// `spec.package` field is missing or empty.
    MissingPackage,
}

/// A single Crossplane package dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossplaneDep {
    /// Crossplane resource `kind` (e.g. `Provider`, `Configuration`).
    pub kind: String,
    /// Full OCI image reference (e.g. `xpkg.upbound.io/upbound/provider-aws:v0.27.0`).
    pub package: String,
    /// Image tag / version.
    pub current_value: String,
    pub skip_reason: Option<CrossplaneSkipReason>,
}

// ── Regexes ───────────────────────────────────────────────────────────────────

/// Detects Crossplane resources: `apiVersion: pkg.crossplane.io/v*`.
static CROSSPLANE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"apiVersion:\s*['"]?pkg\.crossplane\.io/v"#).unwrap());

/// Extracts `kind: <value>` from a YAML document.
static KIND_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^kind:\s*(\w+)\s*$").unwrap());

/// Extracts `package: <image_ref>` from `spec:` block.
static PACKAGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?m)^\s+package:\s*['"]?([^\s'"]+)['"]?\s*$"#).unwrap());

// ── Parsing ───────────────────────────────────────────────────────────────────

/// Extract Crossplane package dependencies from a manifest file.
///
/// Returns an empty Vec if the file contains no `pkg.crossplane.io` resources.
pub fn extract(content: &str) -> Vec<CrossplaneDep> {
    if !CROSSPLANE_RE.is_match(content) {
        return Vec::new();
    }

    let mut deps = Vec::new();

    // Split on `---` to handle multi-document YAML.
    for doc in content.split("\n---") {
        if !CROSSPLANE_RE.is_match(doc) {
            continue;
        }

        let kind = KIND_RE
            .captures(doc)
            .map(|c| c[1].to_owned())
            .unwrap_or_else(|| "Unknown".to_owned());

        let package_cap = PACKAGE_RE.captures(doc);
        let package = package_cap.map(|c| c[1].to_owned());

        match package {
            None => {
                deps.push(CrossplaneDep {
                    kind,
                    package: String::new(),
                    current_value: String::new(),
                    skip_reason: Some(CrossplaneSkipReason::MissingPackage),
                });
            }
            Some(pkg) if pkg.is_empty() => {
                deps.push(CrossplaneDep {
                    kind,
                    package: String::new(),
                    current_value: String::new(),
                    skip_reason: Some(CrossplaneSkipReason::MissingPackage),
                });
            }
            Some(pkg) => {
                let (image_name, tag) = split_image_tag(&pkg);
                let skip_reason = if is_upbound_registry(image_name) {
                    Some(CrossplaneSkipReason::UnsupportedRegistry)
                } else {
                    None
                };
                deps.push(CrossplaneDep {
                    kind,
                    package: pkg.clone(),
                    current_value: tag.to_owned(),
                    skip_reason,
                });
            }
        }
    }

    deps
}

fn split_image_tag(s: &str) -> (&str, &str) {
    if let Some(pos) = s.rfind(':') {
        let tag = &s[pos + 1..];
        if !tag.contains('/') {
            return (&s[..pos], tag);
        }
    }
    (s, "")
}

fn is_upbound_registry(image: &str) -> bool {
    image.starts_with("xpkg.upbound.io/") || image.starts_with("index.docker.io/")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "full test" — crossplane/extract.spec.ts line 94
    #[test]
    fn extracts_provider() {
        let content = r#"
apiVersion: pkg.crossplane.io/v1
kind: Provider
metadata:
  name: provider-aws
spec:
  package: xpkg.upbound.io/upbound/provider-aws:v0.27.0
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.kind, "Provider");
        assert_eq!(d.current_value, "v0.27.0");
        assert_eq!(
            d.skip_reason,
            Some(CrossplaneSkipReason::UnsupportedRegistry)
        );
    }

    // Ported: "return null for kubernetes manifest" — crossplane/extract.spec.ts line 20
    #[test]
    fn skips_non_crossplane_files() {
        let content = "apiVersion: v1\nkind: ConfigMap\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "should work even if there are other resources in the file" — crossplane/extract.spec.ts line 137
    #[test]
    fn handles_multi_document() {
        let content = r#"
apiVersion: pkg.crossplane.io/v1
kind: Provider
spec:
  package: xpkg.upbound.io/upbound/provider-aws:v0.27.0
---
apiVersion: pkg.crossplane.io/v1
kind: Configuration
spec:
  package: xpkg.upbound.io/upbound/platform-ref-aws:v0.9.0
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].kind, "Provider");
        assert_eq!(deps[1].kind, "Configuration");
    }

    // Ported: "return no results for invalid resource" — crossplane/extract.spec.ts line 79
    #[test]
    fn reports_missing_package() {
        let content = r#"
apiVersion: pkg.crossplane.io/v1
kind: Provider
metadata:
  name: provider-aws
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(CrossplaneSkipReason::MissingPackage)
        );
    }

    // Ported: "returns null for empty" — crossplane/extract.spec.ts line 12
    #[test]
    fn empty_content_returns_empty() {
        assert!(extract("nothing here").is_empty());
    }

    // Ported: "return result for double quoted pkg.crossplane.io apiVersion reference" — crossplane/extract.spec.ts line 37
    #[test]
    fn double_quoted_api_version_extracted() {
        let content = r#"apiVersion: "pkg.crossplane.io/v1"
kind: Configuration
spec:
  package: "xpkg.upbound.io/upbound/platform-ref-aws:v0.6.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "v0.6.0");
    }

    // Ported: "return result for single quoted pkg.crossplane.io apiVersion reference" — crossplane/extract.spec.ts line 58
    #[test]
    fn single_quoted_api_version_extracted() {
        let content = "apiVersion: 'pkg.crossplane.io/v1'\nkind: Configuration\nspec:\n  package: 'xpkg.upbound.io/upbound/platform-ref-aws:v0.6.0'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "v0.6.0");
    }
}
