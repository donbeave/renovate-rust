# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/workers/repository/errors-warnings.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/errors-warnings.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 16 | **Status:** not-applicable

### `workers/repository/errors-warnings › getWarnings()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns warning text | 20 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |
| getWarning returns empty string | 41 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |

### `workers/repository/errors-warnings › getDepWarningsPR()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns 2 pr warnings text dependencyDashboard true | 49 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |
| returns 2 pr warnings text dependencyDashboard true with issue link | 97 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |
| returns 2 pr warnings text dependencyDashboard false | 120 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |
| PR warning returns empty string | 168 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |
| suppress notifications contains dependencyLookupWarnings flag then return empty string | 175 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |

### `workers/repository/errors-warnings › getDepWarningsDashboard()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns dependency dashboard warning text | 186 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |
| dependency dashboard warning returns empty string | 236 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |
| suppress notifications contains dependencyLookupWarnings flag then return empty string | 243 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |

### `workers/repository/errors-warnings › getErrors()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns error text | 260 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |
| getError returns empty string | 281 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |

### `workers/repository/errors-warnings › getDepWarningsOnboardingPR()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns onboarding warning text | 289 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |
| handle empty package files | 345 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |
| suppress notifications contains dependencyLookupWarnings flag then return empty string | 356 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |
| handles undefined | 365 | not-applicable | — | — | tests error/warning aggregation tied to TypeScript config infrastructure |

---

