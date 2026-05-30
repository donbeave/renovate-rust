# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/config/app-strings.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/app-strings.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `config/app-strings`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds user configured filenames to list | 8 | ported | `repo_config.rs` | `config_file_names_include_user_configured_names` | — |
| expands brace patterns for json and json5 filenames | 20 | ported | `repo_config.rs` | `config_file_names_expand_json_and_json5_patterns` | — |
| filters based on platform | 33 | ported | `repo_config.rs` | `config_file_names_filter_platform_specific_names` | — |
| does not allow the local platform to have an associated filename | 42 | ported | `repo_config.rs` | `config_file_names_do_not_add_local_platform_names` | — |

---

