# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/npm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/npm/index.spec.ts
**Total tests:** 24 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `modules/datasource/npm/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for no versions | 44 | not-applicable | — | — | Renovate's npm datasource returns null for empty version maps; Rust preserves an empty version cache entry for update-summary consumers. |
| should fetch package info from npm | 55 | not-applicable | — | — | Renovate's npm full release-list, homepage, sourceUrl, sourceDirectory, isPrivate, and cache-control mapping are not implemented in Rust. |
| should parse repo url | 65 | not-applicable | — | — | Renovate's npm repository URL metadata normalization is not implemented in Rust. |
| should parse repo url (string) | 90 | not-applicable | — | — | Renovate's npm per-version repository metadata normalization is not implemented in Rust. |
| should return deprecated | 111 | not-applicable | — | — | Renovate's npm deprecation message output is not implemented in Rust; Rust excludes deprecated versions from update candidates. |
| should return attestation | 144 | not-applicable | — | — | Renovate's npm dist attestation metadata mapping is not implemented in Rust. |
| should handle foobar | 196 | not-applicable | — | — | Renovate's npm full `getPkgReleases` snapshot includes metadata and isPrivate behavior not implemented in Rust. |
| should handle no time | 203 | not-applicable | — | — | Renovate's npm per-release timestamp fallback in full release output is not implemented in Rust. |
| should return null if lookup fails 401 | 210 | not-applicable | — | — | Renovate's npm 401-as-null contract differs from Rust, which returns an HTTP error for non-success packument responses. |
| should return null if lookup fails | 216 | not-applicable | — | — | Renovate's npm 404-as-null contract differs from Rust, which returns an HTTP error for non-success packument responses. |
| should throw error for unparseable | 222 | ported | `npm.rs` | `fetch_versions_unparseable_returns_parse_error` | — |
| should throw error for 429 | 229 | ported | `npm.rs` | `fetch_versions_non_success_statuses_return_error` | Rust verifies the equivalent non-success packument response error behavior. |
| should throw error for 5xx | 236 | ported | `npm.rs` | `fetch_versions_non_success_statuses_return_error` | Rust verifies the equivalent non-success packument response error behavior. |
| should throw error for 408 | 243 | ported | `npm.rs` | `fetch_versions_non_success_statuses_return_error` | Rust verifies the equivalent non-success packument response error behavior. |
| should throw error for others | 250 | ported | `npm.rs` | `fetch_versions_non_success_statuses_return_error` | Rust verifies the equivalent non-success packument response error behavior. |
| should not send an authorization header if public package | 257 | not-applicable | — | — | Renovate's npm request auth-header policy is not implemented in Rust. |
| should send an authorization header if provided | 268 | not-applicable | — | — | Renovate's npmrc auth-header injection is not implemented in Rust. |
| should use host rules by hostName if provided | 283 | not-applicable | — | — | Renovate's npm hostRules auth-header injection is not implemented in Rust. |
| should use host rules by baseUrl if provided | 304 | not-applicable | — | — | Renovate's npm hostRules auth-header injection is not implemented in Rust. |
| resets npmrc | 330 | not-applicable | — | — | Renovate's npmrc global state reset is not implemented in Rust. |
| should use default registry if missing from npmrc | 337 | not-applicable | — | — | Renovate's npmrc registry resolution is not implemented in Rust; Rust callers pass the registry URL directly. |
| should fetch package info from custom registry | 348 | ported | `npm.rs` | `fetch_versions_returns_non_deprecated_sorted` | Rust verifies packument fetches through the supplied registry base URL. |
| should replace any environment variable in npmrc | 363 | not-applicable | — | — | Renovate's npmrc environment variable expansion is not implemented in Rust. |
| should throw error if necessary env var is not present | 380 | not-applicable | — | — | Renovate's npmrc environment variable expansion error path is not implemented in Rust. |

---

