# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/extract/yarn.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/yarn.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `modules/manager/npm/extract/yarn › .getYarnLock()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty if exception parsing | 10 | ported | `npm.rs` | `yarn_lock_returns_empty_if_exception_parsing` | — |
| extracts yarn 1 | 17 | ported | `npm.rs` | `yarn_lock_extracts_yarn1_dependencies` | — |
| extracts yarn 2 | 27 | ported | `npm.rs` | `yarn_lock_extracts_yarn2_dependencies` | — |
| extracts yarn 2 cache version | 37 | ported | `npm.rs` | `yarn_lock_extracts_yarn2_cache_version` | — |
| ignores individual invalid entries | 47 | ported | `npm.rs` | `yarn_lock_ignores_individual_invalid_entries` | — |

### `modules/manager/npm/extract/yarn`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getYarnVersionFromLock | 58 | ported | `npm.rs` | `yarn_version_from_lock_matches_lockfile_version` | — |

### `modules/manager/npm/extract/yarn › .extractYarnCatalogs()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty catalog entries | 78 | ported | `npm.rs` | `yarn_catalogs_handles_empty_catalog_entries` | — |
| parses valid .yarnrc.yml file | 86 | ported | `npm.rs` | `yarn_catalogs_parses_valid_yarnrc_yml` | — |
| finds relevant lockfile | 130 | ported | `npm.rs` | `yarn_catalogs_finds_relevant_lockfile` | — |

---

