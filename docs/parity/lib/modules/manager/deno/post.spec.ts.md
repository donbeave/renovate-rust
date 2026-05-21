# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/post.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/post.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `getDenoLock()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| empty lock file | 23 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| not supported version | 29 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| redirectVersions | 42 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| remoteVersions | 61 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| complex specifiers | 79 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |

### `getLockedVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| empty lock file | 100 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| deno datasource remoteVersions | 105 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| deno datasource redirects | 122 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| get exact lockedVersion | 139 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| get latest lockedVersion | 155 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| get intersects lockedVersion | 171 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| gets lockedVersion for npm package names containing dots | 189 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| invalid lock file content | 206 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |

### `collectPackageJsonAsWorkspaceMember()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should collect package.json files as deno workspace members | 229 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| should handle when extractDenoCompatiblePackageJson returns null | 287 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |

### `normalizeWorkspace()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| nested workspace is invalid | 323 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |

### `postExtract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should handle lock file reading failure | 416 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| should handle invalid lock file JSON | 422 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| should handle deno datasource with no remoteVersions match | 428 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| should handle deno datasource with no depName | 445 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| should handle jsr datasource with no lockedVersions | 458 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| should apply locked versions from lock files | 472 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| should handle lock file with no lockFiles | 506 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| should use lock file cache for multiple packages | 532 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| should handle deno datasource with empty redirectVersions | 585 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| should handle deno datasource with currentValue and depName for redirects | 599 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| should handle dep without lockedVersion match | 616 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |

### `normalizeWorkspace() - additional cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| workspace member not matching any workspace pattern | 652 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| nested workspace removal with packageMap.get returning undefined | 674 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |
| invalidPackageFiles entry not found in packageMap | 699 | not-applicable | — | — | tests Deno post-update script execution via Node.js exec; external tool invocation out of scope |

---

