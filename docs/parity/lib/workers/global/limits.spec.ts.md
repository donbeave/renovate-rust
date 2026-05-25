# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/workers/global/limits.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/limits.spec.ts
**Total tests:** 19 | **Ported:** 19 | **Actionable:** 19 | **Status:** ported

### `workers/global/limits`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| increments limited value | 23 | ported | limits.rs | increments_limited_value | — |
| defaults to unlimited | 38 | ported | limits.rs | defaults_to_unlimited | — |
| increments undefined | 42 | ported | limits.rs | increments_undefined | — |
| resets counter | 47 | ported | limits.rs | resets_counter | — |
| resets limit | 55 | ported | limits.rs | resets_limit | — |
| sets non-positive limit as reached | 63 | ported | limits.rs | sets_non_positive_limit_as_reached | — |

### `workers/global/limits › calcLimit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles single upgrade | 71 | ported | limits.rs | calc_limit_handles_single_upgrade | — |
| inherits prConcurrentLimit if branchConcurrentLimit is null | 85 | ported | limits.rs | calc_limit_inherits_pr_concurrent_when_branch_is_null | — |
| returns 0 if at least one upgrade has no limit in the branch | 99 | ported | limits.rs | calc_limit_returns_zero_when_any_upgrade_has_no_limit | — |
| computes the lowest limit if multiple limits are present | 123 | ported | limits.rs | calc_limit_computes_lowest | — |
| de-duplicates upgrades by depName from debug log | 165 | ported | limits.rs | calc_limit_dedup_by_dep_name_return_value | Return value ported; logger.debug assertion not ported (no mock logger in Rust). |

### `workers/global/limits › hasMultipleLimits`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles single limit | 195 | ported | limits.rs | has_multiple_limits_single_upgrade | — |
| returns false if there are multiple limits with value | 208 | ported | limits.rs | has_multiple_limits_same_values | — |
| handles multiple limits | 226 | ported | limits.rs | has_multiple_limits_different_values | — |

### `workers/global/limits › isLimitReached`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false based on concurrent limits | 251 | ported | limits.rs | is_limit_reached_returns_false_concurrent | — |
| returns true when pr hourly limit is reached | 280 | ported | limits.rs | is_limit_reached_true_pr_hourly | — |
| returns true when concurrent limit is reached | 309 | ported | limits.rs | is_limit_reached_true_concurrent | — |
| commit hourly limit only affects HourlyCommits check | 338 | ported | limits.rs | commit_hourly_limit_only_affects_hourly_commits | — |
| commit hourly limit does not block branch or PR checks | 362 | ported | limits.rs | commit_hourly_limit_does_not_block_branch_or_pr | — |

---

