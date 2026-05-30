# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/tflint-plugin/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/tflint-plugin/extract.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 22 | ported | `tflint_plugin.rs` | `no_plugins_returns_empty` | — |
| returns null when there are no version | 28 | ported | `tflint_plugin.rs` | `missing_version_sets_skip_reason` (+ plugin_without_source_or_version_gets_missing_source) | — |
| extracts plugins | 38 | ported | `tflint_plugin.rs` | `extracts_github_plugin` (+ extracts_multiple_plugins, extracts_plugins_with_org_paths) | — |
| extracts from full configuration | 71 | ported | `tflint_plugin.rs` | `extracts_plugin_from_full_config` | — |
| extracts no source | 112 | ported | `tflint_plugin.rs` | `plugins_without_source_get_missing_source_skip` | — |
| extracts nothing if not from github | 138 | ported | `tflint_plugin.rs` | `non_github_source_skipped` (+ gitlab_source_gets_unsupported_datasource) | — |

---

