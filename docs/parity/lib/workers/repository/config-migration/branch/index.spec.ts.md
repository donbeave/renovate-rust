# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/config-migration/branch/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/config-migration/branch/index.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 12 | **Status:** pending-applicable-applicable

### `workers/repository/config-migration/branch/index › checkConfigMigrationBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing when migration disabled and checkbox unchecked  | 31 | pending | — | — | — |
| creates migration branch when migration disabled but checkbox checked  | 50 | pending | — | — | — |
| does not create a branch if migration branch is modified  | 71 | pending | — | — | — |
| updates migration branch & refreshes pr when migration disabled but open pr exists  | 102 | pending | — | — | — |
| creates migration branch when migration enabled but no pr exists  | 134 | pending | — | — | — |
| updates migration branch & refresh PR when migration enabled and open pr exists  | 157 | pending | — | — | — |
| Dry runs update migration branch  | 184 | pending | — | — | — |
| Dry runs create migration PR  | 209 | pending | — | — | — |

### `workers/repository/config-migration/branch/index › checkConfigMigrationBranch › handle closed PR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not create a branch when migration is disabled but needed and a closed pr exists  | 236 | pending | — | — | — |
| deletes old branch and creates new migration branch when migration is disabled but needed, a closed pr exists and checkbox is checked  | 255 | pending | — | — | — |
| does not create a branch when migration is enabled and a closed pr exists  | 280 | pending | — | — | — |
| dry run:deletes old branch and creates new migration branch when migration is disabled but needed, a closed pr exists and checkbox is checked  | 299 | pending | — | — | — |

---

