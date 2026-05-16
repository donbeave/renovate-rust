# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/pr/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/pr/index.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 29 | **Status:** pending

### `workers/repository/onboarding/pr/index › ensureOnboardingPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if onboarded | 47 | pending | — | — | — |
| returns if onboarded cache is valid | 56 | pending | — | — | — |
| breaks early when onboarding | 65 | pending | — | — | — |
| creates PR | 86 | pending | — | — | — |
| creates semantic PR | 91 | pending | — | — | — |
| creates PR with labels | 108 | pending | — | — | — |
| creates PR with empty footer and header | 125 | pending | — | — | — |
| creates PR with footer and header with trailing and leading newlines | 149 | pending | — | — | — |
| creates PR with footer and header using templating | 174 | pending | — | — | — |
| returns if PR does not need updating | 208 | pending | — | — | — |
| ensures comment, when PR is conflicted | 232 | pending | — | — | — |

### `workers/repository/onboarding/pr/index › ensureOnboardingPr() › when onboardingAutoCloseAge is set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ensures comment, if onboarding cache is up-to-date, but when onboarding pr is over onboardingAutoCloseAge | 252 | pending | — | — | — |
| does not comment, when onboarding pr is exactly at onboardingAutoCloseAge | 279 | pending | — | — | — |
| ensures comment, when onboarding pr is partially over onboardingAutoCloseAge | 300 | pending | — | — | — |
| ensures comment, when onboarding pr is 1 day older than onboardingAutoCloseAge | 327 | pending | — | — | — |
| ensures comment,when onboarding pr is significantly older than onboardingAutoCloseAge | 354 | pending | — | — | — |
| prefers inherited onboardingAutoCloseAge over global config | 376 | pending | — | — | — |
| does not allow inherited onboardingAutoCloseAge to be higher than global config | 405 | pending | — | — | — |
| does nothing in dry run when PR is conflicted | 435 | pending | — | — | — |
| updates PR when modified | 454 | pending | — | — | — |
| creates PR (no require config) | 467 | pending | — | — | — |
| creates PR (require config) | 478 | pending | — | — | — |

### `workers/repository/onboarding/pr/index › ensureOnboardingPr() › the created PR references onboardingConfigFileName`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when set | 485 | pending | — | — | — |
| when not set, falls back to "renovate.json" | 496 | pending | — | — | — |
| when set, but not a valid filename, falls back to "renovate.json" | 504 | pending | — | — | — |
| dryrun of creates PR | 513 | pending | — | — | — |
| dryrun of updates PR | 528 | pending | — | — | — |

### `workers/repository/onboarding/pr/index › ensureOnboardingPr() › ensureOnboardingPr() throws`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws when trying to create a new PR | 559 | pending | — | — | — |
| deletes branch when PR already exists but cannot find it | 567 | pending | — | — | — |

---

