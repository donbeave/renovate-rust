# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/extract.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `getLockFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| found lock file | 22 | not-applicable | — | — | tests Deno import extraction; Rust deno extractor uses different parsing approach |
| not found lock file | 29 | not-applicable | — | — | tests Deno import extraction; Rust deno extractor uses different parsing approach |

### `processImportMap()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| importMap | 37 | not-applicable | — | — | tests Deno import extraction; Rust deno extractor uses different parsing approach |
| remote importMap | 81 | not-applicable | — | — | tests Deno import extraction; Rust deno extractor uses different parsing approach |
| importMap path specified but not exists | 91 | not-applicable | — | — | tests Deno import extraction; Rust deno extractor uses different parsing approach |
| invalid importMap file | 98 | not-applicable | — | — | tests Deno import extraction; Rust deno extractor uses different parsing approach |

### `processDenoExtract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| importMap | 107 | not-applicable | — | — | tests Deno import extraction; Rust deno extractor uses different parsing approach |

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| invalid deno.json file | 133 | not-applicable | — | — | tests Deno import extraction; Rust deno extractor uses different parsing approach |
| multiple matched files with deno.json only | 139 | not-applicable | — | — | tests Deno import extraction; Rust deno extractor uses different parsing approach |
| deno.lock without package.json | 157 | not-applicable | — | — | tests Deno import extraction; Rust deno extractor uses different parsing approach |
| deno.lock when collectPackageJson returns null | 163 | not-applicable | — | — | tests Deno import extraction; Rust deno extractor uses different parsing approach |
| deno.lock when collectPackageJson returns empty array | 168 | not-applicable | — | — | tests Deno import extraction; Rust deno extractor uses different parsing approach |
| complex config with imports, scopes, tasks and lint | 173 | not-applicable | — | — | tests Deno import extraction; Rust deno extractor uses different parsing approach |

### `extractAllPackageFiles() › workspaces`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| npm workspace compatible | 362 | not-applicable | — | — | tests Deno import extraction; Rust deno extractor uses different parsing approach |

---

