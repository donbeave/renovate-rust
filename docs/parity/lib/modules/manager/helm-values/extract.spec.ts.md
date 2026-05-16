# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/helm-values/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helm-values/extract.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid yaml file content | 26 | ported | `helm_values.rs` | `invalid_yaml_returns_empty` | — |
| returns null for empty yaml file content | 31 | ported | `helm_values.rs` | `empty_returns_empty` | — |
| extracts from values.yaml correctly with same structure as "helm create" | 36 | ported | `helm_values.rs` | `helm_create_default_values` | — |
| extracts from complex values file correctly | 52 | ported | `helm_values.rs` | `inline_string_form` (+ `multiple_images`) | — |
| extract data from file with multiple documents | 62 | ported | `helm_values.rs` | `multidoc_yaml_extracts_nested_images` | — |
| extract data from file with registry aliases | 85 | ported | `helm_values.rs` | `registry_prefix_combined` | — |

---

