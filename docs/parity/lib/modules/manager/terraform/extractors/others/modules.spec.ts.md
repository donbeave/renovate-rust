# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/terraform/extractors/others/modules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/terraform/extractors/others/modules.spec.ts
**Total tests:** 13 | **Ported:** 13 | **Actionable:** 13 | **Status:** ported

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return empty array if no module is found | 13 | ported | `terraform.rs` | `modules_extract_empty_content_returns_no_module_deps` | — |

### `githubRefMatchRegex`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should split project and tag from source | 19 | ported | `terraform.rs` | `github_ref_match_re_splits_project_and_tag` | — |
| should parse alpha-numeric characters as well as dots, underscores, and dashes in repo names | 43 | ported | `terraform.rs` | `github_ref_match_re_parses_alphanumeric_repo_names` | — |

### `gitTagsRefMatchRegex`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should split project and tag from source | 55 | ported | `terraform.rs` | `git_tags_ref_match_re_splits_project_and_tag` | — |
| should parse alpha-numeric characters as well as dots, underscores, and dashes in repo names | 108 | ported | `terraform.rs` | `git_tags_ref_match_re_parses_alphanumeric_repo_names` | — |

### `bitbucketRefMatchRegex`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should split workspace, project and tag from source | 156 | ported | `terraform.rs` | `bitbucket_ref_match_re_splits_workspace_project_and_tag` | — |
| should parse alpha-numeric characters as well as dots, underscores, and dashes in repo names | 224 | ported | `terraform.rs` | `bitbucket_ref_match_re_parses_alphanumeric_repo_names` | — |

### `azureDevOpsSshRefMatchRegex`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should split organization, project, repository and tag from source url | 238 | ported | `terraform.rs` | `azure_devops_ssh_ref_match_re_splits_fields` | — |
| should split organization, project, repository and tag from source url with git prefix | 253 | ported | `terraform.rs` | `azure_devops_ssh_ref_match_re_with_git_prefix` | — |
| should split organization, project, repository and tag from source url with subfolder | 268 | ported | `terraform.rs` | `azure_devops_ssh_ref_match_re_with_subfolder` | — |
| should split organization, project, repository and tag from source url with depth argument | 283 | ported | `terraform.rs` | `azure_devops_ssh_ref_match_re_with_depth` | — |
| should parse alpha-numeric characters as well as dots, underscores, and dashes in repo names | 309 | ported | `terraform.rs` | `azure_devops_ssh_ref_match_re_parses_alphanumeric_names` | — |

### `hostnameMatchRegex`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should extact hostname from source url | 326 | ported | `terraform.rs` | `hostname_match_re_extracts_hostname` | — |

---

