# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/helmv3/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helmv3/extract.spec.ts
**Total tests:** 12 | **Ported:** 12 | **Actionable:** 12 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips invalid registry urls | 16 | ported | `helm.rs` | `extract_skips_invalid_registry_urls` | — |
| parses simple Chart.yaml correctly | 40 | ported | `helm.rs` | `extract_parses_simple_chart_yaml` | — |
| extract correctly oci references | 67 | ported | `helm.rs` | `extract_oci_references` | — |
| resolves aliased registry urls | 100 | ported | `helm.rs` | `extract_resolves_aliased_registry_urls` | — |
| doesn't fail if Chart.yaml is invalid | 131 | ported | `helm.rs` | `extract_returns_none_for_invalid_chart_yaml` | — |
| skips local dependencies | 142 | ported | `helm.rs` | `extract_skips_local_dependencies` | — |
| returns null if no dependencies key | 167 | ported | `helm.rs` | `extract_returns_none_if_no_dependencies_key` | — |
| returns null if dependencies are an empty list | 183 | ported | `helm.rs` | `extract_returns_none_if_dependencies_empty_list` | — |
| returns null if dependencies key is invalid | 199 | ported | `helm.rs` | `extract_returns_none_if_dependencies_invalid` | — |
| returns null if Chart.yaml is empty | 215 | ported | `helm.rs` | `extract_returns_none_if_chart_yaml_empty` | — |
| returns null if Chart.yaml uses an unsupported apiVersion | 222 | ported | `helm.rs` | `extract_returns_none_if_unsupported_api_version` | — |
| returns null if name and version are missing for all dependencies | 235 | ported | `helm.rs` | `extract_returns_none_if_all_deps_missing_name_version` | — |

---

