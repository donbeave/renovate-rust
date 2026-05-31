# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/pr/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/pr/index.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `workers/repository/onboarding/pr/index › ensureOnboardingPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if onboarded  | 47 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| returns if onboarded cache is valid  | 56 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| breaks early when onboarding  | 65 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| creates PR  | 86 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| creates semantic PR  | 91 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| creates PR with labels  | 108 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| creates PR with empty footer and header  | 125 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| creates PR with footer and header with trailing and leading newlines  | 149 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| creates PR with footer and header using templating  | 174 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| returns if PR does not need updating  | 208 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| ensures comment, when PR is conflicted  | 232 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |

### `workers/repository/onboarding/pr/index › ensureOnboardingPr() › when onboardingAutoCloseAge is set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ensures comment, if onboarding cache is up-to-date, but when onboarding pr is over onboardingAutoCloseAge  | 252 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| does not comment, when onboarding pr is exactly at onboardingAutoCloseAge  | 279 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| ensures comment, when onboarding pr is partially over onboardingAutoCloseAge  | 300 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| ensures comment, when onboarding pr is 1 day older than onboardingAutoCloseAge  | 327 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| ensures comment,when onboarding pr is significantly older than onboardingAutoCloseAge  | 354 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| prefers inherited onboardingAutoCloseAge over global config  | 376 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| does not allow inherited onboardingAutoCloseAge to be higher than global config  | 405 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| does nothing in dry run when PR is conflicted  | 435 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| updates PR when modified  | 454 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| creates PR (no require config)  | 467 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| creates PR (require config)  | 478 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |

### `workers/repository/onboarding/pr/index › ensureOnboardingPr() › the created PR references onboardingConfigFileName`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when set  | 485 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| when not set, falls back to "renovate.json"  | 496 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| when set, but not a valid filename, falls back to "renovate.json"  | 504 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| dryrun of creates PR  | 513 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| dryrun of updates PR  | 528 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |

### `workers/repository/onboarding/pr/index › ensureOnboardingPr() › ensureOnboardingPr() throws`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws when trying to create a new PR  | 559 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |
| deletes branch when PR already exists but cannot find it  | 567 | not-applicable | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer | — | Mock framework internals — tests onboarding PR via vitest-mocked platform/SCM; Rust tests this at different layer |

---
