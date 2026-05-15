//! Global rate-limiting and concurrency limit tracking.
//!
//! Mirrors `lib/workers/global/limits.ts`.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

// ---------- module-level state -----------------------------------------------

struct LimitEntry {
    max: Option<u32>,
    current: u32,
}

static LIMITS: LazyLock<Mutex<HashMap<String, LimitEntry>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static COUNTS: LazyLock<Mutex<HashMap<String, i64>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Clear all limit and count state.
///
/// Mirrors `resetAllLimits` from `lib/workers/global/limits.ts`.
pub fn reset_all_limits() {
    LIMITS.lock().unwrap().clear();
    COUNTS.lock().unwrap().clear();
}

/// Set the maximum for a named limit key. `None` (null) = unlimited.
/// Non-positive values are clamped to 0, making the limit immediately reached.
///
/// Mirrors `setMaxLimit` from `lib/workers/global/limits.ts`.
pub fn set_max_limit(key: &str, val: Option<i64>) {
    let max = val.map(|v| v.max(0) as u32);
    LIMITS.lock().unwrap().insert(key.to_owned(), LimitEntry { max, current: 0 });
}

/// Increment the current count for the named limit.
///
/// Mirrors `incLimitedValue` from `lib/workers/global/limits.ts`.
pub fn inc_limited_value(key: &str, inc_by: u32) {
    let mut m = LIMITS.lock().unwrap();
    let e = m.entry(key.to_owned()).or_insert(LimitEntry { max: None, current: 0 });
    e.current += inc_by;
}

/// Set a named count value directly.
///
/// Mirrors `setCount` from `lib/workers/global/limits.ts`.
pub fn set_count(key: &str, val: i64) {
    COUNTS.lock().unwrap().insert(key.to_owned(), val);
}

/// Increment a named count value by 1.
///
/// Mirrors `incCountValue` from `lib/workers/global/limits.ts`.
pub fn inc_count_value(key: &str) {
    let mut m = COUNTS.lock().unwrap();
    let v = *m.get(key).unwrap_or(&0);
    m.insert(key.to_owned(), v + 1);
}

/// Get a named count value.
///
/// Mirrors `getCount` from `lib/workers/global/limits.ts`.
pub fn get_count(key: &str) -> i64 {
    *COUNTS.lock().unwrap().get(key).unwrap_or(&0)
}

/// Check if the `'Commits'` limit is reached.
///
/// Mirrors the `'Commits'` overload of `isLimitReached`.
pub fn is_commits_limit_reached() -> bool {
    let m = LIMITS.lock().unwrap();
    if let Some(e) = m.get("Commits")
        && let Some(max) = e.max
    {
        return e.current >= max;
    }
    false
}

// ---------- upgrade config types for concurrent limits -----------------------

/// Branch-concurrent-limit value that distinguishes `null` (inherit from
/// `pr_concurrent_limit`) from `undefined` (skip / treat as MAX).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BranchConcurrentLimit {
    #[default]
    Unset,        // undefined → use i64::MAX in calc_limit
    Null,         // null → inherit from upgrade's pr_concurrent_limit
    Value(i64),   // explicit number (0 = no limit)
}

/// Per-upgrade configuration fields used for limit calculations.
///
/// Mirrors the limit-relevant fields of `BranchUpgradeConfig` from
/// `lib/workers/types.ts`.
///
/// For optional fields: `None` = `undefined` (treat as i64::MAX in
/// `calc_limit`).  `Some(0)` = no-limit sentinel (unlimited).
#[derive(Debug, Clone, Default)]
pub struct UpgradeConfig {
    pub dep_name: Option<String>,
    pub pr_hourly_limit: Option<i64>,
    pub branch_concurrent_limit: BranchConcurrentLimit,
    pub pr_concurrent_limit: Option<i64>,
    pub commit_hourly_limit: Option<i64>,
}

/// Branch config wrapper — only the `upgrades` slice is used here.
#[derive(Debug, Clone, Default)]
pub struct BranchConfig {
    pub upgrades: Vec<UpgradeConfig>,
}

