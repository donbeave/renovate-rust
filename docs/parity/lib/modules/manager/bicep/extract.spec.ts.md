# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bicep/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bicep/extract.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should extract a normal resource | 5 | ported | `bicep.rs` | `extracts_resource_declaration` (+ extracts_multiple_resources, preview_version_captured) | — |
| should not extract a commented out resource | 37 | ported | `bicep.rs` | `comment_lines_skipped` (+ no_resources_returns_empty) | — |
| should extract a conditional resource | 58 | ported | `bicep.rs` | `extracts_conditional_resource` | — |
| should extract a existing resource | 90 | ported | `bicep.rs` | `extracts_existing_resource` | — |
| should extract a conditional loop resource | 117 | ported | `bicep.rs` | `extracts_conditional_loop_resource` | — |
| should extract a loop resource | 149 | ported | `bicep.rs` | `extracts_loop_resource` | — |
| should not extract a nested unversioned resource | 181 | ported | `bicep.rs` | `nested_unversioned_resource_skipped` | — |
| should not extract a nested versioned resource | 217 | ported | `bicep.rs` | `nested_versioned_resource_skipped` | — |
| should extract a sub resource | 253 | ported | `bicep.rs` | `extracts_sub_resource_with_multiple_slashes` | — |

---

