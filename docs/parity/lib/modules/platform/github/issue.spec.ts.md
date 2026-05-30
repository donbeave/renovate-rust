# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/github/issue.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/github/issue.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** done

### `GithubIssueCache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty cache | 16 | ported | `github_api_cache.rs` | `issue_cache_returns_none_for_empty` | — |
| stores issues to the cache | 20 | ported | `github_api_cache.rs` | `issue_cache_stores_issues` | — |
| returns issues from the cache in the correct order | 64 | ported | `github_api_cache.rs` | `issue_cache_returns_sorted_by_last_modified_desc` | — |
| updates particular issue in the cache | 120 | ported | `github_api_cache.rs` | `issue_cache_updates_issue` | — |
| removes particular issue from the cache | 162 | ported | `github_api_cache.rs` | `issue_cache_deletes_issue` | — |
| reconciles cache | 188 | ported | `github_api_cache.rs` | `issue_cache_reconciles` | — |
| resets cache during failed reconciliation | 246 | ported | `github_api_cache.rs` | `issue_cache_resets_on_failed_reconcile` | — |

---
