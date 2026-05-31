# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/extract-update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/extract-update.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `workers/repository/process/extract-update › extract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs with no baseBranchPatterns  | 57 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |
| runs with baseBranchPatterns  | 80 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |
| uses repository cache  | 99 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |
| fetches vulnerabilities  | 122 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |
| handles exception when fetching vulnerabilities  | 141 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |

### `workers/repository/process/extract-update › extract() › malicious package detection › when using mocks`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips malicious package updates  | 159 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |

### `workers/repository/process/extract-update › extract() › malicious package detection › when manually specifying the `skipReason`s › when skipReason=malicious-version-in-use`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs a warning  | 259 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |
| deletes the skipReason and skipStage, to allow the update phase to continue updating  | 313 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |
| when skipReason=malicious-version-in-use, it logs a warning for each skipReason  | 361 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |

### `workers/repository/process/extract-update › isCacheExtractValid()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| undefined cache  | 449 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |
| returns false if no revision  | 454 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |
| returns false if revision mismatch  | 460 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |
| partial cache  | 466 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |
| sha mismatch  | 471 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |
| config change  | 481 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |
| invalid if no extractionFingerprints  | 491 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |
| invalid if changed fingerprints  | 508 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |
| valid cache and config  | 515 | not-applicable | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer | — | Mock framework internals — tests extract-update via vitest-mocked cache/datasource; Rust tests this at different layer |

---
