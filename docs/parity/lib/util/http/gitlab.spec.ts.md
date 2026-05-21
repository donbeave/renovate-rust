# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/gitlab.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/gitlab.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/http/gitlab`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| paginates | 35 | not-applicable | — | — | tests GitlabHttp (got-based) with httpMock; Rust uses reqwest |
| paginates with GITLAB_IGNORE_REPO_URL set | 63 | not-applicable | — | — | tests GitlabHttp (got-based) with httpMock; Rust uses reqwest |
| supports different datasources | 85 | not-applicable | — | — | tests GitlabHttp (got-based) with httpMock; Rust uses reqwest |
| attempts to paginate | 100 | not-applicable | — | — | tests GitlabHttp (got-based) with httpMock; Rust uses reqwest |
| posts | 110 | not-applicable | — | — | tests GitlabHttp (got-based) with httpMock; Rust uses reqwest |
| sets baseUrl | 117 | not-applicable | — | — | tests GitlabHttp (got-based) with httpMock; Rust uses reqwest |

### `util/http/gitlab › fails with`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| 403 | 122 | not-applicable | — | — | tests GitlabHttp (got-based) with httpMock; Rust uses reqwest |
| 404 | 131 | not-applicable | — | — | tests GitlabHttp (got-based) with httpMock; Rust uses reqwest |
| 500 | 140 | not-applicable | — | — | tests GitlabHttp (got-based) with httpMock; Rust uses reqwest |
| EAI_AGAIN | 147 | not-applicable | — | — | tests GitlabHttp (got-based) with httpMock; Rust uses reqwest |
| ParseError | 157 | not-applicable | — | — | tests GitlabHttp (got-based) with httpMock; Rust uses reqwest |

### `util/http/gitlab › handles 409 errors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| retries the request on resource lock | 178 | not-applicable | — | — | tests GitlabHttp (got-based) with httpMock; Rust uses reqwest |
| does not retry more than twice on resource lock | 186 | not-applicable | — | — | tests GitlabHttp (got-based) with httpMock; Rust uses reqwest |
| does not retry for other reasons | 196 | not-applicable | — | — | tests GitlabHttp (got-based) with httpMock; Rust uses reqwest |

---

