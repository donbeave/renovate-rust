# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/hex/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/hex/index.spec.ts
**Total tests:** 33 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `modules/datasource/hex/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 112 | not-applicable | — | — | Renovate's Hex JSON `getReleases` empty-body null contract is not implemented in Rust; Rust returns a JSON parse error for malformed empty responses. |
| returns null for missing fields | 122 | ported | `hex.rs` | `fetch_latest_missing_stable_version_returns_none` | — |
| returns null for 404 | 135 | ported | `hex.rs` | `fetch_latest_404_returns_none` | — |
| returns null for 401 | 142 | ported | `hex.rs` | `fetch_latest_unauthorized_returns_none` | — |
| throws for 429 | 149 | not-applicable | — | — | Renovate's Hex external-host-error contract for rate limits is not implemented in Rust; Rust treats non-success responses as missing latest-version data. |
| throws for 5xx | 156 | not-applicable | — | — | Renovate's Hex external-host-error contract for server errors is not implemented in Rust; Rust treats non-success responses as missing latest-version data. |
| returns null for unknown error | 163 | not-applicable | — | — | Renovate's Hex null-on-network-error `getReleases` contract is not implemented in Rust; Rust propagates HTTP client errors. |
| returns null with wrong auth token | 170 | not-applicable | — | — | Renovate's Hex hostRules authentication and private-token request path are not implemented in Rust. |
| processes real data | 193 | not-applicable | — | — | Renovate's Hex release-list, homepage, sourceUrl, timestamp, and deprecation mapping are not implemented in Rust; Rust only exposes latest_stable_version. |
| process public repo without auth | 207 | not-applicable | — | — | Renovate's Hex release-list and hostRules-aware public repository path are not implemented in Rust; Rust only exposes latest_stable_version. |
| extracts depreceated info | 222 | not-applicable | — | — | Renovate's Hex release deprecation metadata mapping is not implemented in Rust; Rust only exposes latest_stable_version. |
| processes a private repo with auth | 235 | not-applicable | — | — | Renovate's Hex private repository package-name syntax, authentication, and release-list response mapping are not implemented in Rust. |

### `modules/datasource/hex/index › getReleases (V2 protocol)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts versions from V2 response | 272 | not-applicable | — | — | Renovate's Hex v2 protobuf/gzip package protocol is not implemented in Rust; Rust uses the JSON latest-version API only. |
| marks retired releases as deprecated | 318 | not-applicable | — | — | Renovate's Hex v2 retired-release metadata mapping is not implemented in Rust; Rust uses the JSON latest-version API only. |
| filters releases without versions | 359 | not-applicable | — | — | Renovate's Hex v2 release-list filtering is not implemented in Rust; Rust uses the JSON latest-version API only. |
| handles organization packages via V2 | 396 | not-applicable | — | — | Renovate's Hex v2 organization repository package path is not implemented in Rust; Rust uses the JSON latest-version API only. |
| returns null for empty releases | 428 | not-applicable | — | — | Renovate's Hex v2 empty release-list contract is not implemented in Rust; Rust uses the JSON latest-version API only. |
| throws for 5xx errors | 451 | not-applicable | — | — | Renovate's Hex v2 external-host-error contract is not implemented in Rust; Rust uses the JSON latest-version API only. |
| throws for 429 errors | 466 | not-applicable | — | — | Renovate's Hex v2 external-host-error contract is not implemented in Rust; Rust uses the JSON latest-version API only. |
| returns null for 404 | 481 | not-applicable | — | — | Renovate's Hex v2 registryUrl request path is not implemented in Rust; Rust uses the JSON latest-version API only. |
| returns null for network error | 496 | not-applicable | — | — | Renovate's Hex v2 null-on-network-error contract is not implemented in Rust; Rust uses the JSON latest-version API only. |
| returns null for malformed gzip | 511 | not-applicable | — | — | Renovate's Hex v2 gzip/protobuf decoding is not implemented in Rust; Rust uses the JSON latest-version API only. |
| verifies signature when public key is available | 526 | not-applicable | — | — | Renovate's Hex v2 public-key retrieval and signature verification are not implemented in Rust. |
| returns null for invalid signature when public key is available | 561 | not-applicable | — | — | Renovate's Hex v2 public-key retrieval and signature verification are not implemented in Rust. |
| returns null for missing signature when public key is available | 593 | not-applicable | — | — | Renovate's Hex v2 public-key retrieval and signature verification are not implemented in Rust. |
| returns null for malformed public key when verification is enabled | 625 | not-applicable | — | — | Renovate's Hex v2 public-key retrieval and signature verification are not implemented in Rust. |
| falls back to unsigned payload when public key response is empty | 654 | not-applicable | — | — | Renovate's Hex v2 public-key fallback and unsigned protobuf payload handling are not implemented in Rust. |
| uses pinned Hex public key for repo.hex.pm | 686 | not-applicable | — | — | Renovate's pinned Hex v2 public-key verification for repo.hex.pm is not implemented in Rust. |
| maps repo.hex.pm host aliases to hexpm repository checks | 714 | not-applicable | — | — | Renovate's Hex v2 host alias and repository-name validation are not implemented in Rust. |
| caches public key responses for subsequent package lookups | 747 | not-applicable | — | — | Renovate's Hex v2 public-key memory cache is not implemented in Rust. |
| returns null for package name mismatch | 813 | not-applicable | — | — | Renovate's Hex v2 package-name validation is not implemented in Rust; Rust uses the JSON latest-version API only. |
| returns null for organization repository mismatch | 842 | not-applicable | — | — | Renovate's Hex v2 organization repository validation is not implemented in Rust. |
| uses JSON API for hex.pm default registry | 871 | not-applicable | — | — | Renovate's default-registry selection between JSON and v2 release-list protocols is not implemented in Rust; Rust always uses the JSON latest-version API. |

---

