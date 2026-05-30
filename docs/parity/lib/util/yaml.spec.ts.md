# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/yaml.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/yaml.spec.ts
**Total tests:** 19 | **Ported:** 12 | **Actionable:** 12 | **Status:** partial

### `util/yaml › loadAll`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array for empty string | 7 | ported | `util.rs` | `test_parse_yaml_empty` | — |
| should parse content with single document | 11 | ported | `util.rs` | `test_parse_yaml_single` | — |
| should parse content with single document with schema | 26 | pending | — | — | — |
| should parse content with multiple documents | 50 | ported | `util.rs` | `test_parse_yaml_multiple` | — |
| should parse content with multiple documents with schema | 70 | pending | — | — | — |
| should throw if schema does not match | 102 | pending | — | — | — |
| should throw if schema does not match and failureBehaviour "throw" | 122 | pending | — | — | — |
| should still return valid elements if schema does not match with "filter" behaviour | 143 | pending | — | — | — |
| should parse content with templates | 170 | ported | `util.rs` | `test_parse_yaml_templates` | — |
| should parse content with templates without quotes | 193 | ported | — | — | — |

### `util/yaml › load`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return undefined | 222 | ported | `util.rs` | `test_parse_single_yaml_empty` | — |
| should parse content with single document | 226 | ported | `util.rs` | `test_parse_single_yaml_single` | — |
| should parse invalid content using strict=false | 239 | ported | — | — | — |
| should parse content with single document with schema | 253 | pending | — | — | — |
| should throw with single document with schema if parsing fails | 275 | pending | — | — | — |
| should parse content with multiple documents | 292 | ported | — | — | — |
| should parse content with template | 303 | ported | `util.rs` | `test_parse_single_yaml_template` | — |
| should parse content with template without quotes | 326 | ported | — | — | — |
| should parse content with yaml tags | 353 | ported | `util.rs` | `test_parse_single_yaml_custom_tags` | — |

---
