# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/datasource/conan/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/conan/index.spec.ts
**Total tests:** 22 | **Ported:** 4 | **Actionable:** 18 | **Status:** partial

### `modules/datasource/conan/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles package without digest | 38 | pending | — | — | —|
| handles digest | 43 | pending | — | — | —|
| returns null for missing revision | 56 | pending | — | — | —|
| handles bad return | 69 | pending | — | — | —|
| handles empty return | 82 | ported | `conan.rs` | `parses_config_yml_empty_returns_empty` | Tests empty versions list from config.yml. |
| handles bad registries | 95 | pending | — | — | —|
| handles missing packages | 109 | pending | — | — | —|
| processes real versioned data | 122 | ported | `conan.rs` | `parses_config_yml` | Tests parsing version keys from YAML. |
| processes mixed case names | 154 | pending | — | — | —|
| uses github instead of conan center | 180 | pending | — | — | —|
| works with empty releases | 221 | ported | `conan.rs` | `parses_config_yml_empty_returns_empty` | Tests empty versions list. |
| rejects userAndChannel for Conan Center | 237 | pending | — | — | —|
| handles mismatched userAndChannel versioned data | 247 | pending | — | — | —|
| handles malformed packages | 261 | ported | `conan.rs` | `parses_config_yml_empty_returns_empty` | Tests malformed/empty YAML parsing. |
| handles non 404 errors | 282 | pending | — | — | —|
| handles missing slash on registries | 297 | pending | — | — | —|
| artifactory sourceurl | 312 | pending | — | — | —|
| artifactory header without api | 367 | pending | — | — | —|
| artifactory invalid version | 398 | pending | — | — | —|
| non artifactory header | 425 | pending | — | — | —|
| artifactory no package url | 442 | pending | — | — | —|
| artifactory http error | 492 | pending | — | — | —|

---
