# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/azure/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/azure/util.spec.ts
**Total tests:** 29 | **Ported:** 27 | **Actionable:** 0 | **Status:** done

### `getGitStatusContextCombinedName`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return undefined if null context passed | 16 | ported | `azure_utils.rs` | `git_status_context_combined_name_none_genre_empty_name` | — |
| should combine valid genre and name with slash | 21 | ported | `azure_utils.rs` | `git_status_context_combined_name_genre_and_name` | — |
| should combine valid empty genre and name without a slash | 29 | ported | `azure_utils.rs` | `git_status_context_combined_name_undefined_genre` | — |

### `getGitStatusContextFromCombinedName`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return undefined if null context passed | 39 | ported | `azure_utils.rs` | `git_status_context_from_combined_name_empty_returns_none` | — |
| should parse valid genre and name with slash | 44 | ported | `azure_utils.rs` | `git_status_context_from_combined_name_slash` | — |
| should parse valid genre and name with multiple slashes | 54 | ported | `azure_utils.rs` | `git_status_context_from_combined_name_multiple_slashes` | — |
| should parse valid empty genre and name without a slash | 64 | ported | `azure_utils.rs` | `git_status_context_from_combined_name_no_slash` | — |

### `getBranchNameWithoutRefsheadsPrefix`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be renamed | 74 | ported | `azure_utils.rs` | `branch_name_strips_refs_heads_prefix` | — |
| should log error and return undefined | 79 | ported | `azure_utils.rs` | `branch_name_empty_returns_none` | — |
| should return the input | 84 | ported | `azure_utils.rs` | `branch_name_without_prefix_returns_as_is` | — |

### `getRenovatePRFormat`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be formated (closed) | 91 | ported | `azure_utils.rs` | `azure_pr_state_closed` | — |
| should be formated (closed v2) | 96 | ported | `azure_utils.rs` | `azure_pr_state_merged` | — |
| should be formated (not closed) | 101 | ported | `azure_utils.rs` | `azure_pr_state_open` | — |

### `streamToString`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| converts Readable stream to string | 108 | not-applicable | — | — | TypeScript module system test; uses Node.js Readable.from() stream; Rust uses async readers |
| handles error | 113 | not-applicable | — | — | TypeScript module system test; uses Node.js Readable stream destroy/error events |

### `getStorageExtraCloneOpts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should configure basic auth | 122 | ported | `azure_utils.rs` | `storage_extra_clone_opts_basic_auth` | — |
| should configure personal access token | 130 | ported | `azure_utils.rs` | `storage_extra_clone_opts_pat` | — |
| should configure bearer token | 137 | ported | `azure_utils.rs` | `storage_extra_clone_opts_bearer` | — |

### `max4000Chars`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be the same | 144 | ported | `azure_utils.rs` | `max4000_chars_short_string_unchanged` | — |
| should be truncated | 149 | ported | `azure_utils.rs` | `max4000_chars_long_string_truncated` | — |

### `getProjectAndRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the object with same strings | 120 | ported | `azure_utils.rs` | `get_project_and_repo_single_name` | — |
| should return the object with project and repo | 125 | ported | `azure_utils.rs` | `get_project_and_repo_project_slash_repo` | — |
| should return an error | 130 | ported | `azure_utils.rs` | `get_project_and_repo_too_many_segments` | — |

### `getRepoByName`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when repos array is empty | 151 | ported | `azure_utils.rs` | `get_repo_by_name_empty_list_returns_none` | — |
| returns null when repo is not found | 157 | ported | `azure_utils.rs` | `get_repo_by_name_not_found_returns_none` | — |
| finds repo | 163 | ported | `azure_utils.rs` | `get_repo_by_name_finds_first_match` | — |
| supports shorthand names | 181 | ported | `azure_utils.rs` | `get_repo_by_name_shorthand` | — |
| is case-independent | 189 | ported | `azure_utils.rs` | `get_repo_by_name_case_insensitive` | — |
| throws when repo name is invalid | 200 | ported | `azure_utils.rs` | `get_repo_by_name_three_part_name_returns_none` | — |

---

