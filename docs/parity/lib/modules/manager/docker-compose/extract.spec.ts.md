# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/docker-compose/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/docker-compose/extract.spec.ts
**Total tests:** 13 | **Ported:** 13 | **Actionable:** 13 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 12 | ported | `docker_compose.rs` | `empty_content_returns_empty` | — |
| returns null for non-object YAML | 16 | ported | `docker_compose.rs` | `non_object_yaml_returns_empty` | — |
| returns null for malformed YAML | 20 | ported | `docker_compose.rs` | `malformed_yaml_returns_empty` | — |
| extracts multiple image lines for version 1 | 24 | ported | `docker_compose.rs` | `renovate_fixture_1_v1_format` | — |
| extracts multiple image lines for version 3 | 30 | ported | `docker_compose.rs` | `extracts_images_from_compose_v3` | — |
| extracts multiple image lines for version 3 without set version key | 36 | ported | `docker_compose.rs` | `no_version_key_extracts_eight_deps` | — |
| extracts default variable values for version 3 | 42 | ported | `docker_compose.rs` | `variable_interpolation_is_skipped` | — |
| extracts can parse yaml tags for version 3 | 59 | ported | `docker_compose.rs` | `yaml_tags_do_not_break_extraction` | — |
| extracts image and replaces registry | 87 | ported | `docker_compose.rs` | `extracts_image_and_replaces_registry` | — |
| extracts image but no replacement | 115 | ported | `docker_compose.rs` | `extracts_image_without_registry_replacement` | — |
| extracts image and no double replacement | 143 | ported | `docker_compose.rs` | `extracts_image_without_double_registry_replacement` | — |
| extracts image of templated compose file | 172 | ported | `docker_compose.rs` | `extracts_image_from_templated_compose_file` | — |
| extract images from fragments | 198 | ported | `docker_compose.rs` | `extracts_image_from_yaml_anchor_fragment` | — |

---

