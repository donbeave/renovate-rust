# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bazel-module/lockfile.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel-module/lockfile.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/manager/bazel-module/lockfile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns updated lockfile when modified | 22 | not-applicable | — | — | Exercises Renovate `updateBazelLockfile()` artifact workflow with `bazel mod deps`, git status, and filesystem writes; Rust has no Bazel lockfile updater |
| returns updated lockfile when in not_added | 54 | not-applicable | — | — | Exercises Renovate `updateBazelLockfile()` artifact workflow with `bazel mod deps`, git status, and filesystem writes; Rust has no Bazel lockfile updater |
| returns null when lockfile is not modified | 83 | not-applicable | — | — | Exercises Renovate `updateBazelLockfile()` artifact workflow with `bazel mod deps` and git status; Rust has no Bazel lockfile updater |
| deletes lockfile during maintenance | 105 | not-applicable | — | — | Exercises Renovate lockfile maintenance deletion before `bazel mod deps`; Rust has no Bazel lockfile updater |
| does not delete lockfile when not in maintenance | 137 | not-applicable | — | — | Exercises Renovate lockfile maintenance deletion control; Rust has no Bazel lockfile updater |
| re-throws TEMPORARY_ERROR | 154 | not-applicable | — | — | Exercises Renovate artifact execution error handling; Rust has no Bazel lockfile updater |
| returns artifactError on exec failure | 168 | not-applicable | — | — | Exercises Renovate artifact execution error reporting; Rust has no Bazel lockfile updater |
| returns null when bazelModDeps is not allowed | 190 | not-applicable | — | — | Exercises Renovate unsafe execution policy for `bazel mod deps`; Rust has no Bazel lockfile updater |

---

