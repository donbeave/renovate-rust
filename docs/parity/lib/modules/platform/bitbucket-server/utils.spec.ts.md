# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/bitbucket-server/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/bitbucket-server/utils.spec.ts
**Total tests:** 17 | **Ported:** 3 | **Actionable:** 17 | **Status:** partial

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getInvalidReviewers | 94 | ported | `util.rs` | `test_get_invalid_reviewers` | — |

### `getRepoGitUrl › endpoint with path`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works gitUrl:undefined generate endpoint | 127 | pending | — | — | — |
| works gitUrl:undefined use endpoint with injected auth | 146 | pending | — | — | — |
| works gitUrl:undefined use ssh | 165 | pending | — | — | — |
| works gitUrl:default | 179 | pending | — | — | — |
| gitUrl:default invalid http url throws CONFIG_GIT_URL_UNAVAILABLE | 196 | pending | — | — | — |
| gitUrl:default no http url returns generated url | 210 | pending | — | — | — |
| gitUrl:ssh no ssh url throws CONFIG_GIT_URL_UNAVAILABLE | 229 | pending | — | — | — |
| works gitUrl:ssh | 243 | pending | — | — | — |
| works gitUrl:endpoint | 255 | pending | — | — | — |
| works gitUrl:endpoint no basic auth | 272 | pending | — | — | — |

### `getRepoGitUrl › endpoint with no path`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works gitUrl:endpoint | 294 | pending | — | — | — |
| gitUrl:default no http url returns generated url | 306 | pending | — | — | — |
| actually respects the gitUrl Setting | 320 | pending | — | — | — |

### `getExtraCloneOpts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws on invalid endpoint URL | 333 | pending | — | — | — |
| should not configure bearer token | 334 | ported | `util.rs` | `test_get_extra_clone_opts_no_token` | — |
| should configure bearer token | 339 | ported | `util.rs` | `test_get_extra_clone_opts_with_token` | — |

---
