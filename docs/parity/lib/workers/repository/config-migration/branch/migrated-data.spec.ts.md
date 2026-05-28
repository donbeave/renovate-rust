# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/config-migration/branch/migrated-data.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/config-migration/branch/migrated-data.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 19 | **Status:** done

### `workers/repository/config-migration/branch/migrated-data › MigratedDataFactory.getAsync`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Calls getAsync a first when migration not needed | 54 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| Calls getAsync a first time to initialize the factory | 62 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| Calls getAsync a second time to get the saved data from before | 69 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |

### `workers/repository/config-migration/branch/migrated-data › MigratedDataFactory.getAsync › MigratedData class`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets the filename from the class instance | 77 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| gets the content from the class instance | 82 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| Resets the factory and gets a new value | 88 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| Resets the factory and gets a new value with default indentation | 95 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| Migrate a JSON5 config file | 110 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| Falls back to JSON.stringify when weave fails | 120 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| Uses JSON.stringify when raw is null | 138 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| Returns nothing due to detectRepoFileConfig throwing | 150 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |

### `workers/repository/config-migration/branch/migrated-data › MigratedDataFactory.applyPrettierFormatting`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not format when no prettier config is present | 184 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| does not format when failing to fetch package.json file | 193 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| does not format when there is an invalid package.json file | 202 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| formats when prettier config file is found | 211 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| formats without prettier if in .renovaterc | 220 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| formats when finds prettier config inside the package.json file | 231 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| formats with default 2 spaces | 243 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |
| formats with printWith=Infinity | 259 | not-applicable | — | — | Requires vi.mock fs/git/platform mock infrastructure |

---
