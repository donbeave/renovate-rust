//! FluxCD `gotk-components.yaml` system manifest extractor.
//!
//! Reads the Flux version from the standard comment header that Flux
//! injects into all generated `gotk-components.yaml` manifests, and
//! returns a single dep for the `fluxcd/flux2` GitHub release.
//!
//! Renovate reference:
//! - `lib/modules/manager/flux/common.ts` — `systemManifestHeaderRegex`
//! - `lib/modules/manager/flux/extract.ts` — `extractSystemManifest`
//! - Pattern: `/(^|/)gotk-components\.ya?ml$/`
//! - Datasource: GitHub Releases (`fluxcd/flux2`)
//!
//! ## Header format
//!
//! ```yaml
//! # Flux Version: v2.2.3
//! # Components: source-controller,kustomize-controller,...
//! ```

use std::sync::LazyLock;

use regex::Regex;

pub const FLUX2_REPO: &str = "fluxcd/flux2";

/// A single extracted FluxCD system-manifest dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FluxSystemDep {
    /// Flux version (e.g. `"v2.2.3"`).
    pub version: String,
    /// Raw components string (e.g. `"source-controller,kustomize-controller"`).
    pub components: Option<String>,
}

static HEADER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"#\s*Flux\s+Version:\s*(\S+)(?:\s*#\s*Components:\s*([A-Za-z,\-]+))?").unwrap()
});

/// Extract the Flux version from a `gotk-components.yaml` file.
///
/// Returns `None` when no valid Flux Version header is found.
pub fn extract(content: &str) -> Option<FluxSystemDep> {
    // Apply against whole content so the optional Components clause can span
    // the line boundary between `# Flux Version:` and `# Components:`.
    let cap = HEADER_RE.captures(content)?;
    Some(FluxSystemDep {
        version: cap[1].to_owned(),
        components: cap.get(2).map(|m| m.as_str().to_owned()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts version and components from system manifest at $filepath" — flux/extract.spec.ts line 72
    #[test]
    fn extracts_version_with_components() {
        let content = "# Flux Version: v2.2.3\n# Components: source-controller,kustomize-controller,helm-controller\napiVersion: v1\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "v2.2.3");
        assert_eq!(
            dep.components.as_deref(),
            Some("source-controller,kustomize-controller,helm-controller")
        );
    }

    // Ported: "considers components optional in system manifests" — flux/extract.spec.ts line 102
    #[test]
    fn extracts_version_without_components() {
        let content = "# Flux Version: v2.1.0\napiVersion: v1\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "v2.1.0");
        assert!(dep.components.is_none());
    }

    #[test]
    fn version_in_middle_of_file() {
        let content = "---\napiVersion: v1\n# Flux Version: v2.0.1\nkind: Namespace\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "v2.0.1");
    }

    // Ported: "ignores system manifests without a version" — flux/extract.spec.ts line 111
    #[test]
    fn no_header_returns_none() {
        let content = "apiVersion: v1\nkind: Namespace\nmetadata:\n  name: flux-system\n";
        assert!(extract(content).is_none());
    }

    #[test]
    fn empty_returns_none() {
        assert!(extract("").is_none());
    }
}
