# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/deb/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/deb/index.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 13 | **Status:** not-applicable

### `modules/datasource/deb/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a valid version for the package `album` and does not require redownload | 72 | not-applicable | — | — | Uses httpMock, fs mock, GlobalConfig, tmp-promise; HTTP/fs mock infrastructure not portable to Rust |
| returns null when registry url misses components | 101 | not-applicable | — | — | Uses httpMock, fs mock, GlobalConfig, tmp-promise; HTTP/fs mock infrastructure not portable to Rust |
| returns null when registry url misses binaryArch | 109 | not-applicable | — | — | Uses httpMock, fs mock, GlobalConfig, tmp-promise; HTTP/fs mock infrastructure not portable to Rust |
| returns null when registry url misses suite or release | 117 | not-applicable | — | — | Uses httpMock, fs mock, GlobalConfig, tmp-promise; HTTP/fs mock infrastructure not portable to Rust |
| returns a valid version for the package `album` | 138 | not-applicable | — | — | Uses httpMock, fs mock, GlobalConfig, tmp-promise; HTTP/fs mock infrastructure not portable to Rust |
| returns a valid version for the package `album` if release is used in the registryUrl | 152 | not-applicable | — | — | Uses httpMock, fs mock, GlobalConfig, tmp-promise; HTTP/fs mock infrastructure not portable to Rust |
| returns null for an unknown package | 169 | not-applicable | — | — | Uses httpMock, fs mock, GlobalConfig, tmp-promise; HTTP/fs mock infrastructure not portable to Rust |
| returns two releases for `album` which is the same across the components | 199 | not-applicable | — | — | Uses httpMock, fs mock, GlobalConfig, tmp-promise; HTTP/fs mock infrastructure not portable to Rust |
| returns two releases for `album` which has different metadata across the components | 216 | not-applicable | — | — | Uses httpMock, fs mock, GlobalConfig, tmp-promise; HTTP/fs mock infrastructure not portable to Rust |
| returns null for the package | 244 | not-applicable | — | — | Uses httpMock, fs mock, GlobalConfig, tmp-promise; HTTP/fs mock infrastructure not portable to Rust |
| supports specifying a custom binary arch | 251 | not-applicable | — | — | Uses httpMock, fs mock, GlobalConfig, tmp-promise; HTTP/fs mock infrastructure not portable to Rust |
| should not lead to a race condition on parallel lookups | 281 | not-applicable | — | — | Uses httpMock, fs mock, GlobalConfig, tmp-promise; HTTP/fs mock infrastructure not portable to Rust |
| should parse the extracted package | 317 | not-applicable | — | — | Uses httpMock, fs mock, GlobalConfig, tmp-promise; HTTP/fs mock infrastructure not portable to Rust |

---

