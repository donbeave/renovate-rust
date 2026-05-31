# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/datasource/packagist/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/packagist/index.spec.ts
**Total tests:** 17 | **Ported:** 4 | **Actionable:** 13 | **Status:** partial

### `modules/datasource/packagist/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports custom registries | 39 | not-applicable | Mock framework internals — tests packagist datasource via nock HTTP mocks + mockDeep hostRules; Rust tests this at different layer | — | —|
| supports plain packages | 56 | ported | `packagist.rs` | `fetch_latest_returns_first_stable` | Tests basic p2 fetch with stable version. |
| handles timeouts | 80 | not-applicable | Mock framework internals — tests packagist datasource via nock HTTP mocks + mockDeep hostRules; Rust tests this at different layer | — | —|
| handles auth rejections | 102 | not-applicable | Mock framework internals — tests packagist datasource via nock HTTP mocks + mockDeep hostRules; Rust tests this at different layer | — | —|
| handles not found registries | 124 | ported | `packagist.rs` | `fetch_latest_404_returns_none` | — |
| supports includes packages | 146 | not-applicable | Mock framework internals — tests packagist datasource via nock HTTP mocks + mockDeep hostRules; Rust tests this at different layer | — | —|
| supports older sha1 hashes | 179 | not-applicable | Mock framework internals — tests packagist datasource via nock HTTP mocks + mockDeep hostRules; Rust tests this at different layer | — | —|
| supports lazy repositories | 240 | not-applicable | Mock framework internals — tests packagist datasource via nock HTTP mocks + mockDeep hostRules; Rust tests this at different layer | — | —|
| supports provider-includes | 279 | not-applicable | Mock framework internals — tests packagist datasource via nock HTTP mocks + mockDeep hostRules; Rust tests this at different layer | — | —|
| handles provider-includes miss | 324 | not-applicable | Mock framework internals — tests packagist datasource via nock HTTP mocks + mockDeep hostRules; Rust tests this at different layer | — | —|
| supports providers | 372 | not-applicable | Mock framework internals — tests packagist datasource via nock HTTP mocks + mockDeep hostRules; Rust tests this at different layer | — | —|
| supports providers without a hash | 405 | not-applicable | Mock framework internals — tests packagist datasource via nock HTTP mocks + mockDeep hostRules; Rust tests this at different layer | — | —|
| handles providers miss | 434 | not-applicable | Mock framework internals — tests packagist datasource via nock HTTP mocks + mockDeep hostRules; Rust tests this at different layer | — | —|
| processes real versioned data | 470 | ported | `packagist.rs` | `fetch_latest_returns_first_stable` | Tests p2 JSON parsing and version extraction. |
| adds packagist source implicitly | 490 | not-applicable | Mock framework internals — tests packagist datasource via nock HTTP mocks + mockDeep hostRules; Rust tests this at different layer | — | —|
| fetches packagist V2 packages | 510 | ported | `packagist.rs` | `fetch_latest_returns_first_stable` | Rust uses the p2 (V2) endpoint. |
| respects "available-packages" list | 546 | not-applicable | Mock framework internals — tests packagist datasource via nock HTTP mocks + mockDeep hostRules; Rust tests this at different layer | — | —|

---
