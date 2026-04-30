//! Glasskube `ClusterPackage`/`Package` manifest extractor.
//!
//! Extracts package name and version from Glasskube Kubernetes custom resources
//! for version tracking via the Glasskube packages registry.
//!
//! Renovate reference:
//! - `lib/modules/manager/glasskube/extract.ts`
//! - Default patterns: `[]` (user-configured). We add `glasskube/` convention.
//! - Datasource: `glasskube-packages`
//!
//! ## Supported resource kinds
//!
//! ```yaml
//! apiVersion: glasskube.dev/v1alpha1
//! kind: ClusterPackage
//! metadata:
//!   name: cert-manager
//! spec:
//!   packageInfo:
//!     name: cert-manager
//!     version: v1.14.2
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// A single Glasskube package dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlasskubeDep {
    /// Package name (e.g. `cert-manager`).
    pub package_name: String,
    /// Version (e.g. `v1.14.2`).
    pub current_value: String,
    /// Resource kind (`ClusterPackage`, `Package`).
    pub kind: String,
}

// ── Regexes ───────────────────────────────────────────────────────────────────

/// Detects Glasskube resources: `apiVersion: glasskube.dev/`.
static GLASSKUBE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"apiVersion:\s*glasskube\.dev/").unwrap());

/// `kind: ClusterPackage` or `kind: Package`.
static KIND_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^kind:\s*(ClusterPackage|Package)\s*$").unwrap());

/// `name:` inside `packageInfo:` section (indent ≥ 4).
static PKG_NAME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?m)^\s{4,}name:\s*['"]?([^\s'"]+)['"]?\s*$"#).unwrap());

/// `version:` inside `packageInfo:` section (indent ≥ 4).
static PKG_VERSION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?m)^\s{4,}version:\s*['"]?([^\s'"]+)['"]?\s*$"#).unwrap());

// ── Public API ────────────────────────────────────────────────────────────────

/// Extract Glasskube package dependencies from a manifest file.
pub fn extract(content: &str) -> Vec<GlasskubeDep> {
    if !GLASSKUBE_RE.is_match(content) {
        return Vec::new();
    }

    let mut deps = Vec::new();

    for doc in content.split("\n---") {
        if !GLASSKUBE_RE.is_match(doc) {
            continue;
        }

        let kind = match KIND_RE.captures(doc) {
            Some(c) => c[1].to_owned(),
            None => continue,
        };

        // Find `packageInfo:` section boundary, then extract name/version.
        let Some(pkg_info_start) = doc.find("packageInfo:") else {
            continue;
        };
        let pkg_section = &doc[pkg_info_start..];

        let package_name = PKG_NAME_RE
            .captures(pkg_section)
            .map(|c| c[1].to_owned())
            .unwrap_or_default();

        let current_value = PKG_VERSION_RE
            .captures(pkg_section)
            .map(|c| c[1].to_owned())
            .unwrap_or_default();

        if package_name.is_empty() || current_value.is_empty() {
            continue;
        }

        deps.push(GlasskubeDep {
            package_name,
            current_value,
            kind,
        });
    }

    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should extract version and registryUrl" — glasskube/extract.spec.ts line 43
    #[test]
    fn extracts_cluster_package() {
        let content = r#"
apiVersion: glasskube.dev/v1alpha1
kind: ClusterPackage
metadata:
  name: cert-manager
spec:
  packageInfo:
    name: cert-manager
    version: v1.14.2
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].package_name, "cert-manager");
        assert_eq!(deps[0].current_value, "v1.14.2");
        assert_eq!(deps[0].kind, "ClusterPackage");
    }

    // Ported: "should skip package with non-existing repo" — glasskube/extract.spec.ts line 67
    #[test]
    fn skips_non_glasskube_files() {
        assert!(extract("apiVersion: v1\nkind: ConfigMap\n").is_empty());
    }

    // Ported: "should extract version and registryUrl" — glasskube/extract.spec.ts line 43
    #[test]
    fn extracts_multiple_packages() {
        let content = r#"
apiVersion: glasskube.dev/v1alpha1
kind: ClusterPackage
spec:
  packageInfo:
    name: ingress-nginx
    version: v4.9.1
---
apiVersion: glasskube.dev/v1alpha1
kind: ClusterPackage
spec:
  packageInfo:
    name: cert-manager
    version: v1.14.2
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
    }
}
