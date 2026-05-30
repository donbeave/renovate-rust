# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/workers/repository/errors-warnings.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/errors-warnings.spec.ts
**Total tests:** 16 | **Ported:** 16 | **Actionable:** 0 | **Status:** done

### `workers/repository/errors-warnings › getWarnings()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns warning text | 22 | ported | `util.rs` | `test_get_warnings_returns_text` | — |
| getWarning returns empty string | 39 | ported | `util.rs` | `test_get_warnings_empty` | — |

### `workers/repository/errors-warnings › getDepWarningsPR()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns 2 pr warnings text dependencyDashboard true | 48 | ported | `util.rs` | `test_get_dep_warnings_pr_dashboard_true` | — |
| returns 2 pr warnings text dependencyDashboard true with issue link | 73 | ported | `util.rs` | `test_get_dep_warnings_pr_with_issue_link` | — |
| returns 2 pr warnings text dependencyDashboard false | 97 | ported | `util.rs` | `test_get_dep_warnings_pr_dashboard_false` | — |
| PR warning returns empty string | 137 | ported | `util.rs` | `test_get_dep_warnings_pr_empty` | — |
| suppress notifications contains dependencyLookupWarnings flag then return empty string | 144 | ported | `util.rs` | `test_get_dep_warnings_pr_suppressed` | — |

### `workers/repository/errors-warnings › getDepWarningsDashboard()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns dependency dashboard warning text | 153 | ported | `util.rs` | `test_get_dep_warnings_dashboard_returns_text` | — |
| dependency dashboard warning returns empty string | 203 | ported | `util.rs` | `test_get_dep_warnings_dashboard_empty` | — |
| suppress notifications contains dependencyLookupWarnings flag then return empty string | 210 | ported | `util.rs` | `test_get_dep_warnings_dashboard_suppressed` | — |

### `workers/repository/errors-warnings › getErrors()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns error text | 318 | ported | `util.rs` | `test_get_errors_returns_text` | — |
| getError returns empty string | 335 | ported | `util.rs` | `test_get_errors_empty` | — |

### `workers/repository/errors-warnings › getDepWarningsOnboardingPR()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns onboarding warning text | 218 | ported | `util.rs` | `test_get_dep_warnings_onboarding_pr_returns_text` | — |
| handle empty package files | 273 | ported | `util.rs` | `test_get_dep_warnings_onboarding_empty` | — |
| suppress notifications contains dependencyLookupWarnings flag then return empty string | 284 | ported | `util.rs` | `test_get_dep_warnings_onboarding_suppressed` | — |
| handles undefined | 290 | ported | `util.rs` | `test_get_dep_warnings_onboarding_handles_undefined` | — |

---
