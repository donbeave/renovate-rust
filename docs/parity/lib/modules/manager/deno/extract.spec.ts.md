# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/extract.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 14 | **Status:** not-applicable

### `getLockFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| found lock file | 22 | pending | — | — | — |
| not found lock file | 29 | pending | — | — | — |

### `processImportMap()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| importMap | 37 | pending | — | — | — |
| remote importMap | 81 | pending | — | — | — |
| importMap path specified but not exists | 91 | pending | — | — | — |
| invalid importMap file | 98 | pending | — | — | — |

### `processDenoExtract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| importMap | 107 | pending | — | — | — |

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| invalid deno.json file | 133 | pending | — | — | — |
| multiple matched files with deno.json only | 139 | pending | — | — | — |
| deno.lock without package.json | 157 | pending | — | — | — |
| deno.lock when collectPackageJson returns null | 163 | pending | — | — | — |
| deno.lock when collectPackageJson returns empty array | 168 | pending | — | — | — |
| complex config with imports, scopes, tasks and lint | 173 | pending | — | — | — |

### `extractAllPackageFiles() › workspaces`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| npm workspace compatible | 362 | pending | — | — | — |

---

