# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/hex/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/hex/index.spec.ts
**Total tests:** 33 | **Ported:** 3 | **Actionable:** 33 | **Status:** partial

### `modules/datasource/hex/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 112 | pending | — | — | —|
| returns null for missing fields | 122 | ported | `hex.rs` | `fetch_latest_missing_stable_version_returns_none` | — |
| returns null for 404 | 135 | ported | `hex.rs` | `fetch_latest_404_returns_none` | — |
| returns null for 401 | 142 | ported | `hex.rs` | `fetch_latest_unauthorized_returns_none` | — |
| throws for 429 | 149 | pending | — | — | —|
| throws for 5xx | 156 | pending | — | — | —|
| returns null for unknown error | 163 | pending | — | — | —|
| returns null with wrong auth token | 170 | pending | — | — | —|
| processes real data | 193 | pending | — | — | —|
| process public repo without auth | 207 | pending | — | — | —|
| extracts depreceated info | 222 | pending | — | — | —|
| processes a private repo with auth | 235 | pending | — | — | —|

### `modules/datasource/hex/index › getReleases (V2 protocol)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts versions from V2 response | 272 | pending | — | — | —|
| marks retired releases as deprecated | 318 | pending | — | — | —|
| filters releases without versions | 359 | pending | — | — | —|
| handles organization packages via V2 | 396 | pending | — | — | —|
| returns null for empty releases | 428 | pending | — | — | —|
| throws for 5xx errors | 451 | pending | — | — | —|
| throws for 429 errors | 466 | pending | — | — | —|
| returns null for 404 | 481 | pending | — | — | —|
| returns null for network error | 496 | pending | — | — | —|
| returns null for malformed gzip | 511 | pending | — | — | —|
| verifies signature when public key is available | 526 | pending | — | — | —|
| returns null for invalid signature when public key is available | 561 | pending | — | — | —|
| returns null for missing signature when public key is available | 593 | pending | — | — | —|
| returns null for malformed public key when verification is enabled | 625 | pending | — | — | —|
| falls back to unsigned payload when public key response is empty | 654 | pending | — | — | —|
| uses pinned Hex public key for repo.hex.pm | 686 | pending | — | — | —|
| maps repo.hex.pm host aliases to hexpm repository checks | 714 | pending | — | — | —|
| caches public key responses for subsequent package lookups | 747 | pending | — | — | —|
| returns null for package name mismatch | 813 | pending | — | — | —|
| returns null for organization repository mismatch | 842 | pending | — | — | —|
| uses JSON API for hex.pm default registry | 871 | pending | — | — | —|

---
