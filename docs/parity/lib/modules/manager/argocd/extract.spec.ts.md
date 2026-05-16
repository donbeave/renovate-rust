# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/argocd/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/argocd/extract.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 11 | ported | `argocd.rs` | `empty_content_returns_empty` | — |
| returns null for invalid | 15 | ported | `argocd.rs` | `invalid_yaml_with_trailing_content_returns_empty` | — |
| return null for kubernetes manifest | 21 | ported | `argocd.rs` | `skips_non_argocd_file` | — |
| return null if deps array would be empty | 26 | ported | `argocd.rs` | `malformed_applications_return_empty` | — |
| return result for double quoted argoproj.io apiVersion reference | 34 | ported | `argocd.rs` | `double_quoted_apiversion_accepted` | — |
| return result for single quoted argoproj.io apiVersion reference | 61 | ported | `argocd.rs` | `single_quoted_apiversion_accepted` | — |
| full test | 88 | ported | `argocd.rs` | `full_test_helm_source, full_test_git_source_dep_name_is_full_url, full_test_docker_source_no_protocol, full_test_oci_helm_chart, full_test_kustomize_images` | — |
| supports applicationsets | 203 | ported | `argocd.rs` | `supports_applicationsets` | — |

---

