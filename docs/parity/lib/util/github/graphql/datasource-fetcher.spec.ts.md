# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/github/graphql/datasource-fetcher.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/github/graphql/datasource-fetcher.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/github/graphql/datasource-fetcher › query`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| can perform query and receive result | 119 | not-applicable | — | — | tests GitHub GraphQL datasource fetcher via httpMock; HTTP API calls out of scope |
| performs query when persistence flag is set and cache is expired | 139 | not-applicable | — | — | tests GitHub GraphQL datasource fetcher via httpMock; HTTP API calls out of scope |
| throws on unknown errors | 160 | not-applicable | — | — | tests GitHub GraphQL datasource fetcher via httpMock; HTTP API calls out of scope |
| throws single GraphQL error wrapped into Error | 171 | not-applicable | — | — | tests GitHub GraphQL datasource fetcher via httpMock; HTTP API calls out of scope |
| throws multiple GraphQL errors wrapped into AggregatedError | 185 | not-applicable | — | — | tests GitHub GraphQL datasource fetcher via httpMock; HTTP API calls out of scope |
| throws when neither of data or errors were provided | 202 | not-applicable | — | — | tests GitHub GraphQL datasource fetcher via httpMock; HTTP API calls out of scope |
| throws when repository field is absent | 210 | not-applicable | — | — | tests GitHub GraphQL datasource fetcher via httpMock; HTTP API calls out of scope |
| throws when payload field is absent | 223 | not-applicable | — | — | tests GitHub GraphQL datasource fetcher via httpMock; HTTP API calls out of scope |
| receives, transforms, and return data | 236 | not-applicable | — | — | tests GitHub GraphQL datasource fetcher via httpMock; HTTP API calls out of scope |
| handles paginated data | 263 | not-applicable | — | — | tests GitHub GraphQL datasource fetcher via httpMock; HTTP API calls out of scope |

### `util/github/graphql/datasource-fetcher › query › Page shrinking`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| shrinks page from 100 to 50 | 333 | not-applicable | — | — | tests GitHub GraphQL datasource fetcher via httpMock; HTTP API calls out of scope |
| shrinks page from 50 to 25 | 360 | not-applicable | — | — | tests GitHub GraphQL datasource fetcher via httpMock; HTTP API calls out of scope |
| re-throws if shrinking did not help | 390 | not-applicable | — | — | tests GitHub GraphQL datasource fetcher via httpMock; HTTP API calls out of scope |

### `util/github/graphql/datasource-fetcher › query › Cacheable flag`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| private=$isPrivate => isPersistent=$isPersistent | 416 | not-applicable | — | — | tests GitHub GraphQL datasource fetcher via httpMock; HTTP API calls out of scope |

### `util/github/graphql/datasource-fetcher › query › maxItems limit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| stops pagination after maxItems | 450 | not-applicable | — | — | tests GitHub GraphQL datasource fetcher via httpMock; HTTP API calls out of scope |

---

