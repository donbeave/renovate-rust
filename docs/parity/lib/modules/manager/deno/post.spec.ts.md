# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/post.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/post.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 30 | **Status:** pending

### `getDenoLock()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| empty lock file | 23 | pending | — | — | Deno lock parsing behavior is in scope |
| not supported version | 29 | pending | — | — | Deno lock parsing behavior is in scope |
| redirectVersions | 42 | pending | — | — | Deno lock parsing behavior is in scope |
| remoteVersions | 61 | pending | — | — | Deno lock parsing behavior is in scope |
| complex specifiers | 79 | pending | — | — | Deno lock parsing behavior is in scope |

### `getLockedVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| empty lock file | 100 | pending | — | — | Deno locked-version behavior is in scope |
| deno datasource remoteVersions | 105 | pending | — | — | Deno locked-version behavior is in scope |
| deno datasource redirects | 122 | pending | — | — | Deno locked-version behavior is in scope |
| get exact lockedVersion | 139 | pending | — | — | Deno locked-version behavior is in scope |
| get latest lockedVersion | 155 | pending | — | — | Deno locked-version behavior is in scope |
| get intersects lockedVersion | 171 | pending | — | — | Deno locked-version behavior is in scope |
| gets lockedVersion for npm package names containing dots | 189 | pending | — | — | Deno locked-version behavior is in scope |
| invalid lock file content | 206 | pending | — | — | Deno locked-version behavior is in scope |

### `collectPackageJsonAsWorkspaceMember()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should collect package.json files as deno workspace members | 229 | pending | — | — | Deno workspace post-processing behavior is in scope |
| should handle when extractDenoCompatiblePackageJson returns null | 287 | pending | — | — | Deno workspace post-processing behavior is in scope |

### `normalizeWorkspace()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| nested workspace is invalid | 323 | pending | — | — | Deno workspace normalization behavior is in scope |

### `postExtract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should handle lock file reading failure | 416 | pending | — | — | Deno postExtract lockfile behavior is in scope |
| should handle invalid lock file JSON | 422 | pending | — | — | Deno postExtract lockfile behavior is in scope |
| should handle deno datasource with no remoteVersions match | 428 | pending | — | — | Deno postExtract dependency behavior is in scope |
| should handle deno datasource with no depName | 445 | pending | — | — | Deno postExtract dependency behavior is in scope |
| should handle jsr datasource with no lockedVersions | 458 | pending | — | — | Deno postExtract dependency behavior is in scope |
| should apply locked versions from lock files | 472 | pending | — | — | Deno postExtract lockfile behavior is in scope |
| should handle lock file with no lockFiles | 506 | pending | — | — | Deno postExtract lockfile behavior is in scope |
| should use lock file cache for multiple packages | 532 | pending | — | — | Deno postExtract lockfile behavior is in scope |
| should handle deno datasource with empty redirectVersions | 585 | pending | — | — | Deno postExtract dependency behavior is in scope |
| should handle deno datasource with currentValue and depName for redirects | 599 | pending | — | — | Deno postExtract dependency behavior is in scope |
| should handle dep without lockedVersion match | 616 | pending | — | — | Deno postExtract dependency behavior is in scope |

### `normalizeWorkspace() - additional cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| workspace member not matching any workspace pattern | 652 | pending | — | — | Deno workspace normalization behavior is in scope |
| nested workspace removal with packageMap.get returning undefined | 674 | pending | — | — | Deno workspace normalization behavior is in scope |
| invalidPackageFiles entry not found in packageMap | 699 | pending | — | — | Deno workspace normalization behavior is in scope |

---