/// Selects which limit field to read from an [`UpgradeConfig`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitName {
    PrHourlyLimit,
    BranchConcurrentLimit,
    PrConcurrentLimit,
    CommitHourlyLimit,
}

/// Resolve the effective limit value for one upgrade + one field.
///
/// Returns `None` for "undefined" (caller treats as `i64::MAX`).
fn upgrade_limit(u: &UpgradeConfig, name: LimitName) -> Option<i64> {
    match name {
        LimitName::PrHourlyLimit => u.pr_hourly_limit,
        LimitName::PrConcurrentLimit => u.pr_concurrent_limit,
        LimitName::CommitHourlyLimit => u.commit_hourly_limit,
        LimitName::BranchConcurrentLimit => match u.branch_concurrent_limit {
            BranchConcurrentLimit::Unset => None,
            BranchConcurrentLimit::Null => u.pr_concurrent_limit, // inherit
            BranchConcurrentLimit::Value(n) => Some(n),
        },
    }
}

/// Compute the lowest effective limit across all upgrades in a branch.
///
/// Returns `0` when any upgrade declares no limit (value 0 / null-inherited-as-0).
/// Returns `i64::MAX` if every upgrade leaves the field undefined.
///
/// Mirrors `calcLimit` from `lib/workers/global/limits.ts`.
pub fn calc_limit(upgrades: &[UpgradeConfig], name: LimitName) -> i64 {
    let mut lowest = i64::MAX;
    for u in upgrades {
        let limit = upgrade_limit(u, name).unwrap_or(i64::MAX);
        if limit == 0 {
            return 0;
        }
        if limit < lowest {
            lowest = limit;
        }
    }
    lowest
}

/// Return `true` when upgrades in a branch carry distinct non-undefined limit values.
///
/// Mirrors `hasMultipleLimits` from `lib/workers/global/limits.ts`.
pub fn has_multiple_limits(upgrades: &[UpgradeConfig], name: LimitName) -> bool {
    if upgrades.len() == 1 {
        return false;
    }
    let mut distinct: Vec<i64> = Vec::new();
    for u in upgrades {
        let Some(val) = upgrade_limit(u, name) else { continue };
        if !distinct.contains(&val) {
            distinct.push(val);
        }
    }
    distinct.len() > 1
}

fn handle_concurrent_limits(key: &str, config: &BranchConfig) -> bool {
    if key == "HourlyCommits" {
        let limit = calc_limit(&config.upgrades, LimitName::CommitHourlyLimit);
        let count = get_count("HourlyCommits");
        return limit != 0 && limit != i64::MAX && count >= limit;
    }

    let hourly_pr_limit = calc_limit(&config.upgrades, LimitName::PrHourlyLimit);
    let hourly_pr_count = get_count("HourlyPRs");
    if hourly_pr_limit != 0 && hourly_pr_limit != i64::MAX && hourly_pr_count >= hourly_pr_limit {
        return true;
    }

    let limit_name = if key == "Branches" {
        LimitName::BranchConcurrentLimit
    } else {
        LimitName::PrConcurrentLimit
    };
    let limit_val = calc_limit(&config.upgrades, limit_name);
    let current = get_count(key);
    limit_val != 0 && limit_val != i64::MAX && current >= limit_val
}

/// Check if a concurrent/hourly count-based limit is reached.
///
/// `key` is one of `"Branches"`, `"ConcurrentPRs"`, or `"HourlyCommits"`.
///
/// Mirrors the overloaded `isLimitReached` for those count names.
pub fn is_count_limit_reached(key: &str, config: &BranchConfig) -> bool {
    handle_concurrent_limits(key, config)
}

