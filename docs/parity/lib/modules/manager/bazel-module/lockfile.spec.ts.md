# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bazel-module/lockfile.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/lockfile.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable

### `modules/manager/bazel-module/lockfile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns updated lockfile when modified | 22 | not-applicable | — | — | Subprocess / git mocking |
| returns updated lockfile when in not_added | 54 | not-applicable | — | — | Subprocess / git mocking |
| returns null when lockfile is not modified | 83 | not-applicable | — | — | Subprocess / git mocking |
| deletes lockfile during maintenance | 105 | not-applicable | — | — | Subprocess / git mocking |
| does not delete lockfile when not in maintenance | 137 | not-applicable | — | — | Subprocess / git mocking |
| re-throws TEMPORARY_ERROR | 154 | not-applicable | — | — | Subprocess / git mocking |
| returns artifactError on exec failure | 168 | not-applicable | — | — | Subprocess / git mocking |
| returns null when bazelModDeps is not allowed | 190 | not-applicable | — | — | Subprocess / git mocking |

---

