# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/github-actions/parse.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/github-actions/parse.spec.ts
**Total tests:** 53 | **Ported:** 53 | **Actionable:** 53 | **Status:** ported

### `modules/manager/github-actions/parse › parseActionReference`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty string | 11 | ported | `github_actions.rs` | `parse_action_reference_returns_none_for_empty_string` | — |
| returns null for empty docker reference | 16 | ported | `github_actions.rs` | `parse_action_reference_returns_none_for_empty_docker_reference` | — |
| parses docker image with digest | 20 | ported | `github_actions.rs` | `parse_action_reference_parses_docker_image_with_digest` | — |
| parses docker image with tag | 29 | ported | `github_actions.rs` | `parse_action_reference_parses_docker_image_with_tag` | — |
| parses docker image with registry port and tag | 38 | ported | `github_actions.rs` | `parse_action_reference_parses_docker_image_with_registry_port_and_tag` | — |
| parses docker image without tag or digest | 51 | ported | `github_actions.rs` | `parse_action_reference_parses_docker_image_without_tag_or_digest` | — |
| parses docker image with registry but no tag | 59 | ported | `github_actions.rs` | `parse_action_reference_parses_docker_image_with_registry_but_no_tag` | — |
| parses ./ local reference | 69 | ported | `github_actions.rs` | `parse_action_reference_parses_dot_slash_local_reference` | — |
| parses ../ local reference | 76 | ported | `github_actions.rs` | `parse_action_reference_parses_dot_dot_slash_local_reference` | — |
| returns null for invalid format | 85 | ported | `github_actions.rs` | `parse_action_reference_returns_none_for_invalid_repository_format` | — |
| parses owner/repo@ref with default hostname | 90 | ported | `github_actions.rs` | `parse_action_reference_parses_owner_repo_ref_with_default_hostname` | — |
| parses owner/repo/path@ref | 102 | ported | `github_actions.rs` | `parse_action_reference_parses_owner_repo_path_ref` | — |
| parses https://host/owner/repo@ref with explicit hostname | 114 | ported | `github_actions.rs` | `parse_action_reference_parses_https_owner_repo_ref_with_explicit_hostname` | — |
| parses https://host/owner/repo/path@ref | 128 | ported | `github_actions.rs` | `parse_action_reference_parses_https_owner_repo_path_ref` | — |

### `modules/manager/github-actions/parse › parseComment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns ratchetExclude for ratchet:exclude | 147 | ported | `github_actions.rs` | `parse_comment_returns_ratchet_exclude_for_ratchet_exclude` | — |
| returns empty object for no match | 154 | ported | `github_actions.rs` | `parse_comment_returns_empty_object_for_no_match` | — |
| parses pinned version with tag= prefix | 159 | ported | `github_actions.rs` | `parse_comment_parses_pinned_version_with_tag_prefix` | — |
| parses pinned version with pin prefix | 168 | ported | `github_actions.rs` | `parse_comment_parses_pinned_version_with_pin_prefix` | — |
| parses pinned version with renovate: prefix | 177 | ported | `github_actions.rs` | `parse_comment_parses_pinned_version_with_renovate_prefix` | — |
| parses pinned version with renovate:pin prefix | 186 | ported | `github_actions.rs` | `parse_comment_parses_pinned_version_with_renovate_pin_prefix` | — |
| parses bare version | 195 | ported | `github_actions.rs` | `parse_comment_parses_bare_version` | — |
| parses version with @ prefix | 204 | ported | `github_actions.rs` | `parse_comment_parses_version_with_at_prefix` | — |
| parses ratchet pinned version | 213 | ported | `github_actions.rs` | `parse_comment_parses_ratchet_pinned_version` | — |
| parses version without v prefix | 222 | ported | `github_actions.rs` | `parse_comment_parses_version_without_v_prefix` | — |
| parses version with leading whitespace | 231 | ported | `github_actions.rs` | `parse_comment_parses_version_with_leading_whitespace` | — |
| parses prefixed version like node/v20 | 240 | ported | `github_actions.rs` | `parse_comment_parses_prefixed_version_like_node_v20` | — |
| parses prerelease version like v2.2-rc.1 | 249 | ported | `github_actions.rs` | `parse_comment_parses_prerelease_version_like_v2_2_rc_1` | — |
| parses full semver prerelease version like v2.2.0-rc.1 | 258 | ported | `github_actions.rs` | `parse_comment_parses_full_semver_prerelease_version_like_v2_2_0_rc_1` | — |
| parses bare non-semver ref | 267 | ported | `github_actions.rs` | `parse_comment_parses_bare_non_semver_ref` | — |
| parses bare branch name | 276 | ported | `github_actions.rs` | `parse_comment_parses_bare_branch_name` | — |
| ignores multi-word comments | 285 | ported | `github_actions.rs` | `parse_comment_ignores_multi_word_comments` | — |

