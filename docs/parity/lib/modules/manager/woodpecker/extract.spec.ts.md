# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/woodpecker/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/woodpecker/extract.spec.ts
**Total tests:** 11 | **Ported:** 11 | **Actionable:** 11 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 8 | ported | `woodpecker.rs` | `empty_returns_empty` | — |
| returns null for non-object YAML | 12 | ported | `woodpecker.rs` | `non_object_yaml_returns_empty` | — |
| returns null for malformed YAML | 17 | ported | `woodpecker.rs` | `malformed_yaml_returns_empty` | — |
| extracts multiple image lines | 21 | ported | `woodpecker.rs` | `extracts_step_image` (+ extracts_service_image, multiple_steps_and_services, steps_section_extracts_image) | — |
| extracts image and replaces registry | 129 | ported | `woodpecker.rs` | `extracts_image_and_replaces_registry` | — |
| extracts image but no replacement | 159 | ported | `woodpecker.rs` | `extracts_image_without_registry_replacement` | — |
| extracts image and no double replacement | 189 | ported | `woodpecker.rs` | `extracts_image_without_double_registry_replacement` | — |
| extracts the v.1.0.x version | 220 | ported | `woodpecker.rs` | `steps_section_extracts_image` | — |
| should parse multiple sources of dependencies together | 246 | ported | `woodpecker.rs` | `clone_and_steps_both_extracted` | — |
| return dependency when an plugin-git is cloned | 286 | ported | `woodpecker.rs` | `clone_section_extracted` | — |
| return null when no dependencies are provided | 313 | ported | `woodpecker.rs` | `no_steps_or_services_returns_empty` | — |

---

