# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/sort.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/sort.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `workers/repository/process/sort › sortBranches()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sorts based on updateType and prTitle | 6 | ported | `branch.rs` | `sort_branches_by_update_type_and_pr_title` | — |
| sorts based on prPriority | 49 | ported | `branch.rs` | `sort_branches_by_pr_priority` | — |
| sorts based on isVulnerabilityAlert | 86 | ported | `branch.rs` | `sort_branches_vulnerability_alert_first` | — |
| sorts based on isVulnerabilityAlert symmetric | 124 | ported | `branch.rs` | `sort_branches_vulnerability_alert_symmetric` | — |

---

