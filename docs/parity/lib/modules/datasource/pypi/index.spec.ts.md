# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/pypi/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/pypi/index.spec.ts
**Total tests:** 39 | **Ported:** 2 | **Actionable:** 37 | **Status:** partial

### `modules/datasource/pypi/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 90 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for 404 | 100 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| processes real data | 111 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| supports custom datasource url | 121 | ported | `pypi.rs` | `fetch_versions_returns_sorted` | Rust verifies lookup through the supplied API base URL. |
| sets private if authorization privided | 142 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| supports multiple custom datasource urls | 159 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| supports Google Auth | 194 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| supports Google Auth not being configured | 222 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns non-github home_page | 246 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| find url from project_urls | 267 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| excludes gh sponsors url from project_urls | 291 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| does not mistake sponsors in project name as sponsors url | 310 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| normalizes the package name according to PEP 503 | 329 | ported | `pypi.rs` | `fetch_versions_normalizes_name` | — |
| normalizes the package name according to PEP 503 when falling back to simple endpoint | 349 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| normalizes the package name according to PEP 503 querying a simple endpoint | 368 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| respects constraints | 384 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| process data from simple endpoint | 413 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| process data from +simple endpoint | 431 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| sets private simple if authorization provided | 449 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| process data from simple endpoint with hyphens | 470 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| process data from simple endpoint with zip archives | 490 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| process data from simple endpoint with hyphens replaced with underscores | 509 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| process data from simple endpoint with mixed-case characters | 527 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| process data from simple endpoint with mixed-case characters when using lower case dependency name | 547 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| process data from simple endpoint with periods | 567 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| process data from simple endpoint with periods when using normalized name | 587 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| process data from simple endpoint for snowflake-legacy | 607 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| ignores invalid distribution file name formats | 633 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| process data from simple endpoint with non normalized name | 649 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| process data from simple endpoint with extra whitespaces in html | 674 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for empty response | 694 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for 404 response from simple endpoint | 712 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for response with no versions | 730 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| fall back from json and process data from simple endpoint | 748 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| parses data-requires-python and respects constraints from simple endpoint | 771 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|

### `modules/datasource/pypi/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports Google Auth with simple endpoint | 791 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| sanitizes GAR userinfo when Google auth is used | 822 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| ignores an invalid URL when checking for auth headers | 853 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| uses https://pypi.org/pypi/ instead of https://pypi.org/simple/ | 865 | not-applicable | Mock framework internals — tests pypi datasource via nock HTTP mocks; Rust tests this at different layer | — | —|

---
