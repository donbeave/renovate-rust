# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/crow/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/crow/extract.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 15 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `crow.rs` | `empty_returns_empty` | — |
| returns null for non-object YAML | 10 | ported | `crow.rs` | `no_image_keys_returns_empty` | — |
| returns null for malformed YAML | 15 | ported | `crow.rs` | `malformed_yaml_returns_empty` | — |
| extracts multiple image lines | 19 | ported | `crow.rs` | `extracts_pipeline_images` | — |
| extracts image and replaces registry | 164 | ported | `crow.rs` | `extracts_image_and_replaces_registry` | — |
| extracts image but no replacement | 194 | ported | `crow.rs` | `extracts_image_without_registry_replacement` | — |
| extracts image and no double replacement | 224 | ported | `crow.rs` | `extracts_image_without_double_registry_replacement` | — |
| extracts the 1.0.0 version | 255 | ported | `crow.rs` | `extracts_semver_version_from_steps` | — |
| should parse multiple sources of dependencies together | 281 | ported | `crow.rs` | `extracts_from_clone_and_steps_sections` | — |
| return dependency when a plugin-git is cloned | 321 | ported | `crow.rs` | `clone_section` | — |
| return null when no dependencies are provided | 348 | ported | `crow.rs` | `no_dependencies_returns_empty` | — |
| handles empty pipeline section gracefully | 362 | ported | `crow.rs` | `empty_pipeline_object_is_skipped` | — |
| returns null when pipeline keys exist but contain no valid images | 390 | ported | `crow.rs` | `pipeline_without_valid_images_returns_empty` | — |
| extracts images from array-based steps format | 408 | ported | `crow.rs` | `steps_as_array` | — |
| extracts images from mixed array and object formats | 447 | ported | `crow.rs` | `extracts_images_from_mixed_array_and_object_formats` | — |

---

