//! @parity lib/modules/versioning/generic.ts full
//! @parity lib/modules/versioning/regex/index.ts full
//! Regex-based versioning.
//!
//! Ports `lib/modules/versioning/regex/index.ts` and
//! `lib/modules/versioning/generic.ts`.
//!
//! A versioner is configured with a regex pattern that contains named capture
//! groups: `major`, `minor`, `patch`, `prerelease`, `compatibility`,
//! `build`, `revision`. At least one of `major`/`minor`/`patch` is required.

use std::cmp::Ordering;

use regex::Regex;

// ── Parsed version ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct ParsedVersion {
    release: Vec<u64>,
    prerelease: Option<String>,
    compatibility: Option<String>,
}

// ── Versioner ─────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct RegexVersioning {
    re: Regex,
}

impl RegexVersioning {
    /// Create a new `RegexVersioning` from the given pattern.
    ///
    /// Returns `Err` when:
    /// - the pattern does not contain at least one of `<major>`, `<minor>`,
    ///   or `<patch>` named groups, OR
    /// - the regex fails to compile.
    pub fn new(pattern: &str) -> Result<Self, String> {
        if !pattern.contains("<major>")
            && !pattern.contains("<minor>")
            && !pattern.contains("<patch>")
        {
            return Err(
                "regex versioning needs at least one major, minor or patch group defined"
                    .to_owned(),
            );
        }
        let re = Regex::new(pattern).map_err(|e| e.to_string())?;
        Ok(RegexVersioning { re })
    }

    /// Parse `'regex:PATTERN'` or `'regex'` into a versioner.
    ///
    /// `'regex'` uses the default pattern `^(?<major>\\d+)?$`.
    pub fn from_config(config: &str) -> Result<Self, String> {
        let pattern = if let Some(p) = config.strip_prefix("regex:") {
            if p.is_empty() { r"^(?<major>\d+)?$" } else { p }
        } else if config == "regex" {
            r"^(?<major>\d+)?$"
        } else {
            return Err(format!("invalid regex versioning config: {config}"));
        };
        Self::new(pattern)
    }

    fn parse(&self, version: &str) -> Option<ParsedVersion> {
        let caps = self.re.captures(version)?;

        let parse_num = |name: &str| -> u64 {
            caps.name(name)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0)
        };

        let major = parse_num("major");
        let minor = parse_num("minor");
        let patch = parse_num("patch");
        let mut release = vec![major, minor, patch];

        // Optional build and revision fields (4th and 5th slots).
        if let Some(build) = caps
            .name("build")
            .and_then(|m| m.as_str().parse::<u64>().ok())
        {
            release.push(build);
            if let Some(rev) = caps
                .name("revision")
                .and_then(|m| m.as_str().parse::<u64>().ok())
            {
                release.push(rev);
            }
        }

        let prerelease = caps
            .name("prerelease")
            .map(|m| m.as_str().to_owned())
            .filter(|s| !s.is_empty());

        let compatibility = caps.name("compatibility").map(|m| m.as_str().to_owned());

