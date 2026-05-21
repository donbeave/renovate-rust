# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/bitbucket-server/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/bitbucket-server/utils.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getInvalidReviewers | 94 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `getRepoGitUrl › endpoint with path`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works gitUrl:undefined generate endpoint | 127 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| works gitUrl:undefined use endpoint with injected auth | 146 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| works gitUrl:undefined use ssh | 165 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| works gitUrl:default | 179 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| gitUrl:default invalid http url throws CONFIG_GIT_URL_UNAVAILABLE | 196 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| gitUrl:default no http url returns generated url | 210 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| gitUrl:ssh no ssh url throws CONFIG_GIT_URL_UNAVAILABLE | 229 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| works gitUrl:ssh | 243 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| works gitUrl:endpoint | 255 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| works gitUrl:endpoint no basic auth | 272 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `getRepoGitUrl › endpoint with no path`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works gitUrl:endpoint | 294 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| gitUrl:default no http url returns generated url | 306 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| actually respects the gitUrl Setting | 320 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

### `getExtraCloneOpts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should not configure bearer token | 334 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |
| should configure bearer token | 339 | not-applicable | — | — | out of scope: tests platform HTTP API interactions via TypeScript httpMock; Rust extraction layer does not call platform APIs |

---

