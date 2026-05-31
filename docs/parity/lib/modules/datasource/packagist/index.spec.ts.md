# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/datasource/packagist/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/packagist/index.spec.ts
**Total tests:** 17 | **Ported:** 4 | **Actionable:** 13 | **Status:** partial

### `modules/datasource/packagist/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports custom registries | 39 | pending | — | — | —|
| supports plain packages | 56 | ported | `packagist.rs` | `fetch_latest_returns_first_stable` | Tests basic p2 fetch with stable version. |
| handles timeouts | 80 | pending | — | — | —|
| handles auth rejections | 102 | pending | — | — | —|
| handles not found registries | 124 | ported | `packagist.rs` | `fetch_latest_404_returns_none` | — |
| supports includes packages | 146 | pending | — | — | —|
| supports older sha1 hashes | 179 | pending | — | — | —|
| supports lazy repositories | 240 | pending | — | — | —|
| supports provider-includes | 279 | pending | — | — | —|
| handles provider-includes miss | 324 | pending | — | — | —|
| supports providers | 372 | pending | — | — | —|
| supports providers without a hash | 405 | pending | — | — | —|
| handles providers miss | 434 | pending | — | — | —|
| processes real versioned data | 470 | ported | `packagist.rs` | `fetch_latest_returns_first_stable` | Tests p2 JSON parsing and version extraction. |
| adds packagist source implicitly | 490 | pending | — | — | —|
| fetches packagist V2 packages | 510 | ported | `packagist.rs` | `fetch_latest_returns_first_stable` | Rust uses the p2 (V2) endpoint. |
| respects "available-packages" list | 546 | pending | — | — | —|

---
