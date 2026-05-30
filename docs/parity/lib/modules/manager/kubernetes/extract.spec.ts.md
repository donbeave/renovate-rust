# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/kubernetes/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/kubernetes/extract.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 14 | ported | `kubernetes.rs` | `returns_empty_for_empty_input` (+ returns_empty_for_non_k8s) | — |
| does not return unknown kind | 18 | ported | `kubernetes.rs` | `configmap_with_no_images_returns_empty` | — |
| extracts multiple Kubernetes configurations | 23 | ported | `kubernetes.rs` | `extracts_docker_hub_images` (+ extracts_non_docker_hub_registries) | — |
| extracts image line in a YAML array | 71 | ported | `kubernetes.rs` | `extracts_docker_hub_images` | — |
| extracts image tag when it contains underscores | 98 | ported | `kubernetes.rs` | `extracts_image_with_underscore_in_tag` | — |
| ignores non-Kubernetes YAML files | 121 | ported | `kubernetes.rs` | `ignores_non_kubernetes_yaml` | — |
| handles invalid YAML files | 125 | ported | `kubernetes.rs` | `handles_invalid_yaml_with_no_images` | — |
| extracts images and replaces registries | 133 | ported | `kubernetes.rs` | `extracts_images_and_replaces_registries` | — |
| extracts images but does no replacement | 155 | ported | `kubernetes.rs` | `extracts_images_without_registry_replacement` | — |
| extracts images and does no double replacements | 177 | ported | `kubernetes.rs` | `extracts_images_without_double_registry_replacement` | — |
| extracts from complex templates | 200 | ported | `kubernetes.rs` | `extracts_from_complex_templates` | — |
| extracts image volumes from $kind | 223 | ported | `kubernetes.rs` | `extracts_image_volumes_from_workload_kinds` | — |
| extracts image volumes from Pod and CronJob | 265 | ported | `kubernetes.rs` | `extracts_image_volumes_from_pod_and_cronjob` | — |
| does not extract image volumes for unsupported kind | 326 | ported | `kubernetes.rs` | `does_not_extract_image_volumes_for_unsupported_kind` | — |
| skips malformed volume entries and extracts valid ones | 349 | ported | `kubernetes.rs` | `skips_malformed_image_volume_entries_and_extracts_valid_ones` | — |

---

