# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/config-migration/branch/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/config-migration/branch/index.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `workers/repository/config-migration/branch/index › checkConfigMigrationBranch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing when migration disabled and checkbox unchecked  | 31 | not-applicable | — | — | Mock framework internals — tests TS-specific checkConfigMigrationBranch via vitest-mocked platform, SCM, and git modules |
| creates migration branch when migration disabled but checkbox checked  | 50 | not-applicable | — | — | Mock framework internals — tests TS-specific checkConfigMigrationBranch via vitest-mocked platform, SCM, and git modules |
| does not create a branch if migration branch is modified  | 71 | not-applicable | — | — | Mock framework internals — tests TS-specific checkConfigMigrationBranch via vitest-mocked platform, SCM, and git modules |
| updates migration branch & refreshes pr when migration disabled but open pr exists  | 102 | not-applicable | — | — | Mock framework internals — tests TS-specific checkConfigMigrationBranch via vitest-mocked platform, SCM, and git modules |
| creates migration branch when migration enabled but no pr exists  | 134 | not-applicable | — | — | Mock framework internals — tests TS-specific checkConfigMigrationBranch via vitest-mocked platform, SCM, and git modules |
| updates migration branch & refresh PR when migration enabled and open pr exists  | 157 | not-applicable | — | — | Mock framework internals — tests TS-specific checkConfigMigrationBranch via vitest-mocked platform, SCM, and git modules |
| Dry runs update migration branch  | 184 | not-applicable | — | — | Mock framework internals — tests TS-specific checkConfigMigrationBranch via vitest-mocked platform, SCM, and git modules |
| Dry runs create migration PR  | 209 | not-applicable | — | — | Mock framework internals — tests TS-specific checkConfigMigrationBranch via vitest-mocked platform, SCM, and git modules |

### `workers/repository/config-migration/branch/index › checkConfigMigrationBranch › handle closed PR`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not create a branch when migration is disabled but needed and a closed pr exists  | 236 | not-applicable | — | — | Mock framework internals — tests TS-specific checkConfigMigrationBranch via vitest-mocked platform, SCM, and git modules |
| deletes old branch and creates new migration branch when migration is disabled but needed, a closed pr exists and checkbox is checked  | 255 | not-applicable | — | — | Mock framework internals — tests TS-specific checkConfigMigrationBranch via vitest-mocked platform, SCM, and git modules |
| does not create a branch when migration is enabled and a closed pr exists  | 280 | not-applicable | — | — | Mock framework internals — tests TS-specific checkConfigMigrationBranch via vitest-mocked platform, SCM, and git modules |
| dry run:deletes old branch and creates new migration branch when migration is disabled but needed, a closed pr exists and checkbox is checked  | 299 | not-applicable | — | — | Mock framework internals — tests TS-specific checkConfigMigrationBranch via vitest-mocked platform, SCM, and git modules |

---