### `modules/manager/github-actions/parse › parseQuote`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty quote for unquoted string | 291 | ported | `github_actions.rs` | `parse_quote_returns_empty_quote_for_unquoted_string` | — |
| returns empty quote for empty string | 295 | ported | `github_actions.rs` | `parse_quote_returns_empty_quote_for_empty_string` | — |
| returns empty quote for single char | 299 | ported | `github_actions.rs` | `parse_quote_returns_empty_quote_for_single_char` | — |
| parses double quoted string | 303 | ported | `github_actions.rs` | `parse_quote_parses_double_quoted_string` | — |
| parses single quoted string | 307 | ported | `github_actions.rs` | `parse_quote_parses_single_quoted_string` | — |
| handles whitespace around quotes | 311 | ported | `github_actions.rs` | `parse_quote_handles_whitespace_around_quotes` | — |
| returns empty quote for mismatched quotes | 315 | ported | `github_actions.rs` | `parse_quote_returns_empty_quote_for_mismatched_quotes` | — |
| returns empty quote for only opening quote | 320 | ported | `github_actions.rs` | `parse_quote_returns_empty_quote_for_only_opening_quote` | — |

### `modules/manager/github-actions/parse › parseUsesLine`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for non-uses lines | 326 | ported | `github_actions.rs` | `parse_uses_line_returns_none_for_non_uses_lines` | — |
| returns null when value is only a comment | 333 | ported | `github_actions.rs` | `parse_uses_line_returns_none_when_value_is_only_a_comment` | — |
| parses simple uses line without comment | 337 | ported | `github_actions.rs` | `parse_uses_line_parses_simple_uses_line_without_comment` | — |
| parses uses line with - prefix | 359 | ported | `github_actions.rs` | `parse_uses_line_parses_uses_line_with_dash_prefix` | — |
| parses uses line with comment | 381 | ported | `github_actions.rs` | `parse_uses_line_parses_uses_line_with_comment` | — |
| parses uses line with multiple spaces before comment | 407 | ported | `github_actions.rs` | `parse_uses_line_parses_uses_line_with_multiple_spaces_before_comment` | — |
| parses double quoted value | 435 | ported | `github_actions.rs` | `parse_uses_line_parses_double_quoted_value` | — |
| parses single quoted value | 457 | ported | `github_actions.rs` | `parse_uses_line_parses_single_quoted_value` | — |
| parses quoted value with comment | 479 | ported | `github_actions.rs` | `parse_uses_line_parses_quoted_value_with_comment` | — |
| parses docker action | 505 | ported | `github_actions.rs` | `parse_uses_line_parses_docker_action` | — |
| parses local action | 524 | ported | `github_actions.rs` | `parse_uses_line_parses_local_action` | — |
| handles ratchet:exclude comment | 541 | ported | `github_actions.rs` | `parse_uses_line_handles_ratchet_exclude_comment` | — |
| handles unrecognized comment | 567 | ported | `github_actions.rs` | `parse_uses_line_handles_unrecognized_comment` | — |
| returns null actionRef for invalid action | 591 | ported | `github_actions.rs` | `parse_uses_line_returns_none_action_ref_for_invalid_action` | — |

---

