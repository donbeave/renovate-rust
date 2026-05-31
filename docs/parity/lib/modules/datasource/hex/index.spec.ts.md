# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/hex/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/hex/index.spec.ts
**Total tests:** 33 | **Ported:** 3 | **Actionable:** 30 | **Status:** partial

### `modules/datasource/hex/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 112 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for missing fields | 122 | ported | `hex.rs` | `fetch_latest_missing_stable_version_returns_none` | — |
| returns null for 404 | 135 | ported | `hex.rs` | `fetch_latest_404_returns_none` | — |
| returns null for 401 | 142 | ported | `hex.rs` | `fetch_latest_unauthorized_returns_none` | — |
| throws for 429 | 149 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| throws for 5xx | 156 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for unknown error | 163 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null with wrong auth token | 170 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| processes real data | 193 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| process public repo without auth | 207 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| extracts depreceated info | 222 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| processes a private repo with auth | 235 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|

### `modules/datasource/hex/index › getReleases (V2 protocol)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts versions from V2 response | 272 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| marks retired releases as deprecated | 318 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| filters releases without versions | 359 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| handles organization packages via V2 | 396 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for empty releases | 428 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| throws for 5xx errors | 451 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| throws for 429 errors | 466 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for 404 | 481 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for network error | 496 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for malformed gzip | 511 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| verifies signature when public key is available | 526 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for invalid signature when public key is available | 561 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for missing signature when public key is available | 593 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for malformed public key when verification is enabled | 625 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| falls back to unsigned payload when public key response is empty | 654 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| uses pinned Hex public key for repo.hex.pm | 686 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| maps repo.hex.pm host aliases to hexpm repository checks | 714 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| caches public key responses for subsequent package lookups | 747 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for package name mismatch | 813 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for organization repository mismatch | 842 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| uses JSON API for hex.pm default registry | 871 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | —|

---
