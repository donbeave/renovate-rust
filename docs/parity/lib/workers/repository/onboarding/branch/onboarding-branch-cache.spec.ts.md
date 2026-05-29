# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/branch/onboarding-branch-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/branch/onboarding-branch-cache.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 20 | **Status:** not-applicable

### `workers/repository/onboarding/branch/onboarding-branch-cache › setOnboardingCache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not create new cache | 24 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| sets new cache | 31 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| updates old cache | 45 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|

### `workers/repository/onboarding/branch/onboarding-branch-cache › deleteOnboardingCache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes cache | 68 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|

### `workers/repository/onboarding/branch/onboarding-branch-cache › hasOnboardingBranchChanged()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return true if cache is absent | 84 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| returns true | 92 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| returns false | 108 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| returns false when branch is modified but has not changed since last run | 124 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|

### `workers/repository/onboarding/branch/onboarding-branch-cache › isOnboardingBranchModified()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| falls back to git if cache is absent | 142 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| falls back to git if onboarding branch is updated | 153 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| returns cached value | 172 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|

### `workers/repository/onboarding/branch/onboarding-branch-cache › isOnboardingBranchConflicted()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| falls back to git if cache is absent | 192 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| falls back to git if default branch is updated | 203 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| falls back to git if onboarding branch is modified | 222 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| returns cached value | 241 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|

### `workers/repository/onboarding/branch/onboarding-branch-cache › getOnboardingFileNameFromCache()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns cached value | 261 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| returns undefined | 271 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|

### `workers/repository/onboarding/branch/onboarding-branch-cache › getOnboardingConfigFromCache()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns cached value | 278 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| returns undefined | 288 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|

### `workers/repository/onboarding/branch/onboarding-branch-cache › setOnboardingConfigDetails()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns cached value | 295 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|

---
