# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/deb/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/deb/index.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/deb/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns a valid version for the package `album` and does not require redownload | 72 | not-applicable | — | — | Debian datasource HTTP mocking |
| returns null when registry url misses components | 101 | not-applicable | — | — | Debian datasource HTTP mocking |
| returns null when registry url misses binaryArch | 109 | not-applicable | — | — | Debian datasource HTTP mocking |
| returns null when registry url misses suite or release | 117 | not-applicable | — | — | Debian datasource HTTP mocking |
| returns a valid version for the package `album` | 138 | not-applicable | — | — | Debian datasource HTTP mocking |
| returns a valid version for the package `album` if release is used in the registryUrl | 152 | not-applicable | — | — | Debian datasource HTTP mocking |
| returns null for an unknown package | 169 | not-applicable | — | — | Debian datasource HTTP mocking |
| returns two releases for `album` which is the same across the components | 199 | not-applicable | — | — | Debian datasource HTTP mocking |
| returns two releases for `album` which has different metadata across the components | 216 | not-applicable | — | — | Debian datasource HTTP mocking |
| returns null for the package | 244 | not-applicable | — | — | Debian datasource HTTP mocking |
| supports specifying a custom binary arch | 251 | not-applicable | — | — | Debian datasource HTTP mocking |
| should not lead to a race condition on parallel lookups | 281 | not-applicable | — | — | Debian datasource HTTP mocking |
| should parse the extracted package | 317 | not-applicable | — | — | Debian datasource HTTP mocking |

---

