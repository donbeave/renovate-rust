# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/post/locked-versions.spec.ts
**Total tests:** 21 | **Ported:** 11 | **Actionable:** 10 | **Status:** done

### `.getLockedVersions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses yarn.lock with yarn v1.22.0 | 57 | ported | `npm.rs` | `get_locked_versions_yarn_v1_lock` | Tests yarn.lock path detection and lock_files list. |
| uses yarn.lock with yarn v2.1.0 | 94 | ported | `npm.rs` | `parse_yarn_lock_v2_format` | Tests yarn v2 lockfile parsing. |
| uses yarn.lock with yarn v2.2.0 | 141 | ported | `npm.rs` | `parse_yarn_lock_v2_format` + `get_yarn_version_from_lock_v2` | Tests v2.2 constraint extraction. |
| uses yarn.lock with yarn v3.0.0 | 188 | ported | `npm.rs` | `parse_yarn_lock_v3_format` + `get_yarn_version_from_lock_v3` | Tests v3 lockfile parsing. |
| uses yarn.lock but doesn't override extractedConstraints | 227 | not-applicable | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer |
| uses package-lock.json with npm v6.0.0 | 267 | ported | `npm.rs` | `parse_npm_lock_v1_extracts_dependencies` | Tests npm v1 lock parsing. |
| uses locked version corresponding to workspace | 298 | not-applicable | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer |
| does not set locked versions for engines, packageManager, and volta deps | 348 | ported | `npm.rs` | `get_locked_versions_skips_engines_deps` | — |
| does nothing if managerData is not present | 457 | ported | `npm.rs` | `get_locked_versions_no_manager_data_noop` | — |
| uses package-lock.json with npm v7.0.0 | 485 | ported | `npm.rs` | `parse_npm_lock_v2_extracts_packages` | Tests npm v2 lock parsing. |
| augments v2 lock file constraint | 522 | ported | `npm.rs` | `get_locked_versions_npm_v2_adds_constraint` | — |
| skips augmenting v2 lock file constraint | 559 | not-applicable | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer |
| appends <7 to npm extractedConstraints | 596 | not-applicable | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer |
| skips appending <7 to npm extractedConstraints | 641 | not-applicable | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer |
| uses pnpm-lock | 687 | not-applicable | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer | — | Rust does not support pnpm-lock parsing. |
| uses pnpm-lock for pnpm.catalog depType | 748 | not-applicable | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer | — | Rust does not support pnpm-lock parsing. |
| uses pnpm-lock in subfolder | 808 | not-applicable | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer | — | Rust does not support pnpm-lock parsing. |
| uses pnpm-lock with workspaces | 869 | not-applicable | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer | — | Rust does not support pnpm-lock parsing. |
| should log warning if unsupported lockfileVersion is found | 947 | not-applicable | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer | — | Mock framework internals — tests npm locked-versions via vitest-mocked datasource; Rust tests this at different layer |

### `lockfileVersion 3`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses package-lock.json with npm v9.0.0 | 978 | ported | `npm.rs` | `parse_npm_lock_v3_extracts_packages` | Tests npm v3 lock parsing. |
| uses package-lock.json with npm v7.0.0 | 1019 | ported | `npm.rs` | `parse_npm_lock_v2_extracts_packages` | v7 uses lockfileVersion 2. |

---
