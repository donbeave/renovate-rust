# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/devbox/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/devbox/artifacts.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 15 | **Status:** pending-applicable

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips if no updatedDeps and no lockFileMaintenance | 38 | pending | — | — | — |
| skips if no lock file in config | 42 | pending | — | — | — |
| skips if cannot read lock file | 47 | pending | — | — | — |
| returns installed devbox.lock | 54 | pending | — | — | — |
| calls install instead of update --no-install if an older version of devbox is constrained | 100 | pending | — | — | — |
| returns installed devbox.lock with multiple updated deps | 150 | pending | — | — | — |
| returns null if no updatedDeps are passed | 213 | pending | — | — | — |
| returns null if no updatedDeps have depNames | 230 | pending | — | — | — |
| returns updated devbox.lock | 252 | pending | — | — | — |
| calls update without --no-install flag if an older version of devbox is being used | 299 | pending | — | — | — |
| returns null if no changes are found | 349 | pending | — | — | — |
| returns null if devbox.lock not found after update | 368 | pending | — | — | — |
| returns null if devbox.lock not found | 395 | pending | — | — | — |
| returns null if no lock file changes are found | 421 | pending | — | — | — |
| returns an artifact error on failure | 449 | pending | — | — | — |

---

