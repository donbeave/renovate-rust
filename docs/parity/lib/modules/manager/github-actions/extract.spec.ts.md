# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/github-actions/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/github-actions/extract.spec.ts
**Total tests:** 28 | **Ported:** 28 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty | 42 | ported | `github_actions.rs` | `empty_content_returns_empty` | — |
| returns null for invalid yaml | 48 | ported | `github_actions.rs` | `invalid_yaml_returns_empty` | — |
| extracts multiple docker image lines from yaml configuration file | 54 | ported | `github_actions.rs` | `docker_container_inline` (+ 5 others) | — |
| extracts multiple action tag lines from yaml configuration file | 65 | ported | `github_actions.rs` | `extracts_simple_action` | — |
| use github.com as registry when no settings provided | 79 | ported | `github_actions.rs` | `use_github_com_as_registry_when_no_settings_provided` | — |
| use github.enterprise.com first and then github.com as registry running against github.enterprise.com | 87 | ported | `github_actions.rs` | `use_enterprise_registry_when_endpoint_is_enterprise` | — |
| use github.enterprise.com first and then github.com as registry running against github.enterprise.com/api/v3 | 102 | ported | `github_actions.rs` | `use_enterprise_registry_when_endpoint_has_api_v3_path` | — |
| use github.com only as registry when running against non-GitHub | 117 | ported | `github_actions.rs` | `use_no_custom_registry_when_platform_is_not_github` | — |
| use github.com only as registry when running against github.com | 129 | ported | `github_actions.rs` | `use_no_custom_registry_when_endpoint_is_github_com` | — |
| use github.com only as registry when running against api.github.com | 141 | ported | `github_actions.rs` | `use_no_custom_registry_when_endpoint_is_api_github_com` | — |
| extracts multiple action tag lines with double quotes and comments | 153 | ported | `github_actions.rs` | `quoted_action_is_parsed` | — |
| maintains quotes | 217 | ported | `github_actions.rs` | `single_and_double_quoted_uses_parsed` | — |
| maintains spaces between hash and comment | 299 | ported | `github_actions.rs` | `inline_comment_stripped` | — |
| extracts tags in different formats | 352 | ported | `github_actions.rs` | `comment_version_formats` | — |
| extracts non-semver ref automatically | 484 | ported | `github_actions.rs` | `non_semver_ref_extracted` | — |
| extracts pinned non-semver ref with digest | 504 | ported | `github_actions.rs` | `pinned_non_semver_ref_with_digest` | — |
| disables naked SHA pins without version comment | 527 | ported | `github_actions.rs` | `full_sha_pin_skipped` | — |
| disables naked short SHA pins without version comment | 546 | ported | `github_actions.rs` | `short_sha_pin_skipped` | — |
| does not disable SHA pins with version comment | 565 | ported | `github_actions.rs` | `full_sha_with_version_comment_not_skipped` | — |
| does not disable short SHA pins with version comment | 590 | ported | `github_actions.rs` | `short_sha_with_version_comment_not_skipped` | — |
| extracts actions with fqdn | 614 | ported | `github_actions.rs` | `extracts_actions_with_fqdn` | — |
| extracts multiple action runners from yaml configuration file | 673 | ported | `github_actions.rs` | `runner_simple_ubuntu` (+ 4 others) | — |
| extracts x-version from actions/setup-x | 741 | ported | `github_actions.rs` | `setup_x_extracts_versioned_deps` | — |
| handles actions/setup-x without x-version field | 873 | ported | `github_actions.rs` | `setup_x_without_version_returns_only_action_dep`, `setup_x_missing_version_key_emits_unspecified` | — |
| extracts x-version from actions/setup-x in composite action | 891 | ported | `github_actions.rs` | `setup_x_composite_action` | — |
| logs unknown schema | 1055 | ported | `github_actions.rs` | `logs_unknown_schema_returns_empty` | — |
| extract from $step.uses | 1063 | ported | `github_actions.rs` | `community_trivy_*`, `community_pnpm_*`, `community_bun_*`, `community_ruby_*`, `community_pyright_*`, `community_jaxxstorm_*`, `community_pixi_*`, `community_zizmor_*`, `community_docker_*`, `community_setup_uv_*` (14 tests) | — |
| returns undefined registryUrls when endpoint is invalid URL | 153 | ported | `github_actions.rs` | `returns_no_registry_urls_when_endpoint_is_invalid` | — |
---

