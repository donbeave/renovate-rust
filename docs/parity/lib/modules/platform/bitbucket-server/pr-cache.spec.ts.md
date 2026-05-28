# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/bitbucket-server/pr-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/bitbucket-server/pr-cache.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 4 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| fetches cache - author defined | 67 | not-applicable | — | — | All tests use httpMock to simulate Bitbucket Server REST API responses; tests exercise platform HTTP cache infrastructure, not pure logic |
| fetches cache - author undefined | 111 | not-applicable | — | — | All tests use httpMock to simulate Bitbucket Server REST API responses; tests exercise platform HTTP cache infrastructure, not pure logic |
| resets cache for not matching authors | 154 | not-applicable | — | — | All tests use httpMock to simulate Bitbucket Server REST API responses; tests exercise platform HTTP cache infrastructure, not pure logic |
| syncs cache | 202 | not-applicable | — | — | All tests use httpMock to simulate Bitbucket Server REST API responses; tests exercise platform HTTP cache infrastructure, not pure logic |

---

