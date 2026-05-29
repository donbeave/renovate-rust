# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/pr/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/pr/index.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 29 | **Status:** not-applicable

### `workers/repository/onboarding/pr/index › ensureOnboardingPr()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if onboarded | 47 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| returns if onboarded cache is valid | 56 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| breaks early when onboarding | 65 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| creates PR | 86 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| creates semantic PR | 91 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| creates PR with labels | 108 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| creates PR with empty footer and header | 125 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| creates PR with footer and header with trailing and leading newlines | 149 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| creates PR with footer and header using templating | 174 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| returns if PR does not need updating | 208 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| ensures comment, when PR is conflicted | 232 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|

### `workers/repository/onboarding/pr/index › ensureOnboardingPr() › when onboardingAutoCloseAge is set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ensures comment, if onboarding cache is up-to-date, but when onboarding pr is over onboardingAutoCloseAge | 252 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| does not comment, when onboarding pr is exactly at onboardingAutoCloseAge | 279 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| ensures comment, when onboarding pr is partially over onboardingAutoCloseAge | 300 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| ensures comment, when onboarding pr is 1 day older than onboardingAutoCloseAge | 327 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| ensures comment,when onboarding pr is significantly older than onboardingAutoCloseAge | 354 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| prefers inherited onboardingAutoCloseAge over global config | 376 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| does not allow inherited onboardingAutoCloseAge to be higher than global config | 405 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| does nothing in dry run when PR is conflicted | 435 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| updates PR when modified | 454 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| creates PR (no require config) | 467 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| creates PR (require config) | 478 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|

### `workers/repository/onboarding/pr/index › ensureOnboardingPr() › the created PR references onboardingConfigFileName`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when set | 485 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| when not set, falls back to "renovate.json" | 496 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| when set, but not a valid filename, falls back to "renovate.json" | 504 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| dryrun of creates PR | 513 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| dryrun of updates PR | 528 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|

### `workers/repository/onboarding/pr/index › ensureOnboardingPr() › ensureOnboardingPr() throws`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws when trying to create a new PR | 559 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|
| deletes branch when PR already exists but cannot find it | 567 | not-applicable | — | — | mocking framework internals — platform/git/scm mock utilities; TypeScript platform integration pipeline|

---
