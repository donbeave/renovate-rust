# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/renovate-config/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/renovate-config/extract.spec.ts
**Total tests:** 20 | **Ported:** 20 | **Actionable:** 20 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty file | 7 | ported | `renovate_config_presets.rs` | `epf_returns_null_for_empty_file` | — |
| returns null for invalid file | 11 | ported | `renovate_config_presets.rs` | `epf_returns_null_for_invalid_file` | — |

### `extractPackageFile() › presets`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for a config file without presets | 18 | ported | `renovate_config_presets.rs` | `epf_returns_null_without_presets` | — |
| returns null for a config file only contains built-in presets | 34 | ported | `renovate_config_presets.rs` | `epf_returns_null_for_only_builtin_presets` | — |
| provides skipReason for unsupported preset sources | 50 | ported | `renovate_config_presets.rs` | `epf_skip_reason_for_unsupported_preset_sources` | — |
| provides skipReason for presets without versions | 88 | ported | `renovate_config_presets.rs` | `epf_skip_reason_for_presets_without_versions` | — |
| extracts from a config file with GitHub hosted presets | 120 | ported | `renovate_config_presets.rs` | `epf_extracts_github_presets` | — |
| extracts from a config file with GitLab hosted presets | 161 | ported | `renovate_config_presets.rs` | `epf_extracts_gitlab_presets` | — |
| extracts from a config file with Gitea hosted presets | 202 | ported | `renovate_config_presets.rs` | `epf_extracts_gitea_presets` | — |
| supports JSON5 | 243 | ported | `renovate_config_presets.rs` | `epf_supports_json5_presets` | — |

### `extractPackageFile() › constraints`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for a config file without constraints | 269 | ported | `renovate_config_presets.rs` | `epf_returns_null_without_constraints` | — |
| returns null for a config file has an empty constraints | 282 | ported | `renovate_config_presets.rs` | `epf_returns_null_for_empty_constraints` | — |
| extracts known `ToolName`s with explicit versions | 295 | ported | `renovate_config_presets.rs` | `epf_extracts_known_toolnames_explicit_versions` | — |
| extracts known `ToolName`s with ranges versions | 332 | ported | `renovate_config_presets.rs` | `epf_extracts_known_toolnames_range_versions` | — |
| extracts `ToolName`s from packageRules | 369 | ported | `renovate_config_presets.rs` | `epf_extracts_toolnames_from_package_rules` | — |
| handles no `constraints` in packageRules | 421 | ported | `renovate_config_presets.rs` | `epf_handles_no_constraints_in_package_rules` | — |
| sets skipReason=unsupported for a constraint that is not a tool | 451 | ported | `renovate_config_presets.rs` | `epf_skip_reason_unsupported_for_unknown_constraint` | — |
| extracts known `ToolName`s with ranges versions | 476 | ported | `renovate_config_presets.rs` | `epf_extracts_toolnames_range_versions_476` | — |
| supports JSON5 | 513 | ported | `renovate_config_presets.rs` | `epf_supports_json5_constraints` | — |
| extracts all types of configuration | 543 | ported | `renovate_config_presets.rs` | `epf_extracts_all_types_of_configuration` | — |

---

