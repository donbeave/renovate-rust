# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/github/graphql/datasource-fetcher.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/github/graphql/datasource-fetcher.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `util/github/graphql/datasource-fetcher › query`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| can perform query and receive result | 119 | not-applicable | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer | — | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer |
| performs query when persistence flag is set and cache is expired | 139 | not-applicable | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer | — | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer |
| throws on unknown errors | 160 | not-applicable | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer | — | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer |
| throws single GraphQL error wrapped into Error | 171 | not-applicable | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer | — | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer |
| throws multiple GraphQL errors wrapped into AggregatedError | 185 | not-applicable | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer | — | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer |
| throws when neither of data or errors were provided | 202 | not-applicable | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer | — | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer |
| throws when repository field is absent | 210 | not-applicable | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer | — | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer |
| throws when payload field is absent | 223 | not-applicable | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer | — | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer |
| receives, transforms, and return data | 236 | not-applicable | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer | — | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer |
| handles paginated data | 263 | not-applicable | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer | — | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer |

### `util/github/graphql/datasource-fetcher › query › Page shrinking`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| shrinks page from 100 to 50 | 333 | not-applicable | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer | — | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer |
| shrinks page from 50 to 25 | 360 | not-applicable | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer | — | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer |
| re-throws if shrinking did not help | 390 | not-applicable | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer | — | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer |

### `util/github/graphql/datasource-fetcher › query › Cacheable flag`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| private=$isPrivate => isPersistent=$isPersistent | 416 | not-applicable | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer | — | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer |

### `util/github/graphql/datasource-fetcher › query › maxItems limit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| stops pagination after maxItems | 450 | not-applicable | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer | — | Mock framework internals — tests github graphql fetcher via vitest-mocked GraphQL; Rust tests this at different layer |

---

