# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/bitbucket-server/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/bitbucket-server/utils.spec.ts
**Total tests:** 17 | **Ported:** 3 | **Actionable:** 3 | **Status:** partial

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getInvalidReviewers | 94 | ported | `util.rs` | `test_get_invalid_reviewers` | — |

### `getRepoGitUrl › endpoint with path`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works gitUrl:undefined generate endpoint | 127 | not-applicable | — | — | Bitbucket Server platform utils |
| works gitUrl:undefined use endpoint with injected auth | 146 | not-applicable | — | — | Bitbucket Server platform utils |
| works gitUrl:undefined use ssh | 165 | not-applicable | — | — | Bitbucket Server platform utils |
| works gitUrl:default | 179 | not-applicable | — | — | Bitbucket Server platform utils |
| gitUrl:default invalid http url throws CONFIG_GIT_URL_UNAVAILABLE | 196 | not-applicable | — | — | Bitbucket Server platform utils |
| gitUrl:default no http url returns generated url | 210 | not-applicable | — | — | Bitbucket Server platform utils |
| gitUrl:ssh no ssh url throws CONFIG_GIT_URL_UNAVAILABLE | 229 | not-applicable | — | — | Bitbucket Server platform utils |
| works gitUrl:ssh | 243 | not-applicable | — | — | Bitbucket Server platform utils |
| works gitUrl:endpoint | 255 | not-applicable | — | — | Bitbucket Server platform utils |
| works gitUrl:endpoint no basic auth | 272 | not-applicable | — | — | Bitbucket Server platform utils |

### `getRepoGitUrl › endpoint with no path`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works gitUrl:endpoint | 294 | not-applicable | — | — | Bitbucket Server platform utils |
| gitUrl:default no http url returns generated url | 306 | not-applicable | — | — | Bitbucket Server platform utils |
| actually respects the gitUrl Setting | 320 | not-applicable | — | — | Bitbucket Server platform utils |

### `getExtraCloneOpts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws on invalid endpoint URL | 333 | not-applicable | — | — | Bitbucket Server platform utils |
| should not configure bearer token | 334 | ported | `util.rs` | `test_get_extra_clone_opts_no_token` | — |
| should configure bearer token | 339 | ported | `util.rs` | `test_get_extra_clone_opts_with_token` | — |

---
