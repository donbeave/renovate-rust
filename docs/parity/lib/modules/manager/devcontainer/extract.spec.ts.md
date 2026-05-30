# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/devcontainer/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/devcontainer/extract.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when the dev container JSON file is empty | 10 | ported | `devcontainer.rs` | `empty_content_returns_empty` | — |
| returns null when the dev container JSON file contents are malformed | 22 | ported | `devcontainer.rs` | `invalid_json_returns_empty` | — |
| tests if JSONC can be parsed | 34 | ported | `devcontainer.rs` | `jsonc_with_comments_and_trailing_commas` | — |
| returns feature image deps when only the features property is defined in dev container JSON file | 72 | ported | `devcontainer.rs` | `extracts_node_feature_and_version` | — |
| returns image and feature image deps when both image and features properties are defined in dev container JSON file | 124 | ported | `devcontainer.rs` | `image_and_feature_combined` | — |
| returns image dep when only the image property is defined in dev container JSON file | 174 | ported | `devcontainer.rs` | `extracts_image` | — |
| returns null when the only feature property is malformed and no image property is defined in dev container JSON file | 207 | ported | `devcontainer.rs` | `malformed_feature_key_returns_empty` | — |
| returns null when the features property is malformed and no image property is defined in dev container JSON file | 227 | ported | `devcontainer.rs` | `features_as_string_returns_empty` | — |
| returns null when the image property is malformed and no features are defined in dev container JSON file | 245 | ported | `devcontainer.rs` | `typo_in_image_key_returns_empty` | — |
| returns null when no image or features properties are defined in dev container JSON file | 263 | ported | `devcontainer.rs` | `empty_object_returns_empty` | — |
| returns null when the features property is null and no image property is defined in dev container JSON file | 278 | ported | `devcontainer.rs` | `null_features_value_returns_empty` | — |
| returns null when the features property is not defined and the image property is null in dev container JSON file | 296 | ported | `devcontainer.rs` | `no_image_returns_empty` | — |
| returns null when both the image and features properties are null | 314 | ported | `devcontainer.rs` | `both_null_returns_empty` | — |
| returns only docker dependencies when non-docker feature types are defined beneath the features property in dev container JSON file | 333 | ported | `devcontainer.rs` | `feature_without_version_skipped_from_version_deps` (+ `local_feature_path_excluded_from_version_deps`) | — |
| parses known tool versions | 372 | ported | `devcontainer.rs` | `extracts_go_feature_and_version` | — |

---

