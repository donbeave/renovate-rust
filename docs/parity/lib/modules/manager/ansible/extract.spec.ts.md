# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/ansible/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ansible/extract.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 6 | ported | `ansible.rs` | `empty_returns_empty` | — |
| extracts multiple image lines from docker_container | 10 | ported | `ansible.rs` | `extracts_images` | — |
| extracts multiple image lines from docker_service | 16 | ported | `ansible.rs` | `extracts_docker_service_images` | — |
| extracts image and replaces registry | 22 | ported | `ansible.rs` | `extracts_image_and_replaces_registry` | — |
| extracts image but no replacement | 52 | ported | `ansible.rs` | `extracts_image_without_registry_replacement` | — |
| extracts image and no double replacement | 82 | ported | `ansible.rs` | `extracts_image_without_double_registry_replacement` | — |

---

