# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/tekton/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/tekton/extract.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts deps from a file | 6 | ported | `tekton.rs` | `extracts_step_images` (+ extracts_gcr_images_without_skip) | — |
| extracts deps from a file in annotations | 15 | ported | `tekton.rs` | `extracts_annotation_task_and_pipeline_refs` | — |
| ignores file without any deps | 96 | ported | `tekton.rs` | `ignores_file_without_deps` | — |
| ignores invalid YAML | 100 | ported | `tekton.rs` | `ignores_invalid_yaml_with_stray_bundle_key` | — |
| ignores empty file | 112 | ported | `tekton.rs` | `ignores_empty_file` | — |

---

