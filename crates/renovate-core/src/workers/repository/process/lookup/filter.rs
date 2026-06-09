//! Version filtering logic.
//!
//! Mirrors `lib/workers/repository/process/lookup/filter.ts`.

use super::types::{FilterConfig, UpdateType};

#[derive(Debug, Clone, Default)]
pub struct Release {
    pub version: String,
    pub is_deprecated: Option<bool>,
    pub is_stable: Option<bool>,
    pub release_timestamp: Option<String>,
}

impl Release {
    pub fn new(version: &str) -> Self {
        Self {
            version: version.into(),
            ..Default::default()
        }
    }

    pub fn is_stable(&self) -> bool {
        self.is_stable.unwrap_or(true)
    }
}

fn is_release_stable(release: &Release) -> bool {
    release.is_stable()
}

pub fn filter_versions(
    config: &FilterConfig,
    current_version: &str,
    releases: &[Release],
) -> Vec<Release> {
    if current_version.is_empty() {
        return Vec::new();
    }

    let mut filtered: Vec<Release> = releases
        .iter()
        .filter(|r| r.version.as_str() > current_version)
        .cloned()
        .collect();

    let current_release = releases.iter().find(|r| r.version == current_version);

    if config.ignore_deprecated.unwrap_or(false)
        && let Some(curr) = current_release
        && !curr.is_deprecated.unwrap_or(false)
    {
        filtered.retain(|r| !r.is_deprecated.unwrap_or(false));
    }

    if let Some(max_inc) = config.max_major_increment
        && max_inc > 0
    {
        let current_major = parse_major(current_version);
        filtered.retain(|r| {
            let release_major = parse_major(&r.version);
            release_major.saturating_sub(current_major) <= max_inc
        });
    }

    if config.ignore_unstable.unwrap_or(true)
        && let Some(curr) = current_release
        && is_release_stable(curr)
    {
        filtered.retain(is_release_stable);
    }

    filtered
}

fn parse_major(version: &str) -> u64 {
    version
        .split('.')
        .next()
        .and_then(|s| {
            s.trim_start_matches(|c: char| !c.is_ascii_digit())
                .parse::<u64>()
                .ok()
        })
        .unwrap_or(0)
}

pub fn is_update_allowed(update_type: UpdateType, allowed_types: &[UpdateType]) -> bool {
    if allowed_types.is_empty() {
        return true;
    }
    allowed_types.contains(&update_type)
}

