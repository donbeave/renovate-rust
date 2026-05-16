# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/config-migration/branch/migrated-data.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/config-migration/branch/migrated-data.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 19 | **Status:** pending

### `workers/repository/config-migration/branch/migrated-data › MigratedDataFactory.getAsync`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Calls getAsync a first when migration not needed | 54 | pending | — | — | — |
| Calls getAsync a first time to initialize the factory | 62 | pending | — | — | — |
| Calls getAsync a second time to get the saved data from before | 69 | pending | — | — | — |

### `workers/repository/config-migration/branch/migrated-data › MigratedDataFactory.getAsync › MigratedData class`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets the filename from the class instance | 77 | pending | — | — | — |
| gets the content from the class instance | 82 | pending | — | — | — |
| Resets the factory and gets a new value | 88 | pending | — | — | — |
| Resets the factory and gets a new value with default indentation | 95 | pending | — | — | — |
| Migrate a JSON5 config file | 110 | pending | — | — | — |
| Falls back to JSON.stringify when weave fails | 120 | pending | — | — | — |
| Uses JSON.stringify when raw is null | 138 | pending | — | — | — |
| Returns nothing due to detectRepoFileConfig throwing | 150 | pending | — | — | — |

### `workers/repository/config-migration/branch/migrated-data › MigratedDataFactory.applyPrettierFormatting`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not format when no prettier config is present | 184 | pending | — | — | — |
| does not format when failing to fetch package.json file | 193 | pending | — | — | — |
| does not format when there is an invalid package.json file | 202 | pending | — | — | — |
| formats when prettier config file is found | 211 | pending | — | — | — |
| formats without prettier if in .renovaterc | 220 | pending | — | — | — |
| formats when finds prettier config inside the package.json file | 231 | pending | — | — | — |
| formats with default 2 spaces | 243 | pending | — | — | — |
| formats with printWith=Infinity | 259 | pending | — | — | — |

---

