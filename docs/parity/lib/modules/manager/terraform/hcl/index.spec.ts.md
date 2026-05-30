# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/terraform/hcl/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terraform/hcl/index.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `parseHCL()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return flat modules | 11 | not-applicable | — | — | Tests upstream JS HCL parser library (hcl2-parser) producing structured AST; Rust uses line-oriented scanner for terraform extraction, no full HCL AST parser |
| should return nested terraform block | 53 | not-applicable | — | — | Tests JS HCL parser producing nested terraform/required_providers AST; Rust scanner extracts deps directly |
| should return resource blocks | 70 | not-applicable | — | — | Tests JS HCL parser producing resource block AST with nested objects; Rust scanner extracts deps directly |

### `parseJSON`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should parse json | 101 | not-applicable | — | — | Tests JSON TF file parsing through JS HCL parser; Rust uses serde_json directly for JSON TF files |

---
