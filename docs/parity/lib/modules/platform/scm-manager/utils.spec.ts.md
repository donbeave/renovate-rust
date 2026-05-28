# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/scm-manager/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/scm-manager/utils.spec.ts
**Total tests:** 12 | **Ported:** 9 | **Actionable:** 12 | **Status:** partial

### `getMergeMethod`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| map merge strategy $strategy on PR merge method $method | 16 | ported | `scm_manager.rs` | `get_scm_merge_method_all_cases` | — |

### `smartLinks`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adjust $body to smart link $result | 39 | ported | `scm_manager.rs` | `smart_links_replaces_pull_links` | — |

### `matchPrState`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| match scm pr state $pr.state to renovate pr state $state | 61 | ported | `scm_manager.rs` | `match_pr_state_all_cases` | — |

### `getRepoUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw error for option $gitUrl, because protocol links are missing | 117 | ported | `scm_manager.rs` | `get_repo_url_errors_no_http_link` | — |
| should throw error because of missing SSH link | 132 | ported | `scm_manager.rs` | `get_repo_url_errors_missing_ssh` | — |
| should throw error because protocol links are not an array | 145 | not-applicable | — | — | Rust uses typed Vec<ProtocolLink> — non-array cannot occur |
| should use the provided ssh link | 158 | ported | `scm_manager.rs` | `get_repo_url_uses_ssh_link` | — |
| should throw error because of missing HTTP link for option $gitUrl | 171 | ported | `scm_manager.rs` | `get_repo_url_errors_no_http_link` | — |
| should throw error because of malformed HTTP link with option $gitUrl | 192 | ported | `scm_manager.rs` | `get_repo_url_errors_malformed_http_link` | — |
| should use empty string, because username was not provided with option $gitUrl | 213 | ported | `scm_manager.rs` | `get_repo_url_no_username_gives_plain_url` | — |
| should use empty string, because token was not provided. With option $gitUrl | 235 | ported | `scm_manager.rs` | `get_repo_url_no_username_gives_plain_url` | — |
| should provide the HTTP link with username, for option $gitUrl | 258 | ported | `scm_manager.rs` | `get_repo_url_with_username_and_token` | — |

---

