# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/npm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/npm/index.spec.ts
**Total tests:** 24 | **Ported:** 6 | **Actionable:** 18 | **Status:** partial

### `modules/datasource/npm/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null for no versions | 44 | pending | — | — | —|
| should fetch package info from npm | 55 | pending | — | — | —|
| should parse repo url | 65 | pending | — | — | —|
| should parse repo url (string) | 90 | pending | — | — | —|
| should return deprecated | 111 | pending | — | — | —|
| should return attestation | 144 | pending | — | — | —|
| should handle foobar | 196 | pending | — | — | —|
| should handle no time | 203 | pending | — | — | —|
| should return null if lookup fails 401 | 210 | pending | — | — | —|
| should return null if lookup fails | 216 | pending | — | — | —|
| should throw error for unparseable | 222 | ported | `npm.rs` | `fetch_versions_unparseable_returns_parse_error` | — |
| should throw error for 429 | 229 | ported | `npm.rs` | `fetch_versions_non_success_statuses_return_error` | Rust verifies the equivalent non-success packument response error behavior. |
| should throw error for 5xx | 236 | ported | `npm.rs` | `fetch_versions_non_success_statuses_return_error` | Rust verifies the equivalent non-success packument response error behavior. |
| should throw error for 408 | 243 | ported | `npm.rs` | `fetch_versions_non_success_statuses_return_error` | Rust verifies the equivalent non-success packument response error behavior. |
| should throw error for others | 250 | ported | `npm.rs` | `fetch_versions_non_success_statuses_return_error` | Rust verifies the equivalent non-success packument response error behavior. |
| should not send an authorization header if public package | 257 | pending | — | — | —|
| should send an authorization header if provided | 268 | pending | — | — | —|
| should use host rules by hostName if provided | 283 | pending | — | — | —|
| should use host rules by baseUrl if provided | 304 | pending | — | — | —|
| resets npmrc | 330 | pending | — | — | —|
| should use default registry if missing from npmrc | 337 | pending | — | — | —|
| should fetch package info from custom registry | 348 | ported | `npm.rs` | `fetch_versions_returns_non_deprecated_sorted` | Rust verifies packument fetches through the supplied registry base URL. |
| should replace any environment variable in npmrc | 363 | pending | — | — | —|
| should throw error if necessary env var is not present | 380 | pending | — | — | —|

---
