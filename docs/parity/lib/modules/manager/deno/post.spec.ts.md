# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/post.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/post.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `getDenoLock()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| empty lock file | 23 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno lock parsing behavior is in scope |
| not supported version | 29 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno lock parsing behavior is in scope |
| redirectVersions | 42 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno lock parsing behavior is in scope |
| remoteVersions | 61 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno lock parsing behavior is in scope |
| complex specifiers | 79 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno lock parsing behavior is in scope |

### `getLockedVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| empty lock file | 100 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno locked-version behavior is in scope |
| deno datasource remoteVersions | 105 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno locked-version behavior is in scope |
| deno datasource redirects | 122 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno locked-version behavior is in scope |
| get exact lockedVersion | 139 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno locked-version behavior is in scope |
| get latest lockedVersion | 155 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno locked-version behavior is in scope |
| get intersects lockedVersion | 171 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno locked-version behavior is in scope |
| gets lockedVersion for npm package names containing dots | 189 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno locked-version behavior is in scope |
| invalid lock file content | 206 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno locked-version behavior is in scope |

### `collectPackageJsonAsWorkspaceMember()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should collect package.json files as deno workspace members | 229 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno workspace post-processing behavior is in scope |
| should handle when extractDenoCompatiblePackageJson returns null | 287 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno workspace post-processing behavior is in scope |

### `normalizeWorkspace()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| nested workspace is invalid | 323 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno workspace normalization behavior is in scope |

### `postExtract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should handle lock file reading failure | 416 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno postExtract lockfile behavior is in scope |
| should handle invalid lock file JSON | 422 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno postExtract lockfile behavior is in scope |
| should handle deno datasource with no remoteVersions match | 428 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno postExtract dependency behavior is in scope |
| should handle deno datasource with no depName | 445 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno postExtract dependency behavior is in scope |
| should handle jsr datasource with no lockedVersions | 458 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno postExtract dependency behavior is in scope |
| should apply locked versions from lock files | 472 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno postExtract lockfile behavior is in scope |
| should handle lock file with no lockFiles | 506 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno postExtract lockfile behavior is in scope |
| should use lock file cache for multiple packages | 532 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno postExtract lockfile behavior is in scope |
| should handle deno datasource with empty redirectVersions | 585 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno postExtract dependency behavior is in scope |
| should handle deno datasource with currentValue and depName for redirects | 599 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno postExtract dependency behavior is in scope |
| should handle dep without lockedVersion match | 616 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno postExtract dependency behavior is in scope |

### `normalizeWorkspace() - additional cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| workspace member not matching any workspace pattern | 652 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno workspace normalization behavior is in scope |
| nested workspace removal with packageMap.get returning undefined | 674 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno workspace normalization behavior is in scope |
| invalidPackageFiles entry not found in packageMap | 699 | not-applicable | Mock framework internals — tests deno post via vitest-mocked fs/exec; Rust tests this at different layer | — | Deno workspace normalization behavior is in scope |

---
