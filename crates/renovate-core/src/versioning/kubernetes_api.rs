//! Kubernetes API versioning.
//!
//! Ports `lib/modules/versioning/kubernetes-api/index.ts` which extends
//! `RegExpVersioningApi` with regex:
//! `^(?:(?<compatibility>\\S+)/)?v(?<major>\\d+)(?<prerelease>(?:alpha|beta)\\d+)?$`
//!
//! Compatibility prefix (e.g. `apps/`, `autoscaling/`) is parsed but ignored
//! for all comparison operations.

use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Prerelease {
    Alpha(u32),
    Beta(u32),
    Stable,
}

impl Prerelease {
    fn order(&self) -> u32 {
        match self {
            Prerelease::Alpha(_) => 0,
            Prerelease::Beta(_) => 1,
            Prerelease::Stable => 2,
        }
    }
}

impl PartialOrd for Prerelease {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Prerelease {
    fn cmp(&self, other: &Self) -> Ordering {
        let lo = self.order();
        let ro = other.order();
        if lo != ro {
            return lo.cmp(&ro);
        }
        match (self, other) {
            (Prerelease::Alpha(a), Prerelease::Alpha(b)) => a.cmp(b),
            (Prerelease::Beta(a), Prerelease::Beta(b)) => a.cmp(b),
            _ => Ordering::Equal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KubeVersion {
    major: u32,
    prerelease: Prerelease,
}

impl PartialOrd for KubeVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for KubeVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then_with(|| self.prerelease.cmp(&other.prerelease))
    }
}

fn parse(version: &str) -> Option<KubeVersion> {
    // Strip optional compatibility prefix: anything before the last '/'
    let s = if let Some(pos) = version.rfind('/') {
        &version[pos + 1..]
    } else {
        version
    };

    let s = s.strip_prefix('v')?;

    // Split into numeric prefix and optional prerelease suffix.
    let alpha_pos = s.find("alpha");
    let beta_pos = s.find("beta");

    if let Some(pos) = alpha_pos {
        let major: u32 = s[..pos].parse().ok()?;
        let num: u32 = s[pos + 5..].parse().ok()?;
        Some(KubeVersion { major, prerelease: Prerelease::Alpha(num) })
    } else if let Some(pos) = beta_pos {
        let major: u32 = s[..pos].parse().ok()?;
        let num: u32 = s[pos + 4..].parse().ok()?;
        Some(KubeVersion { major, prerelease: Prerelease::Beta(num) })
    } else {
        let major: u32 = s.parse().ok()?;
        Some(KubeVersion { major, prerelease: Prerelease::Stable })
    }
}

pub fn is_valid(version: &str) -> bool {
    parse(version).is_some()
}

pub fn is_stable(version: &str) -> bool {
    parse(version).is_some_and(|v| v.prerelease == Prerelease::Stable)
}

pub fn get_major(version: &str) -> Option<i64> {
    parse(version).map(|v| v.major as i64)
}

pub fn get_minor(_version: &str) -> Option<i64> {
    if is_valid(_version) { Some(0) } else { None }
}

pub fn get_patch(_version: &str) -> Option<i64> {
    if is_valid(_version) { Some(0) } else { None }
}

pub fn equals(a: &str, b: &str) -> bool {
    match (parse(a), parse(b)) {
        (Some(va), Some(vb)) => va == vb,
        _ => false,
    }
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    match (parse(a), parse(b)) {
        (Some(va), Some(vb)) => va > vb,
        _ => false,
    }
}

pub fn sort_versions(a: &str, b: &str) -> Ordering {
    match (parse(a), parse(b)) {
        (Some(va), Some(vb)) => va.cmp(&vb),
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        _ => Ordering::Equal,
    }
}

pub fn matches(version: &str, range: &str) -> bool {
    equals(version, range)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isStable("$version") === $expected" — kubernetes-api/index.spec.ts line 6
    #[test]
    fn is_stable_matches_renovate_kubernetes_api_index_spec() {
        assert!(is_stable("v1"));
        assert!(is_stable("v2"));
        assert!(!is_stable("v1alpha1"));
        assert!(!is_stable("v1beta11"));
    }

    // Ported: "isValid("$version") === $expected" — kubernetes-api/index.spec.ts line 16
    #[test]
    fn is_valid_matches_renovate_kubernetes_api_index_spec() {
        assert!(is_valid("v1"));
        assert!(is_valid("v2"));
        assert!(is_valid("v10"));
        assert!(!is_valid("v1.0"));
        assert!(!is_valid("1"));
        assert!(!is_valid("2"));
        assert!(!is_valid("10"));
        assert!(!is_valid("1.0"));
        assert!(is_valid("apps/v1"));
        assert!(is_valid("telemetry.istio.io/v1alpha1"));
        assert!(is_valid("k3d.io/v1alpha2"));
        assert!(is_valid("extensions/v1beta1"));
        assert!(is_valid("apps/v1beta2"));
        assert!(is_valid("autoscaling/v2"));
        assert!(is_valid("acme.cert-manager.io/v1"));
    }

    // Ported: "getMajor, getMinor, getPatch for "$version"" — kubernetes-api/index.spec.ts line 37
    #[test]
    fn get_major_minor_patch_matches_renovate_kubernetes_api_index_spec() {
        let cases: &[(&str, i64, i64, i64)] = &[
            ("v1",      1, 0, 0),
            ("v2",      2, 0, 0),
            ("v1alpha1",1, 0, 0),
            ("v1alpha2",1, 0, 0),
            ("v1beta1", 1, 0, 0),
            ("v1beta2", 1, 0, 0),
        ];
        for (v, major, minor, patch) in cases {
            assert_eq!(get_major(v), Some(*major), "getMajor({v})");
            assert_eq!(get_minor(v), Some(*minor), "getMinor({v})");
            assert_eq!(get_patch(v), Some(*patch), "getPatch({v})");
        }
    }

    // Ported: "equals("$version", "$other") === $expected" — kubernetes-api/index.spec.ts line 54
    #[test]
    fn equals_matches_renovate_kubernetes_api_index_spec() {
        assert!(equals("v1", "v1"));
        assert!(!equals("v1", "v2"));
        assert!(!equals("v1", "v1alpha1"));
        assert!(!equals("v1", "v1alpha2"));
        assert!(!equals("v1", "v1beta1"));
        assert!(!equals("v1", "v1beta2"));
        assert!(equals("v1alpha1", "v1alpha1"));
        assert!(!equals("v1alpha1", "v1alpha2"));
        assert!(!equals("v1alpha1", "v1beta1"));
        assert!(!equals("v1alpha1", "v1beta2"));
        assert!(equals("apps/v1", "apps/v1"));
        assert!(!equals("apps/v1", "apps/v2"));
        assert!(!equals("apps/v1", "apps/v1alpha1"));
        assert!(!equals("apps/v1", "apps/v1beta1"));
        assert!(equals("apps/v1", "autoscaling/v1"));
        assert!(!equals("apps/v1", "autoscaling/v2"));
        assert!(!equals("apps/v1", "autoscaling/v1alpha1"));
        assert!(!equals("apps/v1", "autoscaling/v1beta1"));
    }

    // Ported: "matches("$version", "$other") === $expected" — kubernetes-api/index.spec.ts line 81
    #[test]
    fn matches_matches_renovate_kubernetes_api_index_spec() {
        assert!(matches("v1", "v1"));
        assert!(!matches("v1", "v2"));
        assert!(!matches("v1", "v1alpha1"));
        assert!(!matches("v1", "v1alpha2"));
        assert!(!matches("v1", "v1beta1"));
        assert!(!matches("v1", "v1beta2"));
        assert!(matches("v1alpha1", "v1alpha1"));
        assert!(!matches("v1alpha1", "v1alpha2"));
        assert!(!matches("v1alpha1", "v1beta1"));
        assert!(!matches("v1alpha1", "v1beta2"));
    }

    // Ported: "isGreaterThan("$version", "$other") === $expected" — kubernetes-api/index.spec.ts line 100
    #[test]
    fn is_greater_than_matches_renovate_kubernetes_api_index_spec() {
        assert!(!is_greater_than("v1", "v2"));
        assert!(is_greater_than("v1", "v1alpha1"));
        assert!(is_greater_than("v1", "v1beta1"));
        assert!(is_greater_than("v2", "v1beta1"));
        assert!(!is_greater_than("v1alpha1", "v1alpha2"));
        assert!(is_greater_than("v1beta1", "v1alpha1"));
        assert!(is_greater_than("v1beta2", "v1beta1"));
    }

    // Ported: "sorts versions in an ascending order" — kubernetes-api/index.spec.ts line 116
    #[test]
    fn sort_versions_matches_renovate_kubernetes_api_index_spec() {
        let mut versions = vec![
            "v10", "v2", "v2beta2", "v2beta1", "v2alpha2", "v2alpha1",
            "v1", "v1beta2", "v1beta1", "v1alpha2", "v1alpha1",
        ];
        versions.sort_by(|a, b| sort_versions(a, b));
        assert_eq!(versions, vec![
            "v1alpha1", "v1alpha2", "v1beta1", "v1beta2", "v1",
            "v2alpha1", "v2alpha2", "v2beta1", "v2beta2", "v2", "v10",
        ]);
    }
}
