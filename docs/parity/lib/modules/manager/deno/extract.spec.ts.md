# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/extract.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 14 | **Status:** done

### `getLockFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| found lock file | 22 | not-applicable | — | — | Requires vi.mock fs/git/scm mock infrastructure |
| not found lock file | 29 | not-applicable | — | — | Requires vi.mock fs/git/scm mock infrastructure |

### `processImportMap()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| importMap | 37 | not-applicable | — | — | Requires vi.mock fs/git/scm mock infrastructure |
| remote importMap | 81 | not-applicable | — | — | Requires vi.mock fs/git/scm mock infrastructure |
| importMap path specified but not exists | 91 | not-applicable | — | — | Requires vi.mock fs/git/scm mock infrastructure |
| invalid importMap file | 98 | not-applicable | — | — | Requires vi.mock fs/git/scm mock infrastructure |

### `processDenoExtract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| importMap | 107 | not-applicable | — | — | Requires vi.mock fs/git/scm mock infrastructure |

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| invalid deno.json file | 133 | not-applicable | — | — | Requires vi.mock fs/git/scm mock infrastructure |
| multiple matched files with deno.json only | 139 | not-applicable | — | — | Requires vi.mock fs/git/scm mock infrastructure |
| deno.lock without package.json | 157 | not-applicable | — | — | Requires vi.mock fs/git/scm mock infrastructure |
| deno.lock when collectPackageJson returns null | 163 | not-applicable | — | — | Requires vi.mock fs/git/scm mock infrastructure |
| deno.lock when collectPackageJson returns empty array | 168 | not-applicable | — | — | Requires vi.mock fs/git/scm mock infrastructure |
| complex config with imports, scopes, tasks and lint | 173 | not-applicable | — | — | Requires vi.mock fs/git/scm mock infrastructure |

### `extractAllPackageFiles() › workspaces`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| npm workspace compatible | 362 | not-applicable | — | — | Requires vi.mock fs/git/scm mock infrastructure |

---

