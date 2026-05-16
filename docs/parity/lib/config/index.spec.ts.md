# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/config/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/index.spec.ts
**Total tests:** 12 | **Ported:** 12 | **Actionable:** 12 | **Status:** ported

### `config/index › mergeChildConfig(parentConfig, childConfig)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| merges | 16 | ported | `config.rs` | `merge_child_config_merges_plain_and_nested_options` | — |
| merges packageRules | 32 | ported | `config.rs` | `merge_child_config_appends_package_rules` | — |
| merges constraints | 55 | ported | `config.rs` | `merge_child_config_merges_constraints` | — |
| merges forced options | 73 | ported | `config.rs` | `merge_child_config_merges_force_options` | — |
| handles null parent packageRules | 92 | ported | `config.rs` | `merge_child_config_handles_null_parent_package_rules` | — |
| handles null child packageRules | 105 | ported | `config.rs` | `merge_child_config_handles_missing_child_package_rules` | — |
| handles undefined childConfig | 118 | ported | `config.rs` | `merge_child_config_handles_undefined_child_config` | — |
| getManagerConfig() | 124 | ported | `config.rs` | `get_manager_config_adds_manager_file_patterns` | — |
| filterConfig() | 142 | ported | `config.rs` | `filter_config_returns_object` | — |
| highest vulnerabilitySeverity maintained when config is vulnerability alert | 148 | ported | `config.rs` | `merge_child_config_keeps_highest_vulnerability_severity` | — |

### `config/index › removeGlobalConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| removes all global config | 163 | ported | `config.rs` | `remove_global_config_removes_all_global_config` | — |
| retains inherited config | 170 | ported | `config.rs` | `remove_global_config_retains_inherited_config` | — |

---

