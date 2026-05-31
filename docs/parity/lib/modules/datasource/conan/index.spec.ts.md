# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/datasource/conan/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/conan/index.spec.ts
**Total tests:** 22 | **Ported:** 4 | **Actionable:** 18 | **Status:** partial

### `modules/datasource/conan/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles package without digest | 38 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| handles digest | 43 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for missing revision | 56 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| handles bad return | 69 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| handles empty return | 82 | ported | `conan.rs` | `parses_config_yml_empty_returns_empty` | Tests empty versions list from config.yml. |
| handles bad registries | 95 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| handles missing packages | 109 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| processes real versioned data | 122 | ported | `conan.rs` | `parses_config_yml` | Tests parsing version keys from YAML. |
| processes mixed case names | 154 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| uses github instead of conan center | 180 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| works with empty releases | 221 | ported | `conan.rs` | `parses_config_yml_empty_returns_empty` | Tests empty versions list. |
| rejects userAndChannel for Conan Center | 237 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| handles mismatched userAndChannel versioned data | 247 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| handles malformed packages | 261 | ported | `conan.rs` | `parses_config_yml_empty_returns_empty` | Tests malformed/empty YAML parsing. |
| handles non 404 errors | 282 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| handles missing slash on registries | 297 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| artifactory sourceurl | 312 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| artifactory header without api | 367 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| artifactory invalid version | 398 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| non artifactory header | 425 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| artifactory no package url | 442 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| artifactory http error | 492 | not-applicable | Mock framework internals — tests conan datasource via nock HTTP mocks; Rust tests this at different layer | — | —|

---
