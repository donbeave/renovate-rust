# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/post.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/post.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 30 | **Status:** done

### `getDenoLock()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| empty lock file | 23 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| not supported version | 29 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| redirectVersions | 42 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| remoteVersions | 61 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| complex specifiers | 79 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |

### `getLockedVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| empty lock file | 100 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| deno datasource remoteVersions | 105 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| deno datasource redirects | 122 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| get exact lockedVersion | 139 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| get latest lockedVersion | 155 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| get intersects lockedVersion | 171 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| gets lockedVersion for npm package names containing dots | 189 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| invalid lock file content | 206 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |

### `collectPackageJsonAsWorkspaceMember()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should collect package.json files as deno workspace members | 229 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| should handle when extractDenoCompatiblePackageJson returns null | 287 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |

### `normalizeWorkspace()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| nested workspace is invalid | 323 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |

### `postExtract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should handle lock file reading failure | 416 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| should handle invalid lock file JSON | 422 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| should handle deno datasource with no remoteVersions match | 428 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| should handle deno datasource with no depName | 445 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| should handle jsr datasource with no lockedVersions | 458 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| should apply locked versions from lock files | 472 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| should handle lock file with no lockFiles | 506 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| should use lock file cache for multiple packages | 532 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| should handle deno datasource with empty redirectVersions | 585 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| should handle deno datasource with currentValue and depName for redirects | 599 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| should handle dep without lockedVersion match | 616 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |

### `normalizeWorkspace() - additional cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| workspace member not matching any workspace pattern | 652 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| nested workspace removal with packageMap.get returning undefined | 674 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |
| invalidPackageFiles entry not found in packageMap | 699 | not-applicable | — | — | Requires vi.mock fs/git mock infrastructure |

---

