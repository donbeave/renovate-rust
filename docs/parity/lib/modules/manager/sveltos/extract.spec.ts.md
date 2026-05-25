# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/sveltos/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/sveltos/extract.spec.ts
**Total tests:** 13 | **Ported:** 13 | **Actionable:** 13 | **Status:** ported

### `extractDefinition()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns an empty array when parsing fails | 234 | ported | `sveltos.rs` | `extract_definition_invalid_input_returns_empty` | — |
| returns null if extractDefinition returns an empty array | 240 | ported | `sveltos.rs` | `clusterprofile_with_no_helm_charts_returns_empty` | — |

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 254 | ported | `sveltos.rs` | `empty_content_returns_empty` | — |
| returns null for invalid | 258 | ported | `sveltos.rs` | `malformed_profiles_all_empty_charts_returns_empty` | — |
| return null for Kubernetes manifest | 264 | ported | `sveltos.rs` | `skips_non_sveltos_files` | — |
| return null if deps array would be empty | 269 | ported | `sveltos.rs` | `malformed_no_charts_returns_empty` | — |
| return null if YAML is invalid | 274 | ported | `sveltos.rs` | `invalid_yaml_with_no_valid_helm_charts_returns_empty` | — |
| return result for double quoted projectsveltos.io apiVersion reference | 288 | ported | `sveltos.rs` | `double_quoted_api_version_extracted` | — |
| return result for single quoted projectsveltos.io apiVersion reference | 320 | ported | `sveltos.rs` | `single_quoted_api_version_extracted` | — |
| supports profiles | 352 | ported | `sveltos.rs` | `profile_kind_extracted` | — |
| supports clusterprofiles | 400 | ported | `sveltos.rs` | `extracts_helm_chart` (+ extracts_multiple_charts) | — |
| considers registryAliases | 451 | ported | `sveltos.rs` | `considers_registry_aliases_for_oci_charts` | — |
| supports eventtriggers | 474 | ported | `sveltos.rs` | `eventtrigger_kind_extracted` | — |

---

