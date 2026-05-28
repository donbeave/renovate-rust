# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bazelisk/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazelisk/artifacts.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 6 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no updated deps and not lockfile maintenance | 24 | not-applicable | — | — | Uses vi.mock fs + vi.mock lockfile; filesystem/module mock infrastructure not portable |
| returns null if no MODULE.bazel found | 35 | not-applicable | — | — | Uses vi.mock fs + vi.mock lockfile; filesystem/module mock infrastructure not portable |
| returns null if no MODULE.bazel.lock found | 49 | not-applicable | — | — | Uses vi.mock fs + vi.mock lockfile; filesystem/module mock infrastructure not portable |
| writes package file and delegates to updateBazelLockfile | 65 | not-applicable | — | — | Uses vi.mock fs + vi.mock lockfile; filesystem/module mock infrastructure not portable |
| passes bazelisk constraint to updateBazelLockfile | 106 | not-applicable | — | — | Uses vi.mock fs + vi.mock lockfile; filesystem/module mock infrastructure not portable |
| passes isLockFileMaintenance to updateBazelLockfile | 129 | not-applicable | — | — | Uses vi.mock fs + vi.mock lockfile; filesystem/module mock infrastructure not portable |

---

