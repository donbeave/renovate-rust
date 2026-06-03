# `lib/modules/datasource/deb/index.spec.ts`

[← `datasource/deb`](../../../../_by-module/datasource/deb.md) · [all modules](../../../../README.md)

**0/13 ported** (13 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 72 | returns a valid version for the package `album` and does not require redownload | pending | — |
| 101 | returns null when registry url misses components | pending | — |
| 109 | returns null when registry url misses binaryarch | pending | — |
| 117 | returns null when registry url misses suite or release | pending | — |
| 138 | returns a valid version for the package `album` | pending | — |
| 152 | returns a valid version for the package `album` if release is used in the registryurl | pending | — |
| 169 | returns null for an unknown package | pending | — |
| 199 | returns two releases for `album` which is the same across the components | pending | — |
| 216 | returns two releases for `album` which has different metadata across the components | pending | — |
| 244 | returns null for the package | pending | — |
| 251 | supports specifying a custom binary arch | pending | — |
| 281 | should not lead to a race condition on parallel lookups | pending | — |
| 317 | should parse the extracted package | pending | — |

