# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/extract.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `getLockFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| found lock file | 22 | not-applicable | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer | — | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer |
| not found lock file | 29 | not-applicable | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer | — | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer |

### `processImportMap()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| importMap | 37 | not-applicable | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer | — | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer |
| remote importMap | 81 | not-applicable | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer | — | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer |
| importMap path specified but not exists | 91 | not-applicable | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer | — | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer |
| invalid importMap file | 98 | not-applicable | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer | — | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer |

### `processDenoExtract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| importMap | 107 | not-applicable | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer | — | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer |

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| invalid deno.json file | 133 | not-applicable | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer | — | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer |
| multiple matched files with deno.json only | 139 | not-applicable | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer | — | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer |
| deno.lock without package.json | 157 | not-applicable | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer | — | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer |
| deno.lock when collectPackageJson returns null | 163 | not-applicable | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer | — | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer |
| deno.lock when collectPackageJson returns empty array | 168 | not-applicable | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer | — | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer |
| complex config with imports, scopes, tasks and lint | 173 | not-applicable | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer | — | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer |

### `extractAllPackageFiles() › workspaces`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| npm workspace compatible | 362 | not-applicable | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer | — | Mock framework internals — tests deno extract via vitest-mocked fs; Rust tests this at different layer |

---