        Some(ParsedVersion {
            release,
            prerelease,
            compatibility,
        })
    }

    fn compare(&self, a: &str, b: &str) -> Ordering {
        let Some(left) = self.parse(a) else {
            return Ordering::Greater;
        };
        let Some(right) = self.parse(b) else {
            return Ordering::Greater;
        };

        // Compare release arrays element-by-element; default to 0.
        let len = left.release.len().max(right.release.len());
        for i in 0..len {
            let lv = left.release.get(i).copied().unwrap_or(0);
            let rv = right.release.get(i).copied().unwrap_or(0);
            if lv != rv {
                return lv.cmp(&rv);
            }
        }

        // Compare prerelease (pre-release < no-prerelease).
        match (&left.prerelease, &right.prerelease) {
            (Some(lp), Some(rp)) => {
                let cmp = numeric_aware_cmp(lp, rp);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            (Some(_), None) => return Ordering::Less,
            (None, Some(_)) => return Ordering::Greater,
            (None, None) => {}
        }

        Ordering::Equal
    }

    /// Returns the numeric comparison value like TypeScript's `_compare()`.
    ///
    /// Returns the actual difference of the first differing release component
    /// (e.g. -2 for major 1 vs 3), or -1/+1 for prerelease ordering, or 0 for
    /// equal versions. Mirrors the return value of `GenericVersioningApi._compare`.
    fn compare_numeric(&self, a: &str, b: &str) -> i32 {
        let Some(left) = self.parse(a) else {
            return 1;
        };
        let Some(right) = self.parse(b) else {
            return 1;
        };

        let len = left.release.len().max(right.release.len());
        for i in 0..len {
            let lv = left.release.get(i).copied().unwrap_or(0) as i64;
            let rv = right.release.get(i).copied().unwrap_or(0) as i64;
            if lv != rv {
                let diff = lv - rv;
                return if diff > i32::MAX as i64 {
                    i32::MAX
                } else if diff < i32::MIN as i64 {
                    i32::MIN
                } else {
                    diff as i32
                };
            }
        }

        match (&left.prerelease, &right.prerelease) {
            (Some(lp), Some(rp)) => match numeric_aware_cmp(lp, rp) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            },
            (Some(_), None) => -1,
            (None, Some(_)) => 1,
            (None, None) => 0,
        }
    }

    // ── Public methods ────────────────────────────────────────────────────────

    pub fn is_valid(&self, version: &str) -> bool {
        self.parse(version).is_some()
    }

    pub fn is_stable(&self, version: &str) -> bool {
        self.parse(version).is_some_and(|v| v.prerelease.is_none())
    }

    pub fn get_major(&self, version: &str) -> Option<i64> {
        self.parse(version)
            .and_then(|v| v.release.first().copied())
            .map(|n| n as i64)
    }

    pub fn get_minor(&self, version: &str) -> Option<i64> {
        self.parse(version)
            .and_then(|v| v.release.get(1).copied())
            .map(|n| n as i64)
    }

    pub fn get_patch(&self, version: &str) -> Option<i64> {
        self.parse(version)
            .and_then(|v| v.release.get(2).copied())
            .map(|n| n as i64)
    }

    pub fn equals(&self, a: &str, b: &str) -> bool {
        self.compare(a, b) == Ordering::Equal
    }

    pub fn is_greater_than(&self, a: &str, b: &str) -> bool {
        self.compare(a, b) == Ordering::Greater
    }

    pub fn is_less_than_range(&self, version: &str, range: &str) -> bool {
        self.compare(version, range) == Ordering::Less
    }

    pub fn sort_versions(&self, a: &str, b: &str) -> i32 {
        self.compare_numeric(a, b)
    }

    pub fn matches_range(&self, version: &str, range: &str) -> bool {
        self.equals(version, range)
    }

    pub fn is_compatible(&self, version: &str, current: &str) -> bool {
        let pv = self.parse(version);
        let pc = self.parse(current);
        match pv {
            None => false,
            Some(v) => {
                // When current fails to parse, compare against None (no compatibility group)
                // mirrors TypeScript: parsedVersion.compatibility === parsedCurrent?.compatibility
                let current_compat = pc.as_ref().and_then(|c| c.compatibility.as_deref());
                v.compatibility.as_deref() == current_compat
            }
        }
    }

    pub fn get_satisfying_version<'a>(&self, versions: &[&'a str], range: &str) -> Option<&'a str> {
        versions.iter().find(|&&v| self.equals(v, range)).copied()
    }

    pub fn min_satisfying_version<'a>(&self, versions: &[&'a str], range: &str) -> Option<&'a str> {
        self.get_satisfying_version(versions, range)
    }

    pub fn get_new_value(
        &self,
        current_value: &str,
        current_version: Option<&str>,
        new_version: &str,
    ) -> Option<String> {
        if new_version.is_empty() {
            return None;
        }
        if current_version == Some(&format!("v{current_value}")) {
            return Some(new_version.trim_start_matches('v').to_owned());
        }
        Some(new_version.to_owned())
    }
}

// ── Numeric-aware string comparison ──────────────────────────────────────────

