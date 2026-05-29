# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/extract-update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/extract-update.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** pending

### `workers/repository/process/extract-update › extract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs with no baseBranchPatterns | 57 | pending | — | — | —|
| runs with baseBranchPatterns | 80 | pending | — | — | —|
| uses repository cache | 99 | pending | — | — | —|
| fetches vulnerabilities | 122 | pending | — | — | —|
| handles exception when fetching vulnerabilities | 141 | pending | — | — | —|

### `workers/repository/process/extract-update › extract() › malicious package detection › when using mocks`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips malicious package updates | 159 | pending | — | — | —|

### `workers/repository/process/extract-update › extract() › malicious package detection › when manually specifying the `skipReason`s › when skipReason=malicious-version-in-use`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs a warning | 259 | pending | — | — | —|
| deletes the skipReason and skipStage, to allow the update phase to continue updating | 313 | pending | — | — | —|
| when skipReason=malicious-version-in-use, it logs a warning for each skipReason | 361 | pending | — | — | —|

### `workers/repository/process/extract-update › isCacheExtractValid()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| undefined cache | 449 | pending | — | — | —|
| returns false if no revision | 454 | pending | — | — | —|
| returns false if revision mismatch | 460 | pending | — | — | —|
| partial cache | 466 | pending | — | — | —|
| sha mismatch | 471 | pending | — | — | —|
| config change | 481 | pending | — | — | —|
| invalid if no extractionFingerprints | 491 | pending | — | — | —|
| invalid if changed fingerprints | 508 | pending | — | — | —|
| valid cache and config | 515 | pending | — | — | —|

---
