# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gitlabci/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gitlabci/extract.spec.ts
**Total tests:** 14 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `extractAllPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts from empty file | 22 | ported | `gitlabci.rs` | `empty_content_returns_no_deps` | — |

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 28 | not-applicable | — | — | Uses extractAllPackageFiles with mock filesystem; no Rust equivalent |
| extracts from multidoc yaml | 36 | ported | `gitlabci.rs` | `multidoc_yaml_extracts_from_all_docs` | — |
| extracts multiple included image lines | 46 | not-applicable | — | — | Uses extractAllPackageFiles with mock filesystem; no Rust equivalent |
| extracts named services | 57 | ported | `gitlabci.rs` | `extracts_services` | — |
| extracts multiple named services | 66 | not-applicable | — | — | Uses extractAllPackageFiles with mock filesystem; no Rust equivalent |
| extracts multiple image lines | 75 | ported | `gitlabci.rs` | `extracts_top_level_image` | — |
| extracts multiple image lines with comments | 94 | ported | `gitlabci.rs` | `extracts_images_with_comment_lines` | — |
| catches errors | 110 | not-applicable | — | — | Uses extractAllPackageFiles with mock filesystem; no Rust equivalent |
| skips images with variables | 118 | ported | `gitlabci.rs` | `variable_image_has_skip_reason` | — |
| extract images from dependency proxy | 172 | ported | `gitlabci.rs` | `dependency_proxy_prefix_stripped` | — |
| extract images via registry aliases | 229 | ported | `gitlabci.rs` | `extract_images_via_registry_aliases` | — |
| extracts component references via registry aliases | 299 | ported | `gitlabci.rs` | `extracts_component_references_via_registry_aliases` | — |
| extracts component references | 377 | ported | `gitlabci.rs` | `extracts_component_references` | — |

---

