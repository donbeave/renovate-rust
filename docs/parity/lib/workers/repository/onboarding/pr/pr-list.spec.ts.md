# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/pr/pr-list.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/pr/pr-list.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `workers/repository/onboarding/pr/pr-list › getExpectedPrList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty | 16 | ported | `util.rs` | `test_pr_list_empty` | — |
| has special lock file maintenance description | 28 | ported | `util.rs` | `test_pr_list_lock_file_maintenance` | — |
| handles multiple | 66 | ported | `util.rs` | `test_pr_list_multiple_with_limit` | — |
| shows commitHourlyLimit message when limit is low | 145 | ported | `util.rs` | `test_pr_list_commit_hourly_limit_low` | — |
| does not show commitHourlyLimit message when limit is high | 184 | ported | `util.rs` | `test_pr_list_commit_hourly_limit_high` | — |
| shows only commitHourlyLimit message when both limits are set | 206 | ported | `util.rs` | `test_pr_list_both_limits_commit_wins` | — |

---
