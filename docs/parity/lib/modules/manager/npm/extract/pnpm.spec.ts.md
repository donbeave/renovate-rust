# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/extract/pnpm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/pnpm.spec.ts
**Total tests:** 16 | **Ported:** 5 | **Actionable:** 16 | **Status:** done

### `modules/manager/npm/extract/pnpm › .extractPnpmFilters()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects errors in pnpm-workspace.yml file structure | 28 | not-applicable | — | — | Uses vi.mock fs (readLocalFile mock) + logger spy; fs/logger mock infrastructure not portable to Rust |
| detects errors when opening pnpm-workspace.yml file | 46 | not-applicable | — | — | Uses vi.spyOn(yaml) + logger spy; module mock infrastructure not portable to Rust |

### `modules/manager/npm/extract/pnpm › .findPnpmWorkspace()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects missing pnpm-workspace.yaml | 65 | not-applicable | — | — | Uses fs.findLocalSiblingOrParent mock; fs mock infrastructure not portable to Rust |
| detects missing pnpm-lock.yaml when pnpm-workspace.yaml was already found | 78 | not-applicable | — | — | Uses fs.findLocalSiblingOrParent mock; fs mock infrastructure not portable to Rust |

### `modules/manager/npm/extract/pnpm › .detectPnpmWorkspaces()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses pnpm workspaces | 114 | not-applicable | — | — | Uses fs.localPathExists mock + GlobalConfig + fixture data; fs/GlobalConfig mock infrastructure not portable to Rust |
| skips when pnpm shrinkwrap file has already been provided | 203 | not-applicable | — | — | Uses fs.localPathExists mock + GlobalConfig + fixture data; fs/GlobalConfig mock infrastructure not portable to Rust |
| filters none matching packages | 220 | not-applicable | — | — | Uses fs.localPathExists mock + GlobalConfig + fixture data; fs/GlobalConfig mock infrastructure not portable to Rust |

### `modules/manager/npm/extract/pnpm › .getPnpmLock()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty if failed to parse | 266 | not-applicable | — | — | Uses fs.readLocalFile mock; fs mock infrastructure not portable to Rust |
| extracts version from monorepo | 272 | not-applicable | — | — | Uses fs.readLocalFile mock + Fixtures; fs mock infrastructure not portable to Rust |
| extracts version from normal repo | 279 | not-applicable | — | — | Uses fs.readLocalFile mock + Fixtures; fs mock infrastructure not portable to Rust |
| extracts version from catalogs | 289 | not-applicable | — | — | Uses fs.readLocalFile mock + Fixtures; fs mock infrastructure not portable to Rust |
| returns empty if no deps | 341 | ported | `npm.rs` | `pnpm_workspace_returns_empty_if_no_deps` | — |

### `modules/manager/npm/extract/pnpm › .extractPnpmWorkspaceFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty catalog entries | 349 | ported | `npm.rs` | `pnpm_workspace_handles_empty_catalog_entries` | — |
| parses valid pnpm-workspace.yaml file | 360 | ported | `npm.rs` | `pnpm_workspace_parses_valid_workspace_file` | — |
| parses overrides in pnpm-workspace.yaml file | 395 | ported | `npm.rs` | `pnpm_workspace_parses_overrides` | — |
| finds relevant lockfile | 466 | ported | `npm.rs` | `pnpm_workspace_finds_relevant_lockfile` | — |

---

