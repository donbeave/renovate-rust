# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/scm-manager/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/scm-manager/utils.spec.ts
**Total tests:** 12 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

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
| should throw error for option $gitUrl, because protocol links are missing | 117 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should throw error because of missing SSH link | 132 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should throw error because protocol links are not an array | 145 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should use the provided ssh link | 158 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should throw error because of missing HTTP link for option $gitUrl | 171 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should throw error because of malformed HTTP link with option $gitUrl | 192 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should use empty string, because username was not provided with option $gitUrl | 213 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should use empty string, because token was not provided. With option $gitUrl | 235 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should provide the HTTP link with username, for option $gitUrl | 258 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

---

