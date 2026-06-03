# `lib/modules/manager/deno/post.spec.ts`

[← `manager/deno`](../../../../_by-module/manager/deno.md) · [all modules](../../../../README.md)

**4/30 ported** (26 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 23 | empty lock file | pending | — |
| 29 | not supported version | pending | — |
| 42 | redirectversions | pending | — |
| 61 | remoteversions | pending | — |
| 79 | complex specifiers | pending | — |
| 100 | empty lock file | pending | — |
| 105 | deno datasource remoteversions | pending | — |
| 122 | deno datasource redirects | pending | — |
| 139 | get exact lockedversion | ported | `crates/renovate-core/src/extractors/deno.rs:1008` |
| 155 | get latest lockedversion | ported | `crates/renovate-core/src/extractors/deno.rs:1029` |
| 171 | get intersects lockedversion | pending | — |
| 189 | gets lockedversion for npm package names containing dots | pending | — |
| 206 | invalid lock file content | pending | — |
| 229 | should collect package.json files as deno workspace members | pending | — |
| 287 | should handle when extractdenocompatiblepackagejson returns null | pending | — |
| 323 | nested workspace is invalid | pending | — |
| 416 | should handle lock file reading failure | pending | — |
| 422 | should handle invalid lock file json | pending | — |
| 428 | should handle deno datasource with no remoteversions match | pending | — |
| 445 | should handle deno datasource with no depname | pending | — |
| 458 | should handle jsr datasource with no lockedversions | pending | — |
| 472 | should apply locked versions from lock files | pending | — |
| 506 | should handle lock file with no lockfiles | ported | `crates/renovate-core/src/extractors/deno.rs:1101` |
| 532 | should use lock file cache for multiple packages | ported | `crates/renovate-core/src/extractors/deno.rs:1121` |
| 585 | should handle deno datasource with empty redirectversions | pending | — |
| 599 | should handle deno datasource with currentvalue and depname for redirects | pending | — |
| 616 | should handle dep without lockedversion match | pending | — |
| 652 | workspace member not matching any workspace pattern | pending | — |
| 674 | nested workspace removal with packagemap.get returning undefined | pending | — |
| 699 | invalidpackagefiles entry not found in packagemap | pending | — |

