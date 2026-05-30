# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/local/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/local/index.spec.ts
**Total tests:** 28 | **Ported:** 28 | **Actionable:** 0 | **Status:** done

### `initPlatform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns input | 5 | ported | `util.rs` | `test_local_init_platform_default` | — |
| preserves an explicit dryRun=extract override | 16 | ported | `util.rs` | `test_local_init_platform_extract` | — |
| falls back to lookup when dryRun=full is requested | 29 | ported | `util.rs` | `test_local_init_platform_full_falls_back` | — |

### `getRepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty array | 44 | ported | `util.rs` | `test_local_get_repos` | — |

### `initRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns object | 50 | ported | `util.rs` | `test_local_init_repo` | — |

### `dummy functions`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| findIssue | 62 | ported | `util.rs` | `test_local_find_issue_returns_null` | — |
| getIssueList | 66 | ported | `util.rs` | `test_local_get_issue_list_returns_empty` | — |
| getRawFile | 70 | ported | `util.rs` | `test_local_get_raw_file_returns_null` | — |
| getJsonFile | 74 | ported | `util.rs` | `test_local_get_json_file_returns_null` | — |
| getPrList | 78 | ported | `util.rs` | `test_local_get_pr_list_returns_empty` | — |
| ensureIssueClosing | 82 | ported | `util.rs` | `test_local_ensure_issue_closing_returns_void` | — |
| ensureIssue | 86 | ported | `util.rs` | `test_local_ensure_issue_returns_null` | — |
| massageMarkdown | 90 | ported | `util.rs` | `test_local_massage_markdown_passthrough` | — |
| maxBodyLength | 94 | ported | `util.rs` | `test_local_max_body_length` | — |
| updatePr | 98 | ported | `util.rs` | `test_local_update_pr_returns_void` | — |
| mergePr | 102 | ported | `util.rs` | `test_local_merge_pr_returns_false` | — |
| addReviewers | 106 | ported | `util.rs` | `test_local_add_reviewers_returns_void` | — |
| addAssignees | 110 | ported | `util.rs` | `test_local_add_assignees_returns_void` | — |
| createPr | 114 | ported | `util.rs` | `test_local_create_pr_returns_null` | — |
| deleteLabel | 118 | ported | `util.rs` | `test_local_delete_label_returns_void` | — |
| setBranchStatus | 122 | ported | `util.rs` | `test_local_set_branch_status_returns_void` | — |
| getBranchStatus | 126 | ported | `util.rs` | `test_local_get_branch_status_returns_red` | — |
| getBranchStatusCheck | 130 | ported | `util.rs` | `test_local_get_branch_status_check_returns_null` | — |
| ensureCommentRemoval | 134 | ported | `util.rs` | `test_local_ensure_comment_removal_returns_void` | — |
| ensureComment | 138 | ported | `util.rs` | `test_local_ensure_comment_returns_false` | — |
| getPr | 142 | ported | `util.rs` | `test_local_get_pr_returns_null` | — |
| findPr | 146 | ported | `util.rs` | `test_local_find_pr_returns_null` | — |
| getBranchPr | 150 | ported | `util.rs` | `test_local_get_branch_pr_returns_null` | — |

---
