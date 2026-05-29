# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/github/graphql/datasource-fetcher.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/github/graphql/datasource-fetcher.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 15 | **Status:** pending

### `util/github/graphql/datasource-fetcher › query`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| can perform query and receive result | 119 | pending | — | — | —|
| performs query when persistence flag is set and cache is expired | 139 | pending | — | — | —|
| throws on unknown errors | 160 | pending | — | — | —|
| throws single GraphQL error wrapped into Error | 171 | pending | — | — | —|
| throws multiple GraphQL errors wrapped into AggregatedError | 185 | pending | — | — | —|
| throws when neither of data or errors were provided | 202 | pending | — | — | —|
| throws when repository field is absent | 210 | pending | — | — | —|
| throws when payload field is absent | 223 | pending | — | — | —|
| receives, transforms, and return data | 236 | pending | — | — | —|
| handles paginated data | 263 | pending | — | — | —|

### `util/github/graphql/datasource-fetcher › query › Page shrinking`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| shrinks page from 100 to 50 | 333 | pending | — | — | —|
| shrinks page from 50 to 25 | 360 | pending | — | — | —|
| re-throws if shrinking did not help | 390 | pending | — | — | —|

### `util/github/graphql/datasource-fetcher › query › Cacheable flag`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| private=$isPrivate => isPersistent=$isPersistent | 416 | pending | — | — | —|

### `util/github/graphql/datasource-fetcher › query › maxItems limit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| stops pagination after maxItems | 450 | pending | — | — | —|

---

