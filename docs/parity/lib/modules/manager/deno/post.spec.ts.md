# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/post.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/post.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 30 | **Status:** pending

### `getDenoLock()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| empty lock file | 23 | pending | — | — | —|
| not supported version | 29 | pending | — | — | —|
| redirectVersions | 42 | pending | — | — | —|
| remoteVersions | 61 | pending | — | — | —|
| complex specifiers | 79 | pending | — | — | —|

### `getLockedVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| empty lock file | 100 | pending | — | — | —|
| deno datasource remoteVersions | 105 | pending | — | — | —|
| deno datasource redirects | 122 | pending | — | — | —|
| get exact lockedVersion | 139 | pending | — | — | —|
| get latest lockedVersion | 155 | pending | — | — | —|
| get intersects lockedVersion | 171 | pending | — | — | —|
| gets lockedVersion for npm package names containing dots | 189 | pending | — | — | —|
| invalid lock file content | 206 | pending | — | — | —|

### `collectPackageJsonAsWorkspaceMember()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should collect package.json files as deno workspace members | 229 | pending | — | — | —|
| should handle when extractDenoCompatiblePackageJson returns null | 287 | pending | — | — | —|

### `normalizeWorkspace()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| nested workspace is invalid | 323 | pending | — | — | —|

### `postExtract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should handle lock file reading failure | 416 | pending | — | — | —|
| should handle invalid lock file JSON | 422 | pending | — | — | —|
| should handle deno datasource with no remoteVersions match | 428 | pending | — | — | —|
| should handle deno datasource with no depName | 445 | pending | — | — | —|
| should handle jsr datasource with no lockedVersions | 458 | pending | — | — | —|
| should apply locked versions from lock files | 472 | pending | — | — | —|
| should handle lock file with no lockFiles | 506 | pending | — | — | —|
| should use lock file cache for multiple packages | 532 | pending | — | — | —|
| should handle deno datasource with empty redirectVersions | 585 | pending | — | — | —|
| should handle deno datasource with currentValue and depName for redirects | 599 | pending | — | — | —|
| should handle dep without lockedVersion match | 616 | pending | — | — | —|

### `normalizeWorkspace() - additional cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| workspace member not matching any workspace pattern | 652 | pending | — | — | —|
| nested workspace removal with packageMap.get returning undefined | 674 | pending | — | — | —|
| invalidPackageFiles entry not found in packageMap | 699 | pending | — | — | —|

---