pub fn sort_by_update_type(updates: &mut [(String, UpdateType)]) {
    let priority = |ut: &UpdateType| -> u32 {
        match ut {
            UpdateType::Major => 0,
            UpdateType::Minor => 1,
            UpdateType::Patch => 2,
            UpdateType::Pin => 3,
            UpdateType::Bump => 4,
            UpdateType::Digest => 5,
            UpdateType::PinDigest => 6,
            UpdateType::LockFileMaintenance => 7,
            UpdateType::LockfileUpdate => 8,
            UpdateType::Rollback => 9,
            UpdateType::Replacement => 10,
            UpdateType::Replace => 11,
        }
    };
    updates.sort_by_key(|(_, ut)| priority(ut));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_new() {
        let r = Release::new("1.2.3");
        assert_eq!(r.version, "1.2.3");
        assert!(r.is_deprecated.is_none());
    }

    #[test]
    fn release_is_stable_default() {
        let r = Release::new("1.0.0");
        assert!(r.is_stable());
    }

    #[test]
    fn release_is_stable_explicit_false() {
        let mut r = Release::new("1.0.0-beta");
        r.is_stable = Some(false);
        assert!(!r.is_stable());
    }

    #[test]
    fn filter_versions_empty_current() {
        let config = FilterConfig::default();
        let releases = vec![Release::new("1.0.0")];
        let result = filter_versions(&config, "", &releases);
        assert!(result.is_empty());
    }

    #[test]
    fn filter_versions_basic() {
        let config = FilterConfig::default();
        let releases = vec![
            Release::new("1.0.0"),
            Release::new("1.0.1"),
            Release::new("1.1.0"),
            Release::new("2.0.0"),
        ];
        let result = filter_versions(&config, "1.0.0", &releases);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].version, "1.0.1");
        assert_eq!(result[1].version, "1.1.0");
        assert_eq!(result[2].version, "2.0.0");
    }

    #[test]
    fn filter_versions_ignore_deprecated() {
        let config = FilterConfig {
            ignore_deprecated: Some(true),
            ..Default::default()
        };
        let mut deprecated = Release::new("1.0.1");
        deprecated.is_deprecated = Some(true);
        let releases = vec![Release::new("1.0.0"), deprecated, Release::new("1.1.0")];
        let result = filter_versions(&config, "1.0.0", &releases);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].version, "1.1.0");
    }

    #[test]
    fn filter_versions_max_major_increment() {
        let config = FilterConfig {
            max_major_increment: Some(1),
            ..Default::default()
        };
        let releases = vec![
            Release::new("1.0.0"),
            Release::new("2.0.0"),
            Release::new("3.0.0"),
        ];
        let result = filter_versions(&config, "1.0.0", &releases);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].version, "2.0.0");
    }

    #[test]
    fn filter_versions_ignore_unstable() {
        let config = FilterConfig {
            ignore_unstable: Some(true),
            ..Default::default()
        };
        let mut unstable = Release::new("1.0.1-beta");
        unstable.is_stable = Some(false);
        let releases = vec![Release::new("1.0.0"), unstable, Release::new("1.0.2")];
        let result = filter_versions(&config, "1.0.0", &releases);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].version, "1.0.2");
    }

    // Ported: "filters versions with major increment greater than maxMajorIncrement" — lib/workers/repository/process/lookup/filter.spec.ts line 187
    #[test]
    fn filter_versions_max_major_increment_large() {
        let config = FilterConfig {
            max_major_increment: Some(50),
            ..Default::default()
        };
        let releases = vec![
            Release::new("19.2.0"),
            Release::new("20.0.0"),
            Release::new("21.0.0"),
            Release::new("2023.3.3"),
            Release::new("2024.1.1"),
        ];
        let result = filter_versions(&config, "19.2.0", &releases);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].version, "20.0.0");
        assert_eq!(result[1].version, "21.0.0");
    }

    // Ported: "allows all versions when maxMajorIncrement is 0" — lib/workers/repository/process/lookup/filter.spec.ts line 216
    #[test]
    fn filter_versions_max_major_increment_zero_allows_all() {
        let config = FilterConfig {
            max_major_increment: Some(0),
            ..Default::default()
        };
        let releases = vec![
            Release::new("19.2.0"),
            Release::new("20.0.0"),
            Release::new("2023.3.3"),
        ];
        let result = filter_versions(&config, "19.2.0", &releases);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].version, "20.0.0");
        assert_eq!(result[1].version, "2023.3.3");
    }

    // Ported: "filters with maxMajorIncrement set to 1" — lib/workers/repository/process/lookup/filter.spec.ts line 243
    #[test]
    fn filter_versions_max_major_increment_one() {
        let config = FilterConfig {
            max_major_increment: Some(1),
            ..Default::default()
        };
        let releases = vec![
            Release::new("1.0.1"),
            Release::new("1.2.0"),
            Release::new("2.0.0"),
            Release::new("3.0.0"),
        ];
        let result = filter_versions(&config, "1.0.0", &releases);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].version, "1.0.1");
        assert_eq!(result[1].version, "1.2.0");
        assert_eq!(result[2].version, "2.0.0");
    }

    // Ported: "handles maxMajorIncrement with 0.x versions" — lib/workers/repository/process/lookup/filter.spec.ts line 272
    #[test]
    fn filter_versions_max_major_increment_with_zero_x() {
        let config = FilterConfig {
            max_major_increment: Some(1),
            ..Default::default()
        };
        let releases = vec![
            Release::new("0.1.0"),
            Release::new("0.2.0"),
            Release::new("1.0.0"),
            Release::new("2.0.0"),
        ];
        let result = filter_versions(&config, "0.0.1", &releases);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].version, "0.1.0");
        assert_eq!(result[1].version, "0.2.0");
        assert_eq!(result[2].version, "1.0.0");
    }

    #[test]
    fn filter_versions_no_newer() {
        let config = FilterConfig::default();
        let releases = vec![Release::new("0.9.0")];
        let result = filter_versions(&config, "1.0.0", &releases);
        assert!(result.is_empty());
    }

    #[test]
    fn is_update_allowed_empty_types() {
        assert!(is_update_allowed(UpdateType::Major, &[]));
    }

    #[test]
    fn is_update_allowed_included() {
        assert!(is_update_allowed(
            UpdateType::Major,
            &[UpdateType::Major, UpdateType::Minor]
        ));
    }

    #[test]
    fn is_update_allowed_excluded() {
        assert!(!is_update_allowed(
            UpdateType::Digest,
            &[UpdateType::Major, UpdateType::Minor]
        ));
    }

    #[test]
    fn sort_by_update_type_orders_correctly() {
        let mut updates = vec![
            ("patch".into(), UpdateType::Patch),
            ("major".into(), UpdateType::Major),
            ("digest".into(), UpdateType::Digest),
            ("minor".into(), UpdateType::Minor),
            ("pin".into(), UpdateType::Pin),
        ];
        sort_by_update_type(&mut updates);
        assert_eq!(updates[0].0, "major");
        assert_eq!(updates[1].0, "minor");
        assert_eq!(updates[2].0, "patch");
        assert_eq!(updates[3].0, "pin");
        assert_eq!(updates[4].0, "digest");
    }

    #[test]
    fn sort_by_update_type_empty() {
        let mut updates: Vec<(String, UpdateType)> = vec![];
        sort_by_update_type(&mut updates);
        assert!(updates.is_empty());
    }

    #[test]
    fn parse_major_versions() {
        assert_eq!(parse_major("1.2.3"), 1);
        assert_eq!(parse_major("10.0.0"), 10);
        assert_eq!(parse_major("0.1.0"), 0);
        assert_eq!(parse_major(""), 0);
    }

    // Ported: "should filter versions allowed by semver syntax when allowedversions is not valid version, range or pypi syntax" — lib/workers/repository/process/lookup/filter.spec.ts line 12
    #[test]
    fn filter_versions_allowed_by_semver_when_not_valid_range() {
        let config = FilterConfig {
            ignore_unstable: Some(false),
            ignore_deprecated: Some(false),
            respect_latest: Some(false),
            allowed_versions: Some(">1".to_string()),
            ..Default::default()
        };
        let releases = vec![
            Release::new("1.0.1"),
            Release::new("1.2.0"),
            Release::new("2.0.0"),
            Release::new("2.1.0"),
            Release::new("invalid.version"),
        ];
        let result = filter_versions(&config, "1.0.0", "", &releases, "");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].version, "2.0.0");
        assert_eq!(result[1].version, "2.1.0");
    }

    // Ported: "should filter versions when allowedversions templating is used" — lib/workers/repository/process/lookup/filter.spec.ts line 60
    #[test]
    fn filter_versions_when_allowedversions_templating_used() {
        let config = FilterConfig {
            ignore_unstable: Some(false),
            ignore_deprecated: Some(false),
            respect_latest: Some(false),
            allowed_versions: Some("<={{major}}.{{add minor 1}}.{{patch}}".to_string()),
            ..Default::default()
        };
        let releases = vec![
            Release::new("1.1.0"),
            Release::new("1.2.0"),
            Release::new("1.3.0"),
        ];
        let result = filter_versions(&config, "1.0.0", "", &releases, "");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].version, "1.1.0");
    }

    // Ported: "allows unstable major upgrades" — lib/workers/repository/process/lookup/filter.spec.ts line 98
    #[test]
    fn filter_versions_allows_unstable_major_upgrades() {
        let config = FilterConfig {
            ignore_unstable: Some(true),
            ignore_deprecated: Some(true),
            ..Default::default()
        };
        let releases = vec![
            { let mut r = Release::new("1.0.0-alpha"); r.is_stable = Some(false); r },
            { let mut r = Release::new("1.2.3-beta"); r.is_stable = Some(false); r },
        ];
        let result = filter_versions(&config, "1.0.0-alpha", "", &releases, "");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].version, "1.2.3-beta");
    }

    // Ported: "ignores version insufficient prefixes" — lib/workers/repository/process/lookup/filter.spec.ts line 124
    #[test]
    fn filter_versions_ignores_version_insufficient_prefixes() {
        let config = FilterConfig {
            ignore_unstable: Some(true),
            ignore_deprecated: Some(true),
            ..Default::default()
        };
        let releases = vec![
            Release::new("1.0.1"),
            Release::new("1.2.0"),
            { let mut r = Release::new("2.0.0"); r.is_deprecated = Some(true); r },
            Release::new("2.1.0"),
        ];
        let result = filter_versions(&config, "v1.0.1", "", &releases, "");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].version, "1.2.0");
        assert_eq!(result[1].version, "2.1.0");
    }

    // Ported: "single version range, but invalid current version (for coverage)" — lib/workers/repository/process/lookup/filter.spec.ts line 153
    #[test]
    fn filter_versions_single_version_range_invalid_current() {
        let config = FilterConfig {
            ignore_unstable: Some(false),
            ignore_deprecated: Some(false),
            respect_latest: Some(true),
            ..Default::default()
        };
        let releases = vec![
            Release::new("1.0.1"),
            Release::new("1.2.0"),
            Release::new("2.0.0"),
            Release::new("2.2.0"),
        ];
        let result = filter_versions(&config, "[1.0.1]", "2.0.0", &releases, "");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].version, "1.0.1");
        assert_eq!(result[1].version, "1.2.0");
        assert_eq!(result[2].version, "2.0.0");
    }
}
