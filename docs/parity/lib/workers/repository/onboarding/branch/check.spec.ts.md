# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/branch/check.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/branch/check.spec.ts
**Total tests:** 11 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/onboarding/branch/check`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true if in silent mode | 31 | not-applicable | — | — | Onboarding branch check / platform integration |
| skips normal onboarding check if onboardingCache is valid | 36 | not-applicable | — | — | Onboarding branch check / platform integration |
| continues with normal logic if onboardingCache is invalid | 56 | not-applicable | — | — | Onboarding branch check / platform integration |
| continues with normal logic if closedPr exists - adds closing comment | 72 | not-applicable | — | — | Onboarding branch check / platform integration |

### `workers/repository/onboarding/branch/check › when closedPr exists and onboardingAutoCloseAge is set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds closing comment if exactly at onboardingAutoCloseAge | 97 | not-applicable | — | — | Onboarding branch check / platform integration |
| skips closing comment if onboarding pr is slightly older than onboardingAutoCloseAge | 119 | not-applicable | — | — | Onboarding branch check / platform integration |
| skips closing comment if onboarding pr is 1 day older than onboardingAutoCloseAge | 141 | not-applicable | — | — | Onboarding branch check / platform integration |
| skips closing comment if onboarding pr is significantly older than onboardingAutoCloseAge | 162 | not-applicable | — | — | Onboarding branch check / platform integration |
| prefers inherited onboardingAutoCloseAge over global config | 179 | not-applicable | — | — | Onboarding branch check / platform integration |
| does not allow inherited onboardingAutoCloseAge to be higher than global config | 203 | not-applicable | — | — | Onboarding branch check / platform integration |
| checks git file list for config file when in fork mode | 228 | not-applicable | — | — | Onboarding branch check / platform integration |

---

