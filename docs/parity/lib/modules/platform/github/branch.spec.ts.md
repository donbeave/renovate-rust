# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/github/branch.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/github/branch.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true if the branch exists | 5 | not-applicable | — | — | tests GitHub HTTP API via httpMock; not portable to Rust HTTP client |
| should return false if the branch does not exist | 16 | not-applicable | — | — | tests GitHub HTTP API via httpMock; not portable to Rust HTTP client |
| should throw an error for nested branches | 27 | not-applicable | — | — | tests GitHub HTTP API via httpMock; not portable to Rust HTTP client |
| should throw an error if the request fails for any other reason | 44 | not-applicable | — | — | tests GitHub HTTP API via httpMock; not portable to Rust HTTP client |

---

