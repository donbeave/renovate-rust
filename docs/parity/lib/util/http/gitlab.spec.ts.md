# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/gitlab.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/gitlab.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 14 | **Status:** pending

### `util/http/gitlab`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| paginates | 35 | pending | — | — | — |
| paginates with GITLAB_IGNORE_REPO_URL set | 63 | pending | — | — | — |
| supports different datasources | 85 | pending | — | — | — |
| attempts to paginate | 100 | pending | — | — | — |
| posts | 110 | pending | — | — | — |
| sets baseUrl | 117 | pending | — | — | — |

### `util/http/gitlab › fails with`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| 403 | 122 | pending | — | — | — |
| 404 | 131 | pending | — | — | — |
| 500 | 140 | pending | — | — | — |
| EAI_AGAIN | 147 | pending | — | — | — |
| ParseError | 157 | pending | — | — | — |

### `util/http/gitlab › handles 409 errors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| retries the request on resource lock | 178 | pending | — | — | — |
| does not retry more than twice on resource lock | 186 | pending | — | — | — |
| does not retry for other reasons | 196 | pending | — | — | — |

---

