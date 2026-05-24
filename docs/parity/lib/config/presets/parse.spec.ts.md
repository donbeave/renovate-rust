# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/config/presets/parse.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/presets/parse.spec.ts
**Total tests:** 46 | **Ported:** 46 | **Actionable:** 46 | **Status:** ported

### `config/presets/parse › parsePreset`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns default package name | 6 | ported | `renovate_config_presets.rs` | `pp_returns_default_package_name` | — |
| parses github | 17 | ported | `renovate_config_presets.rs` | `pp_parses_github` | — |
| handles special chars | 28 | ported | `renovate_config_presets.rs` | `pp_handles_special_chars` | — |
| parses github subfiles | 39 | ported | `renovate_config_presets.rs` | `pp_parses_github_subfiles` | — |
| parses github subfiles with preset name | 50 | ported | `renovate_config_presets.rs` | `pp_parses_github_subfiles_with_preset_name` | — |
| parses github file with preset name with .json extension | 61 | ported | `renovate_config_presets.rs` | `pp_parses_github_file_with_json_extension` | — |
| parses github file with preset name with .json5 extension | 73 | ported | `renovate_config_presets.rs` | `pp_parses_github_file_with_json5_extension` | — |
| parses github subfiles with preset name with .json extension | 85 | ported | `renovate_config_presets.rs` | `pp_parses_github_subfiles_with_json_extension` | — |
| parses github subfiles with preset name with .json5 extension | 97 | ported | `renovate_config_presets.rs` | `pp_parses_github_subfiles_with_json5_extension` | — |
| parses github subfiles with preset and sub-preset name | 111 | ported | `renovate_config_presets.rs` | `pp_parses_github_subfiles_with_sub_preset_name` | — |
| parses github subdirectories | 124 | ported | `renovate_config_presets.rs` | `pp_parses_github_subdirectories` | — |
| parses github toplevel file using subdirectory syntax | 137 | ported | `renovate_config_presets.rs` | `pp_parses_github_toplevel_file_subdirectory_syntax` | — |
| parses gitlab | 148 | ported | `renovate_config_presets.rs` | `pp_parses_gitlab` | — |
| parses gitea | 159 | ported | `renovate_config_presets.rs` | `pp_parses_gitea` | — |
| parses forgejo | 170 | ported | `renovate_config_presets.rs` | `pp_parses_forgejo` | — |
| parses local | 181 | ported | `renovate_config_presets.rs` | `pp_parses_local` | — |
| parses local with spaces | 192 | ported | `renovate_config_presets.rs` | `pp_parses_local_with_spaces` | — |
| parses local with subdirectory | 203 | ported | `renovate_config_presets.rs` | `pp_parses_local_with_subdirectory` | — |
| parses local with spaces and subdirectory | 216 | ported | `renovate_config_presets.rs` | `pp_parses_local_with_spaces_and_subdirectory` | — |
| parses local with sub preset and tag | 229 | ported | `renovate_config_presets.rs` | `pp_parses_local_with_sub_preset_and_tag` | — |
| parses local with subdirectory and tag | 243 | ported | `renovate_config_presets.rs` | `pp_parses_local_with_subdirectory_and_tag` | — |
| parses local with subdirectory and branch/tag with a slash | 257 | ported | `renovate_config_presets.rs` | `pp_parses_local_with_subdirectory_and_slash_tag` | — |
| parses local with sub preset and branch/tag with a slash | 271 | ported | `renovate_config_presets.rs` | `pp_parses_local_with_sub_preset_and_slash_tag` | — |
| parses local repo with presetPath with URL-encoded characters | 285 | ported | `renovate_config_presets.rs` | `pp_parses_local_url_encoded_with_preset_path` | — |
| parses local repo with URL-encoded characters | 298 | ported | `renovate_config_presets.rs` | `pp_parses_local_url_encoded` | — |
| parses no prefix as local | 309 | ported | `renovate_config_presets.rs` | `pp_parses_no_prefix_as_local` | — |
| parses local Bitbucket user repo with preset name | 320 | ported | `renovate_config_presets.rs` | `pp_parses_local_bitbucket_user_repo_with_preset_name` | — |
| parses local Bitbucket user repo | 331 | ported | `renovate_config_presets.rs` | `pp_parses_local_bitbucket_user_repo` | — |
| returns default package name with params | 342 | ported | `renovate_config_presets.rs` | `pp_returns_default_package_name_with_params` | — |
| returns simple scope | 354 | ported | `renovate_config_presets.rs` | `pp_returns_simple_scope` | — |
| returns simple scope and params | 365 | ported | `renovate_config_presets.rs` | `pp_returns_simple_scope_and_params` | — |
| returns scope with repo and default | 376 | ported | `renovate_config_presets.rs` | `pp_returns_scope_with_repo_and_default` | — |
| returns scope with repo and params and default | 387 | ported | `renovate_config_presets.rs` | `pp_returns_scope_with_repo_and_params_and_default` | — |
| returns scope with presetName | 400 | ported | `renovate_config_presets.rs` | `pp_returns_scope_with_preset_name` | — |
| returns scope with presetName and params | 411 | ported | `renovate_config_presets.rs` | `pp_returns_scope_with_preset_name_and_params` | — |
| returns scope with repo and presetName | 422 | ported | `renovate_config_presets.rs` | `pp_returns_scope_with_repo_and_preset_name` | — |
| returns scope with repo and presetName and params | 433 | ported | `renovate_config_presets.rs` | `pp_returns_scope_with_repo_and_preset_name_and_params` | — |
| returns non-scoped default | 449 | ported | `renovate_config_presets.rs` | `pp_returns_non_scoped_default` | — |
| returns non-scoped package name | 460 | ported | `renovate_config_presets.rs` | `pp_returns_non_scoped_package_name` | — |
| returns non-scoped package name full | 471 | ported | `renovate_config_presets.rs` | `pp_returns_non_scoped_package_name_full` | — |
| returns non-scoped package name with params | 482 | ported | `renovate_config_presets.rs` | `pp_returns_non_scoped_package_name_with_params` | — |
| parses HTTPS URLs for gitea | 493 | ported | `renovate_config_presets.rs` | `pp_parses_https_urls_for_gitea` | — |
| parses HTTPS URLs for forgejo | 508 | ported | `renovate_config_presets.rs` | `pp_parses_https_urls_for_forgejo` | — |
| parses HTTP URLs | 523 | ported | `renovate_config_presets.rs` | `pp_parses_http_urls` | — |
| parses HTTPS URLs with parameters for gitea | 538 | ported | `renovate_config_presets.rs` | `pp_parses_https_urls_with_params_for_gitea` | — |
| parses HTTPS URLs with parameters for forgejo | 553 | ported | `renovate_config_presets.rs` | `pp_parses_https_urls_with_params_for_forgejo` | — |

---

