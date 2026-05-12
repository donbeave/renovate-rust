//! Helm versioning.
//!
//! Renovate reference:
//! - `lib/modules/versioning/helm/index.ts`

use std::sync::LazyLock;

use regex::Regex;
use semver::{Version, VersionReq};

static WILDCARD_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?<prefix>\d+(?:\.\d+)?)\.(?:x|X|\*)$").unwrap());
static GTE_UPPER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?<op>>=\s*)(?<lower>\S+)(?<mid>\s+<\s*|\s+<=\s*)(?<upper>\S+)$").unwrap()
});

fn parse_version(input: &str) -> Option<Version> {
    Version::parse(input.trim_start_matches('v')).ok()
}

fn major_zero_floor(version: &Version) -> Version {
    if version.major == 0 && version.minor == 0 {
        Version::new(0, 0, version.patch)
    } else if version.major == 0 {
        Version::new(0, version.minor, 0)
    } else {
        Version::new(version.major, 0, 0)
    }
}

fn bump_upper_for(version: &Version, upper: &str) -> String {
    let dot_count = upper.matches('.').count();
    if dot_count == 0 {
        (version.major + 1).to_string()
    } else if dot_count == 1 {
        format!("{}.{}", version.major, version.minor + 1)
    } else if version.major == 0 && version.minor == 0 {
        format!("0.0.{}", version.patch + 1)
    } else {
        format!("{}.{}.0", version.major, version.minor + 1)
    }
}

pub fn is_valid(input: &str) -> bool {
    VersionReq::parse(input).is_ok()
}

pub fn is_single_version(input: &str) -> bool {
    let input = input.trim();
    let input = input.strip_prefix('=').unwrap_or(input).trim();
    parse_version(input).is_some()
}

