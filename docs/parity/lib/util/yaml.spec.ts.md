# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/yaml.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/yaml.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/yaml › loadAll`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty array for empty string | 7 | not-applicable | — | — | Renovate's generic YAML parser helper is not implemented as a Rust API; Rust YAML handling is extractor-specific and global config YAML support is explicitly deferred. |
| should parse content with single document | 11 | not-applicable | — | — | Renovate's generic YAML parser helper is not implemented as a Rust API; Rust YAML handling is extractor-specific. |
| should parse content with single document with schema | 26 | not-applicable | — | — | Renovate's Zod schema validation wrapper around YAML parsing has no Rust API equivalent. |
| should parse content with multiple documents | 50 | not-applicable | — | — | Renovate's generic multi-document YAML helper is not implemented as a Rust API; multi-document behavior is covered in extractor-specific specs where relevant. |
| should parse content with multiple documents with schema | 70 | not-applicable | — | — | Renovate's Zod schema validation wrapper around multi-document YAML parsing has no Rust API equivalent. |
| should throw if schema does not match | 102 | not-applicable | — | — | Renovate's Zod schema validation failure behavior has no Rust API equivalent. |
| should throw if schema does not match and failureBehaviour "throw" | 122 | not-applicable | — | — | Renovate's Zod schema validation failure behavior has no Rust API equivalent. |
| should still return valid elements if schema does not match with "filter" behaviour | 143 | not-applicable | — | — | Renovate's Zod schema filtering mode has no Rust API equivalent. |
| should parse content with templates | 170 | not-applicable | — | — | Renovate's YAML template stripping helper is not implemented as a Rust API. |
| should parse content with templates without quotes | 193 | not-applicable | — | — | Renovate's YAML template stripping helper is not implemented as a Rust API. |

### `util/yaml › load`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return undefined | 222 | not-applicable | — | — | Renovate's single-document YAML parser helper is not implemented as a Rust API. |
| should parse content with single document | 226 | not-applicable | — | — | Renovate's single-document YAML parser helper is not implemented as a Rust API. |
| should parse invalid content using strict=false | 239 | not-applicable | — | — | Renovate's YAML `strict=false` parser behavior has no Rust API equivalent. |
| should parse content with single document with schema | 253 | not-applicable | — | — | Renovate's Zod schema validation wrapper around single-document YAML parsing has no Rust API equivalent. |
| should throw with single document with schema if parsing fails | 275 | not-applicable | — | — | Renovate's Zod schema validation failure behavior has no Rust API equivalent. |
| should parse content with multiple documents | 292 | not-applicable | — | — | Renovate's single-document YAML helper error for multiple documents has no Rust API equivalent. |
| should parse content with template | 303 | not-applicable | — | — | Renovate's YAML template stripping helper is not implemented as a Rust API. |
| should parse content with template without quotes | 326 | not-applicable | — | — | Renovate's YAML template stripping helper is not implemented as a Rust API. |
| should parse content with yaml tags | 353 | not-applicable | — | — | Renovate's YAML tag coercion behavior has no shared Rust API equivalent. |

---

