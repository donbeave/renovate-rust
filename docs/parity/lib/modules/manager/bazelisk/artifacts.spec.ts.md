# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bazelisk/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazelisk/artifacts.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no updated deps and not lockfile maintenance | 24 | not-applicable | — | — | Subprocess artifact generation mocking |
| returns null if no MODULE.bazel found | 35 | not-applicable | — | — | Subprocess artifact generation mocking |
| returns null if no MODULE.bazel.lock found | 49 | not-applicable | — | — | Subprocess artifact generation mocking |
| writes package file and delegates to updateBazelLockfile | 65 | not-applicable | — | — | Subprocess artifact generation mocking |
| passes bazelisk constraint to updateBazelLockfile | 106 | not-applicable | — | — | Subprocess artifact generation mocking |
| passes isLockFileMaintenance to updateBazelLockfile | 129 | not-applicable | — | — | Subprocess artifact generation mocking |

---
