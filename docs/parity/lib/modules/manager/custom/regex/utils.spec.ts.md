# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/custom/regex/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/custom/regex/utils.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not crash for lazy regex | 5 | ported | `managers.rs` | `regex_match_all_does_not_crash_for_lazy_regex` | — |

| sets registryUrls when registryUrl group is a valid URL | 27 | ported | `repo_config.rs` | `custom_manager_sets_registry_urls_when_valid_url` | — |
| warns and skips registryUrls when registryUrl group is an invalid URL | 39 | ported | `repo_config.rs` | `custom_manager_skips_registry_url_when_invalid` | — |
| sets datasource when datasource group is provided | 55 | ported | `repo_config.rs` | `custom_manager_sets_datasource_from_capture_group` | — |
| sets indentation when indentation group is whitespace | 67 | ported | `repo_config.rs` | `custom_manager_sets_indentation_from_whitespace_group` | — |
| sets empty indentation when indentation group is non-whitespace | 79 | ported | `repo_config.rs` | `custom_manager_sets_empty_indentation_for_non_whitespace_group` | — |
| sets depName via default branch | 91 | ported | `repo_config.rs` | `custom_manager_sets_dep_name_from_capture_group` | — |
---

