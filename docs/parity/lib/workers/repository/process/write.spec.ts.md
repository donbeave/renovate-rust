# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/write.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/write.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 16 | **Status:** pending

### `workers/repository/process/write › writeUpdates()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| stops after automerge | 48 | pending | — | — | — |
| increments branch counter | 106 | pending | — | — | — |
| return no-work if branch fingerprint is not different | 147 | pending | — | — | — |
| updates branch fingerprint when new commit is made | 176 | pending | — | — | — |
| caches same fingerprint when no commit is made and branch cache existed | 219 | pending | — | — | — |
| caches same fingerprint when no commit is made | 264 | pending | — | — | — |
| creates new branchCache when cache is not enabled | 306 | pending | — | — | — |

### `workers/repository/process/write › canSkipBranchUpdateCheck()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if no cache | 357 | pending | — | — | — |
| returns false when fingerprints are not same | 368 | pending | — | — | — |
| returns true | 378 | pending | — | — | — |

### `workers/repository/process/write › syncBranchState()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates minimal branch state when cache is not populated | 390 | pending | — | — | — |
| when base branch name is different updates it and invalidates related cache | 405 | pending | — | — | — |
| when base branch sha is different updates it and invalidates related values | 438 | pending | — | — | — |
| when branch sha is different updates it and invalidates related values | 473 | pending | — | — | — |
| when branch sha is different updates it and sets commitTimestamp | 509 | pending | — | — | — |
| no change if all parameters are same | 548 | pending | — | — | — |

---

