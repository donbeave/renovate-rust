# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/helm-requirements/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/helm-requirements/extract.spec.ts
**Total tests:** 11 | **Ported:** 11 | **Actionable:** 11 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ensure that currentValue is string | 8 | ported | `helm.rs` | `at_alias_skipped` | — |
| skips invalid registry urls | 34 | ported | `helm.rs` | `oci_registry_skipped` | — |
| parses simple requirements.yaml correctly | 64 | ported | `helm.rs` | `simple_chart_yaml` (+ requirements_yaml_format) | — |
| parses simple requirements.yaml but skips if necessary fields missing | 96 | ported | `helm.rs` | `no_dependencies_returns_empty` | — |
| resolves aliased registry urls | 112 | ported | `helm.rs` | `stable_alias_resolved` | — |
| skips local dependencies | 141 | ported | `helm.rs` | `local_file_dependency_skipped` | — |
| returns null if no dependencies | 172 | ported | `helm.rs` | `no_dependencies_returns_empty` | — |
| returns null if requirements.yaml is invalid | 192 | ported | `helm.rs` | `invalid_yaml_returns_empty` | — |
| returns null if Chart.yaml is empty | 214 | ported | `helm.rs` | `empty_content_returns_empty` | — |
| validates ${fieldName} is required | 279 | ported | `helm.rs` | `no_repository_skipped` (+ missing_version_dep_skipped, dep_without_name_is_silently_skipped) | — |
| skips only invalid dependences | 293 | ported | `helm.rs` | `skips_only_invalid_deps_keeps_valid_ones` | — |

---

