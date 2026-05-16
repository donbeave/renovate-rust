# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/packagist/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/packagist/index.spec.ts
**Total tests:** 17 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/packagist/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports custom registries | 39 | not-applicable | — | — | Renovate's Packagist/Composer repository resolver, auth handling, provider includes, and full release-list mapping are not implemented in Rust; Rust only exposes direct P2 latest-version lookup. |
| supports plain packages | 56 | not-applicable | — | — | Renovate's Packagist/Composer repository resolver, auth handling, provider includes, and full release-list mapping are not implemented in Rust; Rust only exposes direct P2 latest-version lookup. |
| handles timeouts | 80 | not-applicable | — | — | Renovate's Packagist/Composer timeout and null-on-error contract is not implemented in Rust. |
| handles auth rejections | 102 | not-applicable | — | — | Renovate's Packagist/Composer auth rejection handling is not implemented in Rust. |
| handles not found registries | 124 | not-applicable | — | — | Renovate's Packagist/Composer repository resolver and not-found fallback contract are not implemented in Rust. |
| supports includes packages | 146 | not-applicable | — | — | Renovate's Composer `includes` repository resolver is not implemented in Rust. |
| supports older sha1 hashes | 179 | not-applicable | — | — | Renovate's Composer repository hash validation is not implemented in Rust. |
| supports lazy repositories | 240 | not-applicable | — | — | Renovate's Composer lazy repository resolver is not implemented in Rust. |
| supports provider-includes | 279 | not-applicable | — | — | Renovate's Composer provider-includes resolver is not implemented in Rust. |
| handles provider-includes miss | 324 | not-applicable | — | — | Renovate's Composer provider-includes miss handling is not implemented in Rust. |
| supports providers | 372 | not-applicable | — | — | Renovate's Composer providers resolver is not implemented in Rust. |
| supports providers without a hash | 405 | not-applicable | — | — | Renovate's Composer providers resolver is not implemented in Rust. |
| handles providers miss | 434 | not-applicable | — | — | Renovate's Composer providers miss handling is not implemented in Rust. |
| processes real versioned data | 470 | not-applicable | — | — | Renovate's Packagist full release-list, gitRef, and source metadata mapping are not implemented in Rust; Rust only returns latest stable version and timestamp. |
| adds packagist source implicitly | 490 | not-applicable | — | — | Renovate's Packagist source URL inference is not implemented in Rust. |
| fetches packagist V2 packages | 510 | not-applicable | — | — | Renovate's Packagist V2 full release-list mapping is not implemented in Rust; Rust only returns latest stable version and timestamp. |
| respects "available-packages" list | 546 | not-applicable | — | — | Renovate's Composer `available-packages` repository optimization is not implemented in Rust. |

---

