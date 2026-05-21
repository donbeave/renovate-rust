# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/terragrunt/modules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terragrunt/modules.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `githubRefMatchRegex`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should split project and tag from source | 5 | ported | `terragrunt.rs` | `github_ref_regex_splits_project_and_tag` | — |
| should parse alpha-numeric characters as well as dots, underscores, and dashes in repo names | 15 | ported | `terragrunt.rs` | `github_ref_regex_parses_complex_repo_names` | — |

### `gitTagsRefMatchRegex`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should split host, path and tag from source | 27 | ported | `terragrunt.rs` | `git_ref_regex_splits_host_path_and_tag` | — |
| should parse alpha-numeric characters as well as dots, underscores, and dashes in repo names | 55 | ported | `terragrunt.rs` | `git_ref_regex_parses_complex_repo_path` | — |

| sets skipReason for invalid git tags URL | 89 | ported | `terragrunt.rs` | `sets_skip_reason_for_invalid_git_tags_url` | — |
---

