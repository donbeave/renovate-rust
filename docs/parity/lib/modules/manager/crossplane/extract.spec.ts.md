# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/crossplane/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/crossplane/extract.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 9 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 12 | ported | `crossplane.rs` | `empty_content_returns_empty` | — |
| strips invalid templates | 16 | ported | `crossplane.rs` | `invalid_template_returns_empty` | — |
| return null for kubernetes manifest | 20 | ported | `crossplane.rs` | `skips_non_crossplane_files` | — |
| return invalid-value if deps are not valid images and ignore if missing | 25 | ported | `crossplane.rs` | `malformed_packages_produce_invalid_value_dep` | — |
| return result for double quoted pkg.crossplane.io apiVersion reference | 37 | ported | `crossplane.rs` | `double_quoted_api_version_extracted` | — |
| return result for single quoted pkg.crossplane.io apiVersion reference | 58 | ported | `crossplane.rs` | `single_quoted_api_version_extracted` | — |
| return no results for invalid resource | 79 | ported | `crossplane.rs` | `reports_missing_package` | — |
| full test | 94 | ported | `crossplane.rs` | `extracts_valid_packages_full_test` | — |
| should work even if there are other resources in the file | 137 | ported | `crossplane.rs` | `handles_multi_document` | — |

---