// ---------- tests ------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    fn clean<F: FnOnce()>(f: F) {
        let _g = TEST_MUTEX.lock().unwrap();
        reset_all_limits();
        f();
    }

    // Ported: "increments limited value" — workers/global/limits.spec.ts line 23
    #[test]
    fn increments_limited_value() {
        clean(|| {
            set_max_limit("Commits", Some(3));
            assert!(!is_commits_limit_reached());
            inc_limited_value("Commits", 2);
            assert!(!is_commits_limit_reached());
            inc_limited_value("Commits", 1);
            assert!(is_commits_limit_reached());
            inc_limited_value("Commits", 1);
            assert!(is_commits_limit_reached());
        });
    }

    // Ported: "defaults to unlimited" — workers/global/limits.spec.ts line 38
    #[test]
    fn defaults_to_unlimited() {
        clean(|| {
            assert!(!is_commits_limit_reached());
        });
    }

    // Ported: "increments undefined" — workers/global/limits.spec.ts line 42
    #[test]
    fn increments_undefined() {
        clean(|| {
            inc_limited_value("Commits", 1);
            assert!(!is_commits_limit_reached());
        });
    }

    // Ported: "resets counter" — workers/global/limits.spec.ts line 47
    #[test]
    fn resets_counter() {
        clean(|| {
            set_max_limit("Commits", Some(1));
            inc_limited_value("Commits", 1);
            assert!(is_commits_limit_reached());
            set_max_limit("Commits", Some(1)); // resets current to 0
            assert!(!is_commits_limit_reached());
        });
    }

    // Ported: "resets limit" — workers/global/limits.spec.ts line 55
    #[test]
    fn resets_limit() {
        clean(|| {
            set_max_limit("Commits", Some(1));
            inc_limited_value("Commits", 1);
            assert!(is_commits_limit_reached());
            set_max_limit("Commits", None); // null → unlimited
            assert!(!is_commits_limit_reached());
        });
    }

    // Ported: "sets non-positive limit as reached" — workers/global/limits.spec.ts line 63
    #[test]
    fn sets_non_positive_limit_as_reached() {
        clean(|| {
            set_max_limit("Commits", Some(0));
            assert!(is_commits_limit_reached());
            set_max_limit("Commits", Some(-1000));
            assert!(is_commits_limit_reached());
        });
    }

    fn single_upgrade(
        pr_hourly: Option<i64>,
        branch_conc: BranchConcurrentLimit,
        pr_conc: Option<i64>,
    ) -> Vec<UpgradeConfig> {
        vec![UpgradeConfig {
            pr_hourly_limit: pr_hourly,
            branch_concurrent_limit: branch_conc,
            pr_concurrent_limit: pr_conc,
            ..Default::default()
        }]
    }

    // Ported: "handles single upgrade" — workers/global/limits.spec.ts line 71
    #[test]
    fn calc_limit_handles_single_upgrade() {
        let upgrades = single_upgrade(
            Some(10),
            BranchConcurrentLimit::Value(11),
            Some(12),
        );
        assert_eq!(calc_limit(&upgrades, LimitName::PrHourlyLimit), 10);
        assert_eq!(calc_limit(&upgrades, LimitName::BranchConcurrentLimit), 11);
        assert_eq!(calc_limit(&upgrades, LimitName::PrConcurrentLimit), 12);
    }

    // Ported: "inherits prConcurrentLimit if branchConcurrentLimit is null" — workers/global/limits.spec.ts line 85
    #[test]
    fn calc_limit_inherits_pr_concurrent_when_branch_is_null() {
        let upgrades = single_upgrade(Some(10), BranchConcurrentLimit::Null, Some(12));
        assert_eq!(calc_limit(&upgrades, LimitName::PrHourlyLimit), 10);
        assert_eq!(calc_limit(&upgrades, LimitName::BranchConcurrentLimit), 12);
        assert_eq!(calc_limit(&upgrades, LimitName::PrConcurrentLimit), 12);
    }

    // Ported: "returns 0 if at least one upgrade has no limit in the branch" — workers/global/limits.spec.ts line 99
    #[test]
    fn calc_limit_returns_zero_when_any_upgrade_has_no_limit() {
        let upgrades = vec![
            UpgradeConfig {
                pr_hourly_limit: Some(10),
                branch_concurrent_limit: BranchConcurrentLimit::Value(11),
                pr_concurrent_limit: Some(12),
                ..Default::default()
            },
            UpgradeConfig {
                pr_hourly_limit: Some(0),
                branch_concurrent_limit: BranchConcurrentLimit::Value(0),
                pr_concurrent_limit: Some(0),
                ..Default::default()
            },
            UpgradeConfig {
                pr_hourly_limit: Some(1),
                branch_concurrent_limit: BranchConcurrentLimit::Value(1),
                pr_concurrent_limit: Some(1),
                ..Default::default()
            },
        ];
        assert_eq!(calc_limit(&upgrades, LimitName::PrHourlyLimit), 0);
        assert_eq!(calc_limit(&upgrades, LimitName::BranchConcurrentLimit), 0);
        assert_eq!(calc_limit(&upgrades, LimitName::PrConcurrentLimit), 0);
    }

    // Ported: "computes the lowest limit if multiple limits are present" — workers/global/limits.spec.ts line 123
    #[test]
    fn calc_limit_computes_lowest() {
        let upgrades = vec![
            UpgradeConfig {
                commit_hourly_limit: Some(5),
                pr_hourly_limit: Some(10),
                branch_concurrent_limit: BranchConcurrentLimit::Value(11),
                pr_concurrent_limit: Some(12),
                ..Default::default()
            },
            UpgradeConfig {
                pr_hourly_limit: Some(10),
                branch_concurrent_limit: BranchConcurrentLimit::Value(11),
                pr_concurrent_limit: Some(12),
                ..Default::default()
            },
            UpgradeConfig {
                commit_hourly_limit: Some(3),
                pr_hourly_limit: Some(1),
                branch_concurrent_limit: BranchConcurrentLimit::Value(1),
                pr_concurrent_limit: Some(1),
                ..Default::default()
            },
            UpgradeConfig {
                pr_hourly_limit: Some(5),
                branch_concurrent_limit: BranchConcurrentLimit::Value(6),
                pr_concurrent_limit: Some(3),
                ..Default::default()
            },
            UpgradeConfig {
                pr_hourly_limit: Some(5),
                branch_concurrent_limit: BranchConcurrentLimit::Null,
                pr_concurrent_limit: None, // undefined → MAX
                ..Default::default()
            },
            UpgradeConfig {
                pr_hourly_limit: Some(5),
                branch_concurrent_limit: BranchConcurrentLimit::Value(6),
                pr_concurrent_limit: Some(2),
                ..Default::default()
            },
        ];
        assert_eq!(calc_limit(&upgrades, LimitName::CommitHourlyLimit), 3);
        assert_eq!(calc_limit(&upgrades, LimitName::PrHourlyLimit), 1);
        assert_eq!(calc_limit(&upgrades, LimitName::BranchConcurrentLimit), 1);
        assert_eq!(calc_limit(&upgrades, LimitName::PrConcurrentLimit), 1);
    }

    // Ported: "de-duplicates upgrades by depName from debug log" — workers/global/limits.spec.ts line 165
    #[test]
    fn calc_limit_dedup_by_dep_name_return_value() {
        // Note: logger assertion (debug call) not ported — Rust has no mock logger.
        let upgrades = vec![
            UpgradeConfig {
                dep_name: Some("depA".into()),
                pr_hourly_limit: Some(10),
                ..Default::default()
            },
            UpgradeConfig {
                dep_name: Some("depA".into()),
                pr_hourly_limit: Some(10),
                ..Default::default()
            },
            UpgradeConfig {
                dep_name: Some("depB".into()),
                pr_hourly_limit: Some(1),
                ..Default::default()
            },
        ];
        assert_eq!(calc_limit(&upgrades, LimitName::PrHourlyLimit), 1);
    }

    // Ported: "handles single limit" — workers/global/limits.spec.ts line 195
    #[test]
    fn has_multiple_limits_single_upgrade() {
        let upgrades = single_upgrade(Some(10), BranchConcurrentLimit::Value(11), Some(12));
        assert!(!has_multiple_limits(&upgrades, LimitName::PrHourlyLimit));
        assert!(!has_multiple_limits(&upgrades, LimitName::BranchConcurrentLimit));
        assert!(!has_multiple_limits(&upgrades, LimitName::PrConcurrentLimit));
    }

    // Ported: "returns false if there are multiple limits with value" — workers/global/limits.spec.ts line 208
    #[test]
    fn has_multiple_limits_same_values() {
        let row = UpgradeConfig {
            pr_hourly_limit: Some(10),
            branch_concurrent_limit: BranchConcurrentLimit::Value(11),
            pr_concurrent_limit: Some(12),
            ..Default::default()
        };
        let upgrades = vec![row.clone(), row];
        assert!(!has_multiple_limits(&upgrades, LimitName::PrHourlyLimit));
        assert!(!has_multiple_limits(&upgrades, LimitName::BranchConcurrentLimit));
        assert!(!has_multiple_limits(&upgrades, LimitName::PrConcurrentLimit));
    }

    // Ported: "handles multiple limits" — workers/global/limits.spec.ts line 226
    #[test]
    fn has_multiple_limits_different_values() {
        let upgrades = vec![
            UpgradeConfig {
                pr_hourly_limit: Some(10),
                branch_concurrent_limit: BranchConcurrentLimit::Value(11),
                pr_concurrent_limit: Some(12),
                ..Default::default()
            },
            UpgradeConfig {
                pr_hourly_limit: Some(11),
                branch_concurrent_limit: BranchConcurrentLimit::Value(12),
                pr_concurrent_limit: Some(13),
                ..Default::default()
            },
            UpgradeConfig {
                pr_hourly_limit: Some(0),
                branch_concurrent_limit: BranchConcurrentLimit::Null,
                pr_concurrent_limit: Some(3),
                ..Default::default()
            },
        ];
        assert!(has_multiple_limits(&upgrades, LimitName::PrHourlyLimit));
        assert!(has_multiple_limits(&upgrades, LimitName::BranchConcurrentLimit));
        assert!(has_multiple_limits(&upgrades, LimitName::PrConcurrentLimit));
    }

    fn branch_config(upgrades: Vec<UpgradeConfig>) -> BranchConfig {
        BranchConfig { upgrades }
    }

    fn concurrent_upgrades() -> Vec<UpgradeConfig> {
        vec![
            UpgradeConfig {
                pr_hourly_limit: Some(10),
                branch_concurrent_limit: BranchConcurrentLimit::Value(11),
                pr_concurrent_limit: Some(12),
                ..Default::default()
            },
            UpgradeConfig {
                pr_hourly_limit: Some(11),
                branch_concurrent_limit: BranchConcurrentLimit::Value(12),
                pr_concurrent_limit: Some(13),
                ..Default::default()
            },
            UpgradeConfig {
                pr_hourly_limit: Some(0),
                branch_concurrent_limit: BranchConcurrentLimit::Null,
                pr_concurrent_limit: Some(3),
                ..Default::default()
            },
        ]
    }

    // Ported: "returns false based on concurrent limits" — workers/global/limits.spec.ts line 251
    #[test]
    fn is_limit_reached_returns_false_concurrent() {
        clean(|| {
            set_count("ConcurrentPRs", 1);
            set_count("HourlyPRs", 1);
            inc_count_value("Branches");
            let config = branch_config(concurrent_upgrades());
            assert!(!is_count_limit_reached("Branches", &config));
            assert!(!is_count_limit_reached("ConcurrentPRs", &config));
        });
    }

    // Ported: "returns true when pr hourly limit is reached" — workers/global/limits.spec.ts line 280
    #[test]
    fn is_limit_reached_true_pr_hourly() {
        clean(|| {
            set_count("Branches", 2);
            set_count("ConcurrentPRs", 2);
            set_count("HourlyPRs", 2);
            let upgrades = vec![
                UpgradeConfig {
                    pr_hourly_limit: Some(10),
                    branch_concurrent_limit: BranchConcurrentLimit::Value(11),
                    pr_concurrent_limit: Some(12),
                    ..Default::default()
                },
                UpgradeConfig {
                    pr_hourly_limit: Some(11),
                    branch_concurrent_limit: BranchConcurrentLimit::Value(12),
                    pr_concurrent_limit: Some(13),
                    ..Default::default()
                },
                UpgradeConfig {
                    pr_hourly_limit: Some(2),
                    branch_concurrent_limit: BranchConcurrentLimit::Null,
                    pr_concurrent_limit: Some(3),
                    ..Default::default()
                },
            ];
            let config = branch_config(upgrades);
            assert!(is_count_limit_reached("Branches", &config));
            assert!(is_count_limit_reached("ConcurrentPRs", &config));
        });
    }

    // Ported: "returns true when concurrent limit is reached" — workers/global/limits.spec.ts line 309
    #[test]
    fn is_limit_reached_true_concurrent() {
        clean(|| {
            set_count("Branches", 3);
            set_count("ConcurrentPRs", 3);
            set_count("HourlyPRs", 4);
            let upgrades = vec![
                UpgradeConfig {
                    pr_hourly_limit: Some(10),
                    branch_concurrent_limit: BranchConcurrentLimit::Value(11),
                    pr_concurrent_limit: Some(12),
                    ..Default::default()
                },
                UpgradeConfig {
                    pr_hourly_limit: Some(11),
                    branch_concurrent_limit: BranchConcurrentLimit::Value(12),
                    pr_concurrent_limit: Some(13),
                    ..Default::default()
                },
                UpgradeConfig {
                    pr_hourly_limit: Some(5),
                    branch_concurrent_limit: BranchConcurrentLimit::Null,
                    pr_concurrent_limit: Some(3),
                    ..Default::default()
                },
            ];
            let config = branch_config(upgrades);
            assert!(is_count_limit_reached("Branches", &config));
            assert!(is_count_limit_reached("ConcurrentPRs", &config));
        });
    }

    // Ported: "commit hourly limit only affects HourlyCommits check" — workers/global/limits.spec.ts line 338
    #[test]
    fn commit_hourly_limit_only_affects_hourly_commits() {
        clean(|| {
            set_count("HourlyCommits", 3);
            let mut upgrades = vec![UpgradeConfig {
                commit_hourly_limit: Some(3),
                ..Default::default()
            }];
            let config = BranchConfig { upgrades: upgrades.clone() };
            assert!(is_count_limit_reached("HourlyCommits", &config));

            set_count("HourlyCommits", 2);
            assert!(!is_count_limit_reached("HourlyCommits", &config));

            set_count("HourlyCommits", 100);
            upgrades[0].commit_hourly_limit = Some(0);
            let config2 = BranchConfig { upgrades };
            assert!(!is_count_limit_reached("HourlyCommits", &config2));
        });
    }

    // Ported: "commit hourly limit does not block branch or PR checks" — workers/global/limits.spec.ts line 362
    #[test]
    fn commit_hourly_limit_does_not_block_branch_or_pr() {
        clean(|| {
            set_count("Branches", 0);
            set_count("ConcurrentPRs", 1);
            set_count("HourlyCommits", 10);
            set_count("HourlyPRs", 0);
            let upgrades = vec![UpgradeConfig {
                commit_hourly_limit: Some(2),
                pr_hourly_limit: Some(10),
                branch_concurrent_limit: BranchConcurrentLimit::Value(10),
                pr_concurrent_limit: Some(0),
                ..Default::default()
            }];
            let config = branch_config(upgrades);
            assert!(!is_count_limit_reached("Branches", &config));
            assert!(!is_count_limit_reached("ConcurrentPRs", &config));
        });
    }
}