pub fn get_new_value(
    current_value: &str,
    range_strategy: &str,
    new_version: &str,
) -> Option<String> {
    if range_strategy == "widen" {
        if let Some(bound) = current_value.strip_prefix("<=") {
            let sep = if bound.starts_with(' ') { "<= " } else { "<=" };
            let bound = bound.trim();
            if parse_version(new_version)? > parse_version(bound)? {
                return Some(format!("{sep}{new_version}"));
            }
            return Some(current_value.to_owned());
        }
        if let Some(bound) = current_value.strip_prefix(">=") {
            let sep = if bound.starts_with(' ') { ">= " } else { ">=" };
            let bound = bound.trim();
            if parse_version(new_version)? < parse_version(bound)? {
                return Some(format!("{sep}{bound} || {new_version}"));
            }
            return Some(current_value.to_owned());
        }
        return None;
    }

    if let Some(captures) = GTE_UPPER_RE.captures(current_value) {
        let op = captures.name("op")?.as_str();
        let mid = captures.name("mid")?.as_str();
        let upper = captures.name("upper")?.as_str();
        let new = parse_version(new_version)?;
        let lower = new_version.trim_start_matches('v');
        let upper = bump_upper_for(&new, upper);
        return Some(format!("{op}{lower}{mid}{upper}"));
    }

    if current_value.starts_with('>') && !current_value.starts_with(">=") {
        return None;
    }

    if let Some(rest) = current_value.strip_prefix('=') {
        let sep = if rest.starts_with(' ') { "= " } else { "=" };
        return Some(format!("{sep}{}", new_version.trim_start_matches('v')));
    }

    if let Some(rest) = current_value.strip_prefix(">=") {
        let sep = if rest.starts_with(' ') { ">= " } else { ">=" };
        return Some(format!("{sep}{}", new_version.trim_start_matches('v')));
    }

    if let Some(captures) = WILDCARD_RE.captures(current_value) {
        let prefix = captures.name("prefix")?.as_str();
        let parts = new_version.split('.').collect::<Vec<_>>();
        if prefix.contains('.') {
            return Some(format!("{}.{}.*", parts.first()?, parts.get(1)?));
        }
        return Some(format!("{}.*", parts.first()?));
    }

    let new = parse_version(new_version)?;

    if let Some(rest) = current_value.strip_prefix('^') {
        if range_strategy == "bump" {
            return Some(format!("^{}", new_version.trim_start_matches('v')));
        }
        let current = parse_version(rest)?;
        if new < current {
            return Some(format!("^{}", new_version.trim_start_matches('v')));
        }
        if new.major == current.major && new.major > 0 {
            return Some(current_value.to_owned());
        }
        if new.major == current.major
            && new.major == 0
            && new.minor == current.minor
            && current.minor > 0
        {
            return Some(current_value.to_owned());
        }
        return Some(format!("^{}", major_zero_floor(&new)));
    }

    if let Some(rest) = current_value.strip_prefix('~') {
        if range_strategy == "bump" {
            return Some(format!("~{}", new_version.trim_start_matches('v')));
        }
        let new = parse_version(new_version)?;
        if new.pre.is_empty() {
            return Some(format!("~{}.{}.0", new.major, new.minor));
        }
        let pre = new
            .pre
            .as_str()
            .split('.')
            .next()
            .unwrap_or(new.pre.as_str());
        let _ = rest;
        return Some(format!("~{}.{}.{}-{pre}", new.major, new.minor, new.patch));
    }

    Some(new_version.trim_start_matches('v').to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid(\"$version\") === $isValid" — versioning/helm/index.spec.ts line 4
    #[test]
    fn is_valid_matches_renovate_helm_spec() {
        let cases = [
            ("17.04.0", false),
            ("1.2.3", true),
            ("1.2.3-foo", true),
            ("1.2.3foo", false),
            ("~1.2.3", true),
            ("^1.2.3", true),
            (">1.2.3", true),
            (">=1.2.3", true),
            ("renovatebot/renovate", false),
            ("renovatebot/renovate#main", false),
            ("https://github.com/renovatebot/renovate.git", false),
        ];

        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version})");
        }
    }

    // Ported: "isSingleVersion(\"$version\") === $isSingle" — versioning/helm/index.spec.ts line 22
    #[test]
    fn is_single_version_matches_renovate_helm_spec() {
        let cases = [
            ("1.2.3", true),
            ("1.2.3-alpha.1", true),
            ("=1.2.3", true),
            ("= 1.2.3", true),
            ("1.x", false),
        ];

        for (version, expected) in cases {
            assert_eq!(
                is_single_version(version),
                expected,
                "is_single_version({version})"
            );
        }
    }

    // Ported: "getNewValue(\"$currentValue\", \"$rangeStrategy\", \"$currentVersion\", \"$newVersion\") === \"$expected\"" — versioning/helm/index.spec.ts line 35
    #[test]
    fn get_new_value_matches_renovate_helm_spec() {
        let cases = [
            ("=1.0.0", "bump", "1.1.0", Some("=1.1.0")),
            ("^1.0", "bump", "1.0.7", Some("^1.0.7")),
            (
                "^1",
                "bump",
                "1.0.7-prerelease.1",
                Some("^1.0.7-prerelease.1"),
            ),
            ("^1.0", "bump", "1.1.7", Some("^1.1.7")),
            ("~1.0", "bump", "1.1.7", Some("~1.1.7")),
            (
                "~1.0",
                "bump",
                "1.0.7-prerelease.1",
                Some("~1.0.7-prerelease.1"),
            ),
            ("^1", "bump", "2.1.7", Some("^2.1.7")),
            ("~1", "bump", "1.1.7", Some("~1.1.7")),
            ("5", "bump", "5.1.7", Some("5.1.7")),
            ("5", "bump", "6.1.7", Some("6.1.7")),
            ("5.0", "bump", "5.0.7", Some("5.0.7")),
            ("5.0", "bump", "5.1.7", Some("5.1.7")),
            ("5.0", "bump", "6.1.7", Some("6.1.7")),
            (">=1.0.0", "bump", "1.1.0", Some(">=1.1.0")),
            (">= 1.0.0", "bump", "1.1.0", Some(">= 1.1.0")),
            ("=1.0.0", "replace", "1.1.0", Some("=1.1.0")),
            ("1.0.*", "replace", "1.1.0", Some("1.1.*")),
            ("1.*", "replace", "2.1.0", Some("2.*")),
            ("~0.6.1", "replace", "0.7.0-rc.2", Some("~0.7.0-rc")),
            (
                ">= 0.1.21 < 0.2.0",
                "bump",
                "0.1.24",
                Some(">= 0.1.24 < 0.2.0"),
            ),
            (
                ">= 0.1.21 <= 0.2.0",
                "bump",
                "0.1.24",
                Some(">= 0.1.24 <= 0.2.0"),
            ),
            (">= 0.0.1 <= 0.1", "bump", "0.0.2", Some(">= 0.0.2 <= 0.1")),
            (">= 0.0.1 < 0.1", "bump", "0.2.1", Some(">= 0.2.1 < 0.3")),
            (
                ">= 0.0.1 < 0.0.4",
                "bump",
                "0.0.5",
                Some(">= 0.0.5 < 0.0.6"),
            ),
            (">= 0.0.1 < 1", "bump", "1.0.1", Some(">= 1.0.1 < 2")),
            ("<=1.2.3", "widen", "1.2.3", Some("<=1.2.3")),
            ("<=1.2.3", "widen", "1.2.4", Some("<=1.2.4")),
            (">=1.2.3", "widen", "1.2.3", Some(">=1.2.3")),
            (">=1.2.3", "widen", "1.2.1", Some(">=1.2.3 || 1.2.1")),
            ("^0.0.3", "replace", "0.0.6", Some("^0.0.6")),
            ("^0.0.3", "replace", "0.5.0", Some("^0.5.0")),
            ("^0.0.3", "replace", "0.5.6", Some("^0.5.0")),
            ("^0.0.3", "replace", "4.0.0", Some("^4.0.0")),
            ("^0.0.3", "replace", "4.0.6", Some("^4.0.0")),
            ("^0.0.3", "replace", "4.5.6", Some("^4.0.0")),
            ("^0.2.0", "replace", "0.5.6", Some("^0.5.0")),
            ("^0.2.3", "replace", "0.5.0", Some("^0.5.0")),
            ("^0.2.3", "replace", "0.5.6", Some("^0.5.0")),
            ("^1.2.3", "replace", "4.0.0", Some("^4.0.0")),
            ("^1.2.3", "replace", "4.5.6", Some("^4.0.0")),
            ("^1.0.0", "replace", "4.5.6", Some("^4.0.0")),
            ("^0.2.3", "replace", "0.2.4", Some("^0.2.3")),
            ("^2.3.0", "replace", "2.4.0", Some("^2.3.0")),
            ("^2.3.4", "replace", "2.4.5", Some("^2.3.4")),
            ("^0.0.1", "replace", "0.0.2", Some("^0.0.2")),
            ("^1.0.1", "replace", "2.0.2", Some("^2.0.0")),
            ("^1.2.3", "replace", "1.2.3", Some("^1.2.3")),
            ("^1.2.3", "replace", "1.2.2", Some("^1.2.2")),
            ("^0.9.21", "replace", "0.9.22", Some("^0.9.21")),
        ];

        for (current_value, range_strategy, new_version, expected) in cases {
            assert_eq!(
                get_new_value(current_value, range_strategy, new_version).as_deref(),
                expected,
                "get_new_value({current_value}, {range_strategy}, {new_version})"
            );
        }
    }
}
