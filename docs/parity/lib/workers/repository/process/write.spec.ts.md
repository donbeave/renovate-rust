# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/write.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/write.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `workers/repository/process/write › writeUpdates()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| stops after automerge  | 48 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |
| increments branch counter  | 106 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |
| return no-work if branch fingerprint is not different  | 147 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |
| updates branch fingerprint when new commit is made  | 176 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |
| caches same fingerprint when no commit is made and branch cache existed  | 219 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |
| caches same fingerprint when no commit is made  | 264 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |
| creates new branchCache when cache is not enabled  | 306 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |

### `workers/repository/process/write › canSkipBranchUpdateCheck()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if no cache  | 357 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |
| returns false when fingerprints are not same  | 368 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |
| returns true  | 378 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |

### `workers/repository/process/write › syncBranchState()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates minimal branch state when cache is not populated  | 390 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |
| when base branch name is different updates it and invalidates related cache  | 405 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |
| when base branch sha is different updates it and invalidates related values  | 438 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |
| when branch sha is different updates it and invalidates related values  | 473 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |
| when branch sha is different updates it and sets commitTimestamp  | 509 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |
| no change if all parameters are same  | 548 | not-applicable | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer | — | Mock framework internals — tests process write via vitest-mocked cache/limits/branch; Rust tests this at different layer |

---

