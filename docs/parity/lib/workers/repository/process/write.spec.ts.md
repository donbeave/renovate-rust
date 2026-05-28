# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/write.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/write.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 16 | **Status:** done

### `workers/repository/process/write › writeUpdates()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| stops after automerge | 48 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |
| increments branch counter | 106 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |
| return no-work if branch fingerprint is not different | 147 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |
| updates branch fingerprint when new commit is made | 176 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |
| caches same fingerprint when no commit is made and branch cache existed | 219 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |
| caches same fingerprint when no commit is made | 264 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |
| creates new branchCache when cache is not enabled | 306 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |

### `workers/repository/process/write › canSkipBranchUpdateCheck()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if no cache | 357 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |
| returns false when fingerprints are not same | 368 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |
| returns true | 378 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |

### `workers/repository/process/write › syncBranchState()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates minimal branch state when cache is not populated | 390 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |
| when base branch name is different updates it and invalidates related cache | 405 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |
| when base branch sha is different updates it and invalidates related values | 438 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |
| when branch sha is different updates it and invalidates related values | 473 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |
| when branch sha is different updates it and sets commitTimestamp | 509 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |
| no change if all parameters are same | 548 | not-applicable | — | — | Requires vi.mock platform/scm/git mock infrastructure |

---

