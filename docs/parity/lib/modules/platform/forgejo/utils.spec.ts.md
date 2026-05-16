# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/forgejo/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/forgejo/utils.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| trimTrailingApiPath | 26 | ported | `gitea_forgejo_utils.rs` | `trim_trailing_api_path_strips_api_v1` | — |

### `getRepoUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should abort when endpoint is not valid | 45 | ported | `gitea_forgejo_utils.rs` | `validate_endpoint_url_invalid_throws` | — |
| getMergeMethod("$value") == "$expected" | 53 | ported | `gitea_forgejo_utils.rs` | `get_merge_method_all_cases` | — |

### `usableRepo`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true when repo is usable | 66 | ported | `gitea_forgejo_utils.rs` | `usable_repo_returns_true_for_usable_repo` | — |
| should return false when repo lacks permissions | 70 | ported | `gitea_forgejo_utils.rs` | `usable_repo_returns_false_without_permissions` | — |
| should return false when repo has disabled pull requests | 85 | ported | `gitea_forgejo_utils.rs` | `usable_repo_returns_false_without_pull_requests` | — |

---