/// Compare two strings with embedded numbers compared numerically.
/// Mirrors JavaScript `localeCompare(_, undefined, { numeric: true })`.
fn numeric_aware_cmp(a: &str, b: &str) -> Ordering {
    let a_parts = split_numeric(a);
    let b_parts = split_numeric(b);
    for (ap, bp) in a_parts.iter().zip(b_parts.iter()) {
        let cmp = match (ap.parse::<u64>(), bp.parse::<u64>()) {
            (Ok(an), Ok(bn)) => an.cmp(&bn),
            _ => ap.as_str().cmp(bp.as_str()),
        };
        if cmp != Ordering::Equal {
            return cmp;
        }
    }
    a_parts.len().cmp(&b_parts.len())
}

fn split_numeric(s: &str) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_digit = false;
    for c in s.chars() {
        let is_digit = c.is_ascii_digit();
        if current.is_empty() {
            in_digit = is_digit;
            current.push(c);
        } else if is_digit == in_digit {
            current.push(c);
        } else {
            parts.push(current.clone());
            current = c.to_string();
            in_digit = is_digit;
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn semver_re() -> RegexVersioning {
        RegexVersioning::new(
            r"^(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)(?<prerelease>[^.-]+)?(?:-(?<compatibility>.*))?$",
        )
        .unwrap()
    }

    // Ported: "requires a valid configuration to be initialized" — lib/modules/versioning/regex/index.spec.ts line 10
    #[test]
    fn regex_invalid_config_throws() {
        assert!(RegexVersioning::from_config("regex:not a regex").is_err());
    }

    // Ported: "works without config" — lib/modules/versioning/regex/index.spec.ts line 14
    #[test]
    fn regex_no_config() {
        let re = RegexVersioning::from_config("regex").unwrap();
        assert!(!re.is_valid("alpine")); // 'alpine' doesn't match ^\d+$
    }

    // Ported: "works with missing version" — lib/modules/versioning/regex/index.spec.ts line 19
    #[test]
    fn regex_with_missing_version() {
        let re = RegexVersioning::new(r"^(?<major>\d+)?(?<compabillity>.+)").unwrap();
        assert!(re.is_valid("alpine"));
    }

    // Ported: "on invalid regex: "$regex"" — lib/modules/versioning/regex/index.spec.ts line 25
    #[test]
    fn regex_invalid_patterns() {
        // Unclosed paren
        assert!(RegexVersioning::new(r"^(?<major>\d+)(").is_err());
        // Lookbehind (unsupported by the regex crate)
        assert!(RegexVersioning::new(r"^(?<major>\d+)?(?<!y)x$").is_err());
        assert!(RegexVersioning::new(r"^(?<major>\d+)?(?<=y)x$").is_err());
    }

    // Ported: "isValid("$version") === $expected" — lib/modules/versioning/regex/index.spec.ts line 35
    #[test]
    fn regex_is_valid() {
        let re = semver_re();
        assert!(!re.is_valid("1"));
        assert!(!re.is_valid("aardvark"));
        assert!(!re.is_valid("1.2a1-foo"));
        assert!(re.is_valid("1.2.3"));
        assert!(re.is_valid("1.2.3a1"));
        assert!(re.is_valid("1.2.3b2"));
        assert!(re.is_valid("1.2.3-foo"));
        assert!(re.is_valid("1.2.3b2-foo"));
        assert!(re.is_valid("1.2.3b2-foo-bar"));
        assert!(!re.is_valid("1.2.3.4.5.6.7"));
        assert!(!re.is_valid("1.2.aardvark"));
        assert!(!re.is_valid("1.2a2.3"));
    }

    // Ported: "isCompatible("$version") === $expected" — lib/modules/versioning/regex/index.spec.ts line 58
    #[test]
    fn regex_is_compatible() {
        let re = semver_re();
        // same compatibility
        assert!(re.is_compatible("1.2.3", "2.3.4"));
        assert!(re.is_compatible("1.2.3a1", "2.3.4"));
        assert!(re.is_compatible("1.2.3-foobar", "2.3.4-foobar"));
        assert!(re.is_compatible("1.2.3a1-foobar", "2.3.4-foobar"));
        // different compatibility
        assert!(!re.is_compatible("1.2.3", "2.3.4-foobar"));
        assert!(!re.is_compatible("1.2.3-foobar", "2.3.4"));
        assert!(!re.is_compatible("1.2.3-foo", "2.3.4-bar"));
    }

    // Ported: "isSingleVersion("$version") === $expected" — lib/modules/versioning/regex/index.spec.ts line 83
    #[test]
    fn regex_is_single_version() {
        let re = semver_re();
        assert!(re.is_valid("1.2.3"));
        assert!(!re.is_valid("1"));
    }

    // Ported: "isStable("$version") === $expected" — lib/modules/versioning/regex/index.spec.ts line 104
    #[test]
    fn regex_is_stable() {
        let re = semver_re();
        assert!(re.is_stable("1.2.3"));
        assert!(re.is_stable("1.2.3-foo")); // compatibility, no prerelease
        assert!(!re.is_stable("1.2.3alpha")); // prerelease
        assert!(!re.is_stable("1.2.3b3-foo")); // prerelease + compatibility
    }

    // Ported: "isVersion("$version") === $expected" — lib/modules/versioning/regex/index.spec.ts line 115
    #[test]
    fn regex_is_version() {
        let re = semver_re();
        assert!(re.is_valid("1.2.3"));
        assert!(re.is_valid("1.2.3a1"));
        assert!(!re.is_valid("1"));
    }

    // Ported: "getMajor, getMinor, getPatch for "$version"" — lib/modules/versioning/regex/index.spec.ts line 135
    #[test]
    fn regex_major_minor_patch() {
        let re = semver_re();
        assert_eq!(re.get_major("1.2.3"), Some(1));
        assert_eq!(re.get_minor("1.2.3"), Some(2));
        assert_eq!(re.get_patch("1.2.3"), Some(3));

        assert_eq!(re.get_major("1.2.3a1"), Some(1));
        assert_eq!(re.get_minor("1.2.3a1"), Some(2));
        assert_eq!(re.get_patch("1.2.3a1"), Some(3));

        assert_eq!(re.get_major("1.2.3a1-foo"), Some(1));
        assert_eq!(re.get_minor("1.2.3a1-foo"), Some(2));
        assert_eq!(re.get_patch("1.2.3a1-foo"), Some(3));
    }

    // Ported: "equals($a, $b) === $expected" — lib/modules/versioning/regex/index.spec.ts line 149
    #[test]
    fn regex_equals() {
        let re = semver_re();
        assert!(re.equals("1.2.3", "1.2.3"));
        assert!(re.equals("1.2.3a1", "1.2.3a1"));
        assert!(re.equals("1.2.3a1-foo", "1.2.3a1-foo"));
        // compatibility doesn't affect equality
        assert!(re.equals("1.2.3", "1.2.3-bar"));
        assert!(re.equals("1.2.3a1", "1.2.3a1-bar"));
        assert!(re.equals("1.2.3a1-foo", "1.2.3a1-bar"));
        // different versions
        assert!(!re.equals("1.2.3", "1.2.4"));
        assert!(!re.equals("1.2.3", "1.2.3a1"));
        assert!(!re.equals("1.2.3a1", "1.2.3a2"));
    }

    // Ported: "isGreaterThan("$a", "$b") === $expected" — lib/modules/versioning/regex/index.spec.ts line 171
    #[test]
    fn regex_is_greater_than() {
        let re = semver_re();
        assert!(re.is_greater_than("2.0.0", "1.0.0"));
        assert!(re.is_greater_than("2.2.0", "2.1.0"));
        assert!(re.is_greater_than("2.2.1", "2.2.0"));
        assert!(re.is_greater_than("3.0.0a2", "3.0.0a1"));
        assert!(re.is_greater_than("3.0.0b1", "3.0.0a2"));
        assert!(re.is_greater_than("3.0.0", "3.0.0b2")); // release > pre-release
        // equal versions → not greater
        assert!(!re.is_greater_than("1.0.0", "1.0.0"));
        assert!(!re.is_greater_than("1.0.0", "1.0.0-foo")); // compat ignored
        // less than
        assert!(!re.is_greater_than("1.0.0", "2.0.0"));
        assert!(!re.is_greater_than("3.0.0b2", "3.0.0")); // pre-release < release
    }

    // Ported: "isLessThanRange($version, $range) === $expected" — lib/modules/versioning/regex/index.spec.ts line 204
    #[test]
    fn regex_is_less_than_range() {
        let re = semver_re();
        assert!(re.is_less_than_range("1.2.2", "1.2.3"));
        assert!(re.is_less_than_range("1.2.2", "1.2.3-bar"));
        assert!(re.is_less_than_range("1.2.2", "1.2.3a1"));
        assert!(!re.is_less_than_range("1.2.2", "1.2.2")); // equal → not less
        assert!(!re.is_less_than_range("1.2.4", "1.2.3")); // greater → not less
    }

    // Ported: "getSatisfyingVersion($versions, "$range") === $expected" — lib/modules/versioning/regex/index.spec.ts line 253
    #[test]
    fn regex_get_satisfying_version() {
        let re = semver_re();
        let v = &["2.1.5", "2.1.6a1", "2.1.6", "2.1.6-foo"];
        assert_eq!(re.get_satisfying_version(v, "2.1.6"), Some("2.1.6"));
        assert_eq!(re.get_satisfying_version(v, "2.1.6-foo"), Some("2.1.6")); // equals ignores compat
        assert_eq!(
            re.get_satisfying_version(&["2.1.5-foo", "2.1.6"], "2.1.6-foo"),
            Some("2.1.6")
        );
        assert_eq!(
            re.get_satisfying_version(&["1.2.3", "1.2.4"], "3.5.0"),
            None
        );
        assert_eq!(re.get_satisfying_version(&["1.2.3", "1.2.4"], "!@#"), None);
    }

    // Ported: "minSatisfyingVersion($versions, "$range") === "$expected"" — lib/modules/versioning/regex/index.spec.ts line 267
    #[test]
    fn regex_min_satisfying_version() {
        let re = semver_re();
        let v = &["2.1.5", "2.1.6a1", "2.1.6", "2.1.6-foo"];
        assert_eq!(re.min_satisfying_version(v, "2.1.6"), Some("2.1.6"));
        assert_eq!(re.min_satisfying_version(v, "2.1.6-foo"), Some("2.1.6"));
        assert_eq!(
            re.min_satisfying_version(&["2.1.5", "2.1.6-foo"], "2.1.5-foo"),
            Some("2.1.5")
        );
        assert_eq!(
            re.min_satisfying_version(&["1.2.3", "1.2.4"], "3.5.0"),
            None
        );
        assert_eq!(re.min_satisfying_version(&["1.2.3", "1.2.4"], "!@#"), None);
    }

    // Ported: "returns newVersion" — lib/modules/versioning/regex/index.spec.ts line 282
    #[test]
    fn regex_get_new_value() {
        let re = semver_re();
        assert_eq!(
            re.get_new_value("", None, "1.2.3"),
            Some("1.2.3".to_owned())
        );
    }

    // Ported: "sorts versions in an ascending order" — lib/modules/versioning/regex/index.spec.ts line 295
    #[test]
    fn regex_sort_versions() {
        let re = semver_re();
        let mut versions = vec!["1.2.3a1", "2.0.1", "1.3.4", "1.2.3"];
        versions.sort_by(|a, b| {
            re.sort_versions(a, b)
                .cmp(&0)
                .then(std::cmp::Ordering::Equal)
        });
        // Correct sort: 1.2.3a1 (pre-release of 1.2.3) < 1.2.3 < 1.3.4 < 2.0.1
        assert_eq!(versions, vec!["1.2.3a1", "1.2.3", "1.3.4", "2.0.1"]);
    }

    // Ported: "matches("$version", "$range") === $expected" — lib/modules/versioning/regex/index.spec.ts line 304
    #[test]
    fn regex_matches() {
        let re = semver_re();
        assert!(re.matches_range("1.2.2", "1.2.2"));
        assert!(re.matches_range("1.2.2", "1.2.2-bar")); // compat ignored
        assert!(re.matches_range("1.2.2-foo", "1.2.2"));
        assert!(!re.matches_range("1.2.2", "1.2.3"));
        assert!(!re.matches_range("1.2.4", "1.2.3"));
    }

    // Ported: "isValid(\"$version\") === $expected" — lib/modules/versioning/regex/index.spec.ts line 365
    #[test]
    fn regex_build_revision_is_valid() {
        let re = RegexVersioning::new(
            r"^(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)(:?-(?<compatibility>.+)(?<build>\d+)-r(?<revision>\d+))?$",
        )
        .unwrap();
        assert!(re.is_valid("12.7.0-debian-10-r69"));
        assert!(re.is_valid("12.7.0-debian-10-r100"));
    }

    // Ported: "isCompatible(\"$version\") === $expected" — lib/modules/versioning/regex/index.spec.ts line 373
    #[test]
    fn regex_build_revision_is_compatible() {
        let re = RegexVersioning::new(
            r"^(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)(:?-(?<compatibility>.+)(?<build>\d+)-r(?<revision>\d+))?$",
        )
        .unwrap();
        assert!(re.is_compatible("12.7.0-debian-10-r69", "12.7.0-debian-10-r100"));
    }

    // Ported: "isGreaterThan(\"$a\", \"$b\") === $expected" — lib/modules/versioning/regex/index.spec.ts line 384
    #[test]
    fn regex_build_revision_is_greater_than() {
        let re = RegexVersioning::new(
            r"^(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)(:?-(?<compatibility>.+)(?<build>\d+)-r(?<revision>\d+))?$",
        )
        .unwrap();
        assert!(!re.is_greater_than("12.7.0-debian-10-r69", "12.7.0-debian-10-r100"));
        assert!(re.is_greater_than("12.7.0-debian-10-r169", "12.7.0-debian-10-r100"));
    }

    // Ported: "matches(\"$version\", \"$range\") === $expected" — lib/modules/versioning/regex/index.spec.ts line 392
    #[test]
    fn regex_build_revision_matches() {
        let re = RegexVersioning::new(
            r"^(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)(:?-(?<compatibility>.+)(?<build>\d+)-r(?<revision>\d+))?$",
        )
        .unwrap();
        assert!(re.matches_range("12.7.0-debian-9-r69", "12.7.0-debian-9-r69"));
        assert!(!re.matches_range("12.7.0-debian-9-r69", "12.7.0-debian-10-r68"));
    }

    // Ported: "getSatisfyingVersion" (4-part) — lib/modules/versioning/regex/index.spec.ts line 403
    #[test]
    fn regex_build_revision_satisfying() {
        let re = RegexVersioning::new(
            r"^(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)(:?-(?<compatibility>.+)(?<build>\d+)-r(?<revision>\d+))?$",
        )
        .unwrap();
        let versions = &[
            "12.7.0-debian-10-r69",
            "12.7.0-debian-10-r100",
            "12.7.0-debian-10-r101",
        ];
        assert_eq!(
            re.get_satisfying_version(versions, "12.7.0-debian-10-r100"),
            Some("12.7.0-debian-10-r100")
        );
        assert_eq!(
            re.get_satisfying_version(versions, "12.7.0-debian-12-r100"),
            None
        );
    }

    // Ported: "minSatisfyingVersion" (4-part) — lib/modules/versioning/regex/index.spec.ts line 412
    #[test]
    fn regex_build_revision_min_satisfying() {
        let re = RegexVersioning::new(
            r"^(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)(:?-(?<compatibility>.+)(?<build>\d+)-r(?<revision>\d+))?$",
        )
        .unwrap();
        let versions = &[
            "12.7.0-debian-10-r69",
            "12.7.0-debian-10-r100",
            "12.7.0-debian-10-r101",
        ];
        assert_eq!(
            re.min_satisfying_version(versions, "12.7.0-debian-10-r100"),
            Some("12.7.0-debian-10-r100")
        );
        assert_eq!(
            re.min_satisfying_version(versions, "12.7.0-debian-12-r100"),
            None
        );
    }

    // ── generic.spec.ts — GenericVersioningApi DummyScheme tests ────────────
    //
    // TypeScript uses GenericVersioningApi with _parse() → RegexVersioning
    // with the same named-group pattern is the Rust equivalent.

    fn dummy() -> RegexVersioning {
        // mirrors the TypeScript DummyScheme pattern exactly
        RegexVersioning::new(
            r"^(?P<major>\d)\.(?P<minor>\d)\.(?P<patch>\d)(?:-(?P<prerelease>.+))?$",
        )
        .unwrap()
    }

    // Ported: "Scheme keys" — lib/modules/versioning/generic.spec.ts line 54
    #[test]
    fn generic_scheme_keys_api_methods_exist() {
        let api = dummy();
        // Verify all expected public API methods exist by calling them
        assert!(api.is_valid("1.2.3"));
        assert!(api.is_stable("1.2.3"));
        assert!(api.get_major("1.2.3").is_some());
        assert!(api.get_minor("1.2.3").is_some());
        assert!(api.get_patch("1.2.3").is_some());
        assert!(api.equals("1.2.3", "1.2.3"));
        assert!(api.is_greater_than("3.2.1", "1.2.3"));
        assert!(api.is_compatible("1.2.3", "1.0.0"));
        // isSingleVersion and isVersion in TypeScript = is_valid in Rust for RegexVersioning
        assert!(api.is_valid("1.2.3"));
        assert!(api.matches_range("1.2.3", "1.2.3"));
        assert!(api.sort_versions("1.2.3", "1.2.3") == 0);
        assert!(api.is_less_than_range("1.2.3", "3.2.1"));
        assert!(api.get_new_value("1.2.3", Some("1.2.3"), "3.2.1").is_some());
    }

    // Ported: "equals" — lib/modules/versioning/generic.spec.ts line 82
    #[test]
    fn generic_equals() {
        let api = dummy();
        assert!(api.equals("1.2.3", "1.2.3"));
        assert!(!api.equals("1.2.3", "3.2.1"));
    }

    // Ported: "getMajor" — lib/modules/versioning/generic.spec.ts line 87
    #[test]
    fn generic_get_major() {
        let api = dummy();
        assert_eq!(api.get_major("4.5.6"), Some(4));
        assert_eq!(api.get_major("invalid"), None);
    }

    // Ported: "getMinor" — lib/modules/versioning/generic.spec.ts line 92
    #[test]
    fn generic_get_minor() {
        let api = dummy();
        assert_eq!(api.get_minor("4.5.6"), Some(5));
        assert_eq!(api.get_minor("invalid"), None);
    }

    // Ported: "getPatch" — lib/modules/versioning/generic.spec.ts line 97
    #[test]
    fn generic_get_patch() {
        let api = dummy();
        assert_eq!(api.get_patch("4.5.6"), Some(6));
        assert_eq!(api.get_patch("invalid"), None);
    }

    // Ported: "getNewValue" — lib/modules/versioning/generic.spec.ts line 102
    #[test]
    fn generic_get_new_value() {
        let api = dummy();
        assert_eq!(
            api.get_new_value("1.2.3", Some("1.2.3"), "3.2.1"),
            Some("3.2.1".to_owned())
        );
        // currentVersion is v-prefixed → strip v from newVersion
        assert_eq!(
            api.get_new_value("1.2.3", Some("v1.2.3"), "v3.2.1"),
            Some("3.2.1".to_owned())
        );
        // empty newVersion (equivalent to partial<NewValueConfig>({})) → None
        assert_eq!(api.get_new_value("", None, ""), None);
    }

    // Ported: "isCompatible" — lib/modules/versioning/generic.spec.ts line 124
    #[test]
    fn generic_is_compatible() {
        let api = dummy();
        assert!(api.is_compatible("1.2.3", ""));
    }

    // Ported: "isGreaterThan" — lib/modules/versioning/generic.spec.ts line 128
    #[test]
    fn generic_is_greater_than() {
        let api = dummy();
        assert!(!api.is_greater_than("1.2.3", "3.2.1"));
        assert!(api.is_greater_than("3.2.1", "1.2.3"));
        assert!(api.is_greater_than("1.2.3-a10", "1.2.3-a1"));
    }

    // Ported: "isSingleVersion" — lib/modules/versioning/generic.spec.ts line 134
    // TypeScript isSingleVersion = is_valid in Rust for RegexVersioning (all valid versions are single)
    #[test]
    fn generic_is_single_version() {
        let api = dummy();
        assert!(api.is_valid("1.2.3"));
    }

    // Ported: "isStable" — lib/modules/versioning/generic.spec.ts line 138
    #[test]
    fn generic_is_stable() {
        let api = dummy();
        assert!(api.is_stable("1.2.3"));
    }

    // Ported: "isValid" — lib/modules/versioning/generic.spec.ts line 142
    #[test]
    fn generic_is_valid() {
        let api = dummy();
        assert!(api.is_valid("1.2.3"));
        assert!(api.is_valid("1.2.3-a1"));
        assert!(!api.is_valid("invalid"));
    }

    // Ported: "isVersion" — lib/modules/versioning/generic.spec.ts line 148
    // TypeScript isVersion = is_valid in Rust for RegexVersioning
    #[test]
    fn generic_is_version() {
        let api = dummy();
        assert!(api.is_valid("1.2.3"));
        assert!(!api.is_valid("invalid"));
    }

    // Ported: "matches" — lib/modules/versioning/generic.spec.ts line 153
    #[test]
    fn generic_matches() {
        let api = dummy();
        assert!(api.matches_range("1.2.3", "1.2.3"));
        assert!(!api.matches_range("1.2.3", "3.2.1"));
    }

    // Ported: "sortVersions" — lib/modules/versioning/generic.spec.ts line 158
    #[test]
    fn generic_sort_versions() {
        let api = dummy();
        assert_eq!(api.sort_versions("1.2.3", "1.2.3"), 0);
        assert_eq!(api.sort_versions("1.2.3", "3.2.1"), -2);
        assert_eq!(api.sort_versions("3.2.1", "1.2.3"), 2);
    }

    // Ported: "isLessThanRange" — lib/modules/versioning/generic.spec.ts line 164
    #[test]
    fn generic_is_less_than_range() {
        let api = dummy();
        assert!(api.is_less_than_range("1.2.3", "3.2.1"));
        assert!(!api.is_less_than_range("3.2.1", "1.2.3"));
    }

    // Ported: "minSatisfyingVersion" — lib/modules/versioning/generic.spec.ts line 169
    #[test]
    fn generic_min_satisfying_version() {
        let api = dummy();
        assert_eq!(
            api.min_satisfying_version(&["1.2.3"], "1.2.3"),
            Some("1.2.3")
        );
        assert_eq!(
            api.min_satisfying_version(&["1.1.1", "2.2.2", "3.3.3"], "2.2.2"),
            Some("2.2.2")
        );
        assert_eq!(
            api.min_satisfying_version(&["1.1.1", "2.2.2", "3.3.3"], "1.2.3"),
            None
        );
    }

    // Ported: "isSame" — lib/modules/versioning/generic.spec.ts line 189
    // TypeScript: isSame(type, a, b) compares a component of two versions
    #[test]
    fn generic_is_same() {
        let api = dummy();
        // major: 4 == 4 (4.5.6 vs 4.6.0)
        assert_eq!(api.get_major("4.5.6"), api.get_major("4.6.0")); // both 4
        // major: 4 != 5 (4.5.6 vs 5.0.0)
        assert_ne!(api.get_major("4.5.6"), api.get_major("5.0.0")); // 4 vs 5
        // minor: 5 == 5 (4.5.6 vs 5.5.0)
        assert_eq!(api.get_minor("4.5.6"), api.get_minor("5.5.0")); // both 5
        // minor: 5 != 6 (4.5.6 vs 4.6.0)
        assert_ne!(api.get_minor("4.5.6"), api.get_minor("4.6.0")); // 5 vs 6
        // patch: 6 == 6 (4.5.6 vs 5.5.6)
        assert_eq!(api.get_patch("4.5.6"), api.get_patch("5.5.6")); // both 6
        // patch: 6 != 0 (4.5.6 vs 4.6.0)
        assert_ne!(api.get_patch("4.5.6"), api.get_patch("4.6.0")); // 6 vs 0
    }

    // Ported: "getSatisfyingVersion" — lib/modules/versioning/generic.spec.ts line 179
    #[test]
    fn generic_get_satisfying_version() {
        let api = dummy();
        assert_eq!(
            api.get_satisfying_version(&["1.2.3"], "1.2.3"),
            Some("1.2.3")
        );
        assert_eq!(
            api.get_satisfying_version(&["1.1.1", "2.2.2", "3.3.3"], "2.2.2"),
            Some("2.2.2")
        );
        assert_eq!(
            api.get_satisfying_version(&["1.1.1", "2.2.2", "3.3.3"], "1.2.3"),
            None
        );
    }
}
