# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/devbox/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/devbox/artifacts.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips if no updatedDeps and no lockFileMaintenance | 38 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| skips if no lock file in config | 42 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| skips if cannot read lock file | 47 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns installed devbox.lock | 54 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| calls install instead of update --no-install if an older version of devbox is constrained | 100 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns installed devbox.lock with multiple updated deps | 150 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if no updatedDeps are passed | 213 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if no updatedDeps have depNames | 230 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated devbox.lock | 252 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| calls update without --no-install flag if an older version of devbox is being used | 299 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if no changes are found | 349 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if devbox.lock not found after update | 368 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if devbox.lock not found | 395 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if no lock file changes are found | 421 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns an artifact error on failure | 449 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---

