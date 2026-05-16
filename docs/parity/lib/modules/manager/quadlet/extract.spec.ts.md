# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/quadlet/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/quadlet/extract.spec.ts
**Total tests:** 11 | **Ported:** 11 | **Actionable:** 11 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid quadlet file content | 19 | ported | `quadlet.rs` | `ignores_non_container_sections` | — |
| returns null for empty yaml file content | 24 | ported | `quadlet.rs` | `empty_returns_empty` | — |
| extracts from quadlet container unit | 29 | ported | `quadlet.rs` | `extracts_container_image` (+ skips_local_transport, skips_comment_lines, variable_ref_skipped) | — |
| extracts from quadlet image unit | 47 | ported | `quadlet.rs` | `image_section_extracted` | — |
| extracts from quadlet volume unit | 65 | ported | `quadlet.rs` | `volume_section_extracted` | — |
| handles docker prefix | 83 | ported | `quadlet.rs` | `strips_docker_transport_prefix` | — |
| handles docker-daemon prefix | 101 | ported | `quadlet.rs` | `docker_daemon_prefix_stripped` | — |
| does not extract an image file reference | 119 | ported | `quadlet.rs` | `image_file_reference_skipped` | — |
| does not extract an build file reference | 129 | ported | `quadlet.rs` | `build_file_reference_skipped` | — |
| extract data from file with registry aliases | 139 | ported | `quadlet.rs` | `applies_registry_aliases_to_package_name` | — |
| handles an unsuccessful parse | 158 | ported | `quadlet.rs` | `container_section_without_image_returns_empty` | — |

---

