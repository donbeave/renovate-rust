# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/velaci/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/velaci/extract.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should handle invalid YAML | 6 | ported | `velaci.rs` | `invalid_yaml_returns_empty` (+ empty_returns_empty) | — |
| should handle YAML without pipeline/images | 11 | ported | `velaci.rs` | `yaml_without_pipeline_returns_empty` | — |
| extracts multiple step pipeline image lines | 16 | ported | `velaci.rs` | `extracts_step_image` | — |
| extracts multiple services pipeline image lines | 30 | ported | `velaci.rs` | `extracts_service_image` | — |
| extracts multiple stages pipeline image lines | 48 | ported | `velaci.rs` | `extracts_stages_pipeline_images` | — |
| extracts multiple secrets pipeline image lines | 62 | ported | `velaci.rs` | `extracts_secrets_pipeline_images` | — |

---

