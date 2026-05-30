# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/branch/check.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/branch/check.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 11 | **Status:** not-applicable

### `workers/repository/onboarding/branch/check`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true if in silent mode | 31 | pending | — | — | — |
| skips normal onboarding check if onboardingCache is valid | 36 | pending | — | — | — |
| continues with normal logic if onboardingCache is invalid | 56 | pending | — | — | — |
| continues with normal logic if closedPr exists - adds closing comment | 72 | pending | — | — | — |

### `workers/repository/onboarding/branch/check › when closedPr exists and onboardingAutoCloseAge is set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds closing comment if exactly at onboardingAutoCloseAge | 97 | pending | — | — | — |
| skips closing comment if onboarding pr is slightly older than onboardingAutoCloseAge | 119 | pending | — | — | — |
| skips closing comment if onboarding pr is 1 day older than onboardingAutoCloseAge | 141 | pending | — | — | — |
| skips closing comment if onboarding pr is significantly older than onboardingAutoCloseAge | 162 | pending | — | — | — |
| prefers inherited onboardingAutoCloseAge over global config | 179 | pending | — | — | — |
| does not allow inherited onboardingAutoCloseAge to be higher than global config | 203 | pending | — | — | — |
| checks git file list for config file when in fork mode | 228 | pending | — | — | — |

---

