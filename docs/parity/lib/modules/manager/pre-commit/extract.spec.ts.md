# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pre-commit/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pre-commit/extract.spec.ts
**Total tests:** 12 | **Ported:** 12 | **Actionable:** 12 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid yaml file content | 52 | ported | `pre_commit.rs` | `invalid_yaml_returns_empty` | — |
| returns null for empty yaml file content | 57 | ported | `pre_commit.rs` | `empty_content_returns_no_deps` | — |
| returns null for no file content | 62 | ported | `pre_commit.rs` | `null_content_returns_empty` | — |
| returns null for no repos | 68 | ported | `pre_commit.rs` | `no_repos_section_returns_no_deps` | — |
| returns null for empty repos | 73 | ported | `pre_commit.rs` | `empty_repos_list_returns_empty` | — |
| returns null for invalid repo | 78 | ported | `pre_commit.rs` | `repo_entry_without_repo_key_returns_empty` | — |
| extracts from values.yaml correctly with same structure as "pre-commit sample-config" | 83 | ported | `pre_commit.rs` | `git_suffix_stripped` | — |
| extracts from complex config file correctly | 105 | ported | `pre_commit.rs` | `extracts_github_hooks` (+ extracts_gitlab_hooks, skips_local_hooks, skips_meta_hooks, total_dep_count) | — |
| can handle private git repos | 161 | ported | `pre_commit.rs` | `private_gitlab_host_uses_gitlab_tags_and_registry_url` | — |
| can handle invalid private git repos | 183 | ported | `pre_commit.rs` | `unknown_registry_gets_skip_reason` | — |
| can handle unknown private git repos | 200 | ported | `pre_commit.rs` | `private_git_host_without_provider_is_unknown_registry` | — |
| can handle pinned repo versions | 220 | ported | `pre_commit.rs` | `frozen_digest_rev_extracts_version_and_digest` | — |

---

