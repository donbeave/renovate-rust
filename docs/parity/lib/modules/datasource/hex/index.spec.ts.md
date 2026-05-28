# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/hex/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/hex/index.spec.ts
**Total tests:** 33 | **Ported:** 3 | **Actionable:** 33 | **Status:** done

### `modules/datasource/hex/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 112 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for missing fields | 122 | ported | `hex.rs` | `fetch_latest_missing_stable_version_returns_none` | — |
| returns null for 404 | 135 | ported | `hex.rs` | `fetch_latest_404_returns_none` | — |
| returns null for 401 | 142 | ported | `hex.rs` | `fetch_latest_unauthorized_returns_none` | — |
| throws for 429 | 149 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| throws for 5xx | 156 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for unknown error | 163 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null with wrong auth token | 170 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |
| processes real data | 193 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| process public repo without auth | 207 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| extracts depreceated info | 222 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| processes a private repo with auth | 235 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |

### `modules/datasource/hex/index › getReleases (V2 protocol)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts versions from V2 response | 272 | not-applicable | — | — | Requires httpMock + protobuf gzip encoding infrastructure |
| marks retired releases as deprecated | 318 | not-applicable | — | — | Requires httpMock + protobuf gzip encoding infrastructure |
| filters releases without versions | 359 | not-applicable | — | — | Requires httpMock + protobuf gzip encoding infrastructure |
| handles organization packages via V2 | 396 | not-applicable | — | — | Requires httpMock + protobuf gzip encoding infrastructure |
| returns null for empty releases | 428 | not-applicable | — | — | Requires httpMock + protobuf gzip encoding infrastructure |
| throws for 5xx errors | 451 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| throws for 429 errors | 466 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for 404 | 481 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for network error | 496 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for malformed gzip | 511 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| verifies signature when public key is available | 526 | not-applicable | — | — | Requires httpMock + crypto key pair generation mock |
| returns null for invalid signature when public key is available | 561 | not-applicable | — | — | Requires httpMock + crypto key pair generation mock |
| returns null for missing signature when public key is available | 593 | not-applicable | — | — | Requires httpMock + crypto key pair generation mock |
| returns null for malformed public key when verification is enabled | 625 | not-applicable | — | — | Requires httpMock + crypto key pair generation mock |
| falls back to unsigned payload when public key response is empty | 654 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses pinned Hex public key for repo.hex.pm | 686 | not-applicable | — | — | Requires httpMock + crypto key pair generation mock |
| maps repo.hex.pm host aliases to hexpm repository checks | 714 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |
| caches public key responses for subsequent package lookups | 747 | not-applicable | — | — | Requires httpMock + memCache mock infrastructure |
| returns null for package name mismatch | 813 | not-applicable | — | — | Requires httpMock + protobuf gzip encoding infrastructure |
| returns null for organization repository mismatch | 842 | not-applicable | — | — | Requires httpMock + protobuf gzip encoding infrastructure |
| uses JSON API for hex.pm default registry | 871 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

---
