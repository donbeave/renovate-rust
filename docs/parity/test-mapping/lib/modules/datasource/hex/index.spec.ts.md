# `lib/modules/datasource/hex/index.spec.ts`

[← `datasource/hex`](../../../../_by-module/datasource/hex.md) · [all modules](../../../../README.md)

**2/33 ported** (31 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 112 | returns null for empty result | pending | — |
| 122 | returns null for missing fields | ported | [`crates/renovate-core/src/datasources/hex.rs:168`](../../../../../../../crates/renovate-core/src/datasources/hex.rs#L168) |
| 135 | returns null for 404 | pending | — |
| 142 | returns null for 401 | ported | [`crates/renovate-core/src/datasources/hex.rs:200`](../../../../../../../crates/renovate-core/src/datasources/hex.rs#L200) |
| 149 | throws for 429 | pending | — |
| 156 | throws for 5xx | pending | — |
| 163 | returns null for unknown error | pending | — |
| 170 | returns null with wrong auth token | pending | — |
| 193 | processes real data | pending | — |
| 207 | process public repo without auth | pending | — |
| 222 | extracts depreceated info | pending | — |
| 235 | processes a private repo with auth | pending | — |
| 272 | extracts versions from v2 response | pending | — |
| 318 | marks retired releases as deprecated | pending | — |
| 359 | filters releases without versions | pending | — |
| 396 | handles organization packages via v2 | pending | — |
| 428 | returns null for empty releases | pending | — |
| 451 | throws for 5xx errors | pending | — |
| 466 | throws for 429 errors | pending | — |
| 481 | returns null for 404 | pending | — |
| 496 | returns null for network error | pending | — |
| 511 | returns null for malformed gzip | pending | — |
| 526 | verifies signature when public key is available | pending | — |
| 561 | returns null for invalid signature when public key is available | pending | — |
| 593 | returns null for missing signature when public key is available | pending | — |
| 625 | returns null for malformed public key when verification is enabled | pending | — |
| 654 | falls back to unsigned payload when public key response is empty | pending | — |
| 686 | uses pinned hex public key for repo.hex.pm | pending | — |
| 714 | maps repo.hex.pm host aliases to hexpm repository checks | pending | — |
| 747 | caches public key responses for subsequent package lookups | pending | — |
| 813 | returns null for package name mismatch | pending | — |
| 842 | returns null for organization repository mismatch | pending | — |
| 871 | uses json api for hex.pm default registry | pending | — |

