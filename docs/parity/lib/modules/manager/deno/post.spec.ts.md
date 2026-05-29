# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/post.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/post.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 30 | **Status:** not-applicable

### `getDenoLock()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| empty lock file | 23 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| not supported version | 29 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| redirectVersions | 42 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| remoteVersions | 61 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| complex specifiers | 79 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|

### `getLockedVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| empty lock file | 100 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| deno datasource remoteVersions | 105 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| deno datasource redirects | 122 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| get exact lockedVersion | 139 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| get latest lockedVersion | 155 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| get intersects lockedVersion | 171 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| gets lockedVersion for npm package names containing dots | 189 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| invalid lock file content | 206 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|

### `collectPackageJsonAsWorkspaceMember()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should collect package.json files as deno workspace members | 229 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| should handle when extractDenoCompatiblePackageJson returns null | 287 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|

### `normalizeWorkspace()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| nested workspace is invalid | 323 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|

### `postExtract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should handle lock file reading failure | 416 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| should handle invalid lock file JSON | 422 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| should handle deno datasource with no remoteVersions match | 428 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| should handle deno datasource with no depName | 445 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| should handle jsr datasource with no lockedVersions | 458 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| should apply locked versions from lock files | 472 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| should handle lock file with no lockFiles | 506 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| should use lock file cache for multiple packages | 532 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| should handle deno datasource with empty redirectVersions | 585 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| should handle deno datasource with currentValue and depName for redirects | 599 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| should handle dep without lockedVersion match | 616 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|

### `normalizeWorkspace() - additional cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| workspace member not matching any workspace pattern | 652 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| nested workspace removal with packageMap.get returning undefined | 674 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|
| invalidPackageFiles entry not found in packageMap | 699 | not-applicable | — | — | mocking framework internals — vi.mock on fs/exec/platform; TypeScript Deno post-processing pipeline|

---

