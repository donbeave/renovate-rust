# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/extract-update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/extract-update.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** done

### `workers/repository/process/extract-update › extract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs with no baseBranchPatterns | 57 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |
| runs with baseBranchPatterns | 80 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |
| uses repository cache | 99 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |
| fetches vulnerabilities | 122 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |
| handles exception when fetching vulnerabilities | 141 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |

### `workers/repository/process/extract-update › extract() › malicious package detection › when using mocks`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips malicious package updates | 159 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |

### `workers/repository/process/extract-update › extract() › malicious package detection › when manually specifying the `skipReason`s › when skipReason=malicious-version-in-use`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs a warning | 259 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |
| deletes the skipReason and skipStage, to allow the update phase to continue updating | 313 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |
| when skipReason=malicious-version-in-use, it logs a warning for each skipReason | 361 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |

### `workers/repository/process/extract-update › isCacheExtractValid()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| undefined cache | 449 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |
| returns false if no revision | 454 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |
| returns false if revision mismatch | 460 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |
| partial cache | 466 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |
| sha mismatch | 471 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |
| config change | 481 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |
| invalid if no extractionFingerprints | 491 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |
| invalid if changed fingerprints | 508 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |
| valid cache and config | 515 | not-applicable | — | — | Requires vi.mock platform/scm/git/lookup mock infrastructure |

---
