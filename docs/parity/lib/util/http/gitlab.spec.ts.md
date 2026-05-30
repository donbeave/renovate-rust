# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/gitlab.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/gitlab.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `util/http/gitlab`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| paginates | 35 | not-applicable | — | — | GitLab-specific HTTP wrapper (`got` pagination, error handling); Rust uses generic `reqwest` client |
| paginates with GITLAB_IGNORE_REPO_URL set | 63 | not-applicable | — | — | GitLab-specific HTTP wrapper (`got` pagination, error handling); Rust uses generic `reqwest` client |
| supports different datasources | 85 | not-applicable | — | — | GitLab-specific HTTP wrapper (`got` pagination, error handling); Rust uses generic `reqwest` client |
| attempts to paginate | 100 | not-applicable | — | — | GitLab-specific HTTP wrapper (`got` pagination, error handling); Rust uses generic `reqwest` client |
| posts | 110 | not-applicable | — | — | GitLab-specific HTTP wrapper (`got` pagination, error handling); Rust uses generic `reqwest` client |
| sets baseUrl | 117 | not-applicable | — | — | GitLab-specific HTTP wrapper (`got` pagination, error handling); Rust uses generic `reqwest` client |

### `util/http/gitlab › fails with`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| 403 | 122 | not-applicable | — | — | GitLab-specific HTTP wrapper (`got` pagination, error handling); Rust uses generic `reqwest` client |
| 404 | 131 | not-applicable | — | — | GitLab-specific HTTP wrapper (`got` pagination, error handling); Rust uses generic `reqwest` client |
| 500 | 140 | not-applicable | — | — | GitLab-specific HTTP wrapper (`got` pagination, error handling); Rust uses generic `reqwest` client |
| EAI_AGAIN | 147 | not-applicable | — | — | GitLab-specific HTTP wrapper (`got` pagination, error handling); Rust uses generic `reqwest` client |
| ParseError | 157 | not-applicable | — | — | GitLab-specific HTTP wrapper (`got` pagination, error handling); Rust uses generic `reqwest` client |

### `util/http/gitlab › handles 409 errors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| retries the request on resource lock | 178 | not-applicable | — | — | GitLab-specific HTTP wrapper (`got` pagination, error handling); Rust uses generic `reqwest` client |
| does not retry more than twice on resource lock | 186 | not-applicable | — | — | GitLab-specific HTTP wrapper (`got` pagination, error handling); Rust uses generic `reqwest` client |
| does not retry for other reasons | 196 | not-applicable | — | — | GitLab-specific HTTP wrapper (`got` pagination, error handling); Rust uses generic `reqwest` client |

---

