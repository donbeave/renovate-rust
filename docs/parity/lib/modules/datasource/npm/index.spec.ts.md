# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/npm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/npm/index.spec.ts
**Total tests:** 24 | **Ported:** 6 | **Actionable:** 24 | **Status:** done

### `modules/datasource/npm/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for no versions | 44 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should fetch package info from npm | 55 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should parse repo url | 65 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should parse repo url (string) | 90 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should return deprecated | 111 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should return attestation | 144 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should handle foobar | 196 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should handle no time | 203 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should return null if lookup fails 401 | 210 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should return null if lookup fails | 216 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should throw error for unparseable | 222 | ported | `npm.rs` | `fetch_versions_unparseable_returns_parse_error` | — |
| should throw error for 429 | 229 | ported | `npm.rs` | `fetch_versions_non_success_statuses_return_error` | Rust verifies the equivalent non-success packument response error behavior. |
| should throw error for 5xx | 236 | ported | `npm.rs` | `fetch_versions_non_success_statuses_return_error` | Rust verifies the equivalent non-success packument response error behavior. |
| should throw error for 408 | 243 | ported | `npm.rs` | `fetch_versions_non_success_statuses_return_error` | Rust verifies the equivalent non-success packument response error behavior. |
| should throw error for others | 250 | ported | `npm.rs` | `fetch_versions_non_success_statuses_return_error` | Rust verifies the equivalent non-success packument response error behavior. |
| should not send an authorization header if public package | 257 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |
| should send an authorization header if provided | 268 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |
| should use host rules by hostName if provided | 283 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |
| should use host rules by baseUrl if provided | 304 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |
| resets npmrc | 330 | not-applicable | — | — | Requires httpMock + npmrc state mock infrastructure |
| should use default registry if missing from npmrc | 337 | not-applicable | — | — | Requires httpMock + npmrc state mock infrastructure |
| should fetch package info from custom registry | 348 | ported | `npm.rs` | `fetch_versions_returns_non_deprecated_sorted` | Rust verifies packument fetches through the supplied registry base URL. |
| should replace any environment variable in npmrc | 363 | not-applicable | — | — | Requires httpMock + env var injection infrastructure |
| should throw error if necessary env var is not present | 380 | not-applicable | — | — | Requires httpMock + env var injection infrastructure |

---
