# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/gitlab.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/gitlab.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 14 | **Status:** done

### `util/http/gitlab`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| paginates | 35 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| paginates with GITLAB_IGNORE_REPO_URL set | 63 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports different datasources | 85 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| attempts to paginate | 100 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| posts | 110 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| sets baseUrl | 117 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `util/http/gitlab › fails with`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| 403 | 122 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| 404 | 131 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| 500 | 140 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| EAI_AGAIN | 147 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| ParseError | 157 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `util/http/gitlab › handles 409 errors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| retries the request on resource lock | 178 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| does not retry more than twice on resource lock | 186 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| does not retry for other reasons | 196 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

---

