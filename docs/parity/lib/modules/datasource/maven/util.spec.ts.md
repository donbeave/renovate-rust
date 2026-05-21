# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/maven/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/maven/util.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/maven/util`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns error for unsupported protocols | 52 | not-applicable | — | — | Renovate's Maven datasource HTTP/S3 utility error contract is not exposed as a Rust API. |
| returns error for xml parse error | 63 | not-applicable | — | — | Renovate's Maven datasource HTTP/S3 utility error contract is not exposed as a Rust API. |
| returns the downloaded text body | 81 | not-applicable | — | — | Renovate's Maven datasource download utility is not exposed as a Rust API. |
| returns error for non-S3 URLs | 98 | not-applicable | — | — | Renovate's Maven S3 utility helpers are not implemented in Rust. |
| uses correct cache provider for %s | 118 | not-applicable | — | — | Renovate's Maven datasource HTTP cache-provider selection is not implemented in Rust. |
| returns empty for HOST_DISABLED error | 108 | not-applicable | — | — | Renovate's Maven datasource host-rule error classification is not implemented in Rust. |
| returns empty for host error | 119 | not-applicable | — | — | Renovate's Maven datasource host-rule error classification is not implemented in Rust. |
| returns empty for temporary error | 130 | not-applicable | — | — | Renovate's Maven datasource temporary error classification is not implemented in Rust. |
| throws ExternalHostError for 429 status with redis cache | 153 | not-applicable | — | — | Renovate's Maven external-host-error and Redis cache behavior is not implemented in Rust. |
| throws ExternalHostError for 429 status without redis cache | 174 | not-applicable | — | — | Renovate's Maven external-host-error behavior is not implemented in Rust. |
| throws ExternalHostError for non-429 temporary error on maven central | 195 | not-applicable | — | — | Renovate's Maven external-host-error behavior is not implemented in Rust. |
| returns empty for connection error | 210 | not-applicable | — | — | Renovate's Maven datasource connection-error classification is not implemented in Rust. |
| returns empty for unsupported error | 221 | not-applicable | — | — | Renovate's Maven datasource utility error classification is not implemented in Rust. |

| caches 404 for maven-metadata.xml URLs | 301 | not-applicable | — | — | Renovate's Maven datasource utility error classification is not implemented in Rust. |
| does not cache 404 for non-metadata URLs | 327 | not-applicable | — | — | Renovate's Maven datasource utility error classification is not implemented in Rust. |
| returns cached not-found without making HTTP request | 343 | not-applicable | — | — | Renovate's Maven datasource utility error classification is not implemented in Rust. |
---

