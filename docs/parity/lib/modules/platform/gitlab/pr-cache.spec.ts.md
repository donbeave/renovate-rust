# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gitlab/pr-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gitlab/pr-cache.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fetches cache initially | 81 | not-applicable | — | — | Mock framework internals — tests GitLab PR cache via nock HTTP mocks; Rust tests platform APIs at different layer |
| fetches cache with ignorePrAuthor=true | 110 | not-applicable | — | — | Mock framework internals — tests GitLab PR cache via nock HTTP mocks; Rust tests platform APIs at different layer |
| resets cache for not matching authors | 128 | not-applicable | — | — | Mock framework internals — tests GitLab PR cache via nock HTTP mocks; Rust tests platform APIs at different layer |
| resets cache for older format with milliseconds | 169 | not-applicable | — | — | Mock framework internals — tests GitLab PR cache via nock HTTP mocks; Rust tests platform APIs at different layer |
| syncs cache with updated_after parameter | 210 | not-applicable | — | — | Mock framework internals — tests GitLab PR cache via nock HTTP mocks; Rust tests platform APIs at different layer |
| handles empty response | 251 | not-applicable | — | — | Mock framework internals — tests GitLab PR cache via nock HTTP mocks; Rust tests platform APIs at different layer |
| returns items in reverse order (most recent first) | 267 | not-applicable | — | — | Mock framework internals — tests GitLab PR cache via nock HTTP mocks; Rust tests platform APIs at different layer |
| normalizes timestamps by removing milliseconds | 280 | not-applicable | — | — | Mock framework internals — tests GitLab PR cache via nock HTTP mocks; Rust tests platform APIs at different layer |

---

