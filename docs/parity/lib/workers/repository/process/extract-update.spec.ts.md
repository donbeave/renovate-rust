# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/extract-update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/extract-update.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** not-applicable

### `workers/repository/process/extract-update › extract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs with no baseBranchPatterns | 57 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| runs with baseBranchPatterns | 80 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| uses repository cache | 99 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| fetches vulnerabilities | 122 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| handles exception when fetching vulnerabilities | 141 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|

### `workers/repository/process/extract-update › extract() › malicious package detection › when using mocks`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips malicious package updates | 159 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|

### `workers/repository/process/extract-update › extract() › malicious package detection › when manually specifying the `skipReason`s › when skipReason=malicious-version-in-use`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs a warning | 259 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| deletes the skipReason and skipStage, to allow the update phase to continue updating | 313 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| when skipReason=malicious-version-in-use, it logs a warning for each skipReason | 361 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|

### `workers/repository/process/extract-update › isCacheExtractValid()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| undefined cache | 449 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| returns false if no revision | 454 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| returns false if revision mismatch | 460 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| partial cache | 466 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| sha mismatch | 471 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| config change | 481 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| invalid if no extractionFingerprints | 491 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| invalid if changed fingerprints | 508 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|
| valid cache and config | 515 | not-applicable | — | — | mocking framework internals — platform/git/scm/fs mock utilities; TypeScript platform integration pipeline|

---
