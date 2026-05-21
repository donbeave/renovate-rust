# Renovate Test Detail

[Back to test map](../../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/changelog/github/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/changelog/github/index.spec.ts
**Total tests:** 17 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/update/pr/changelog/github/index › getChangeLogJSON`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if @types | 55 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| returns null if no currentVersion | 64 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| returns null if currentVersion equals newVersion | 73 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| skips invalid repos | 83 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| works without Github | 92 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| uses GitHub tags | 118 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| filters unnecessary warns | 144 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| supports node engines | 171 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| handles no sourceUrl | 198 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| handles invalid sourceUrl | 207 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| handles missing Github token | 216 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| handles suppressed Github warnings | 226 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| handles no releases | 236 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| handles not enough releases | 245 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| supports github enterprise and github.com changelog | 254 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| supports github enterprise and github enterprise changelog | 285 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |
| works with same version releases but different prefix | 318 | not-applicable | — | — | tests GitHub release notes fetching via GraphQL API; platform API calls out of scope |

---
