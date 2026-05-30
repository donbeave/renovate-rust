# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/pypi/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/pypi/index.spec.ts
**Total tests:** 39 | **Ported:** 2 | **Actionable:** 37 | **Status:** partial

### `modules/datasource/pypi/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 90 | pending | — | — | —|
| returns null for 404 | 100 | pending | — | — | —|
| processes real data | 111 | pending | — | — | —|
| supports custom datasource url | 121 | ported | `pypi.rs` | `fetch_versions_returns_sorted` | Rust verifies lookup through the supplied API base URL. |
| sets private if authorization privided | 142 | pending | — | — | —|
| supports multiple custom datasource urls | 159 | pending | — | — | —|
| supports Google Auth | 194 | pending | — | — | —|
| supports Google Auth not being configured | 222 | pending | — | — | —|
| returns non-github home_page | 246 | pending | — | — | —|
| find url from project_urls | 267 | pending | — | — | —|
| excludes gh sponsors url from project_urls | 291 | pending | — | — | —|
| does not mistake sponsors in project name as sponsors url | 310 | pending | — | — | —|
| normalizes the package name according to PEP 503 | 329 | ported | `pypi.rs` | `fetch_versions_normalizes_name` | — |
| normalizes the package name according to PEP 503 when falling back to simple endpoint | 349 | pending | — | — | —|
| normalizes the package name according to PEP 503 querying a simple endpoint | 368 | pending | — | — | —|
| respects constraints | 384 | pending | — | — | —|
| process data from simple endpoint | 413 | pending | — | — | —|
| process data from +simple endpoint | 431 | pending | — | — | —|
| sets private simple if authorization provided | 449 | pending | — | — | —|
| process data from simple endpoint with hyphens | 470 | pending | — | — | —|
| process data from simple endpoint with zip archives | 490 | pending | — | — | —|
| process data from simple endpoint with hyphens replaced with underscores | 509 | pending | — | — | —|
| process data from simple endpoint with mixed-case characters | 527 | pending | — | — | —|
| process data from simple endpoint with mixed-case characters when using lower case dependency name | 547 | pending | — | — | —|
| process data from simple endpoint with periods | 567 | pending | — | — | —|
| process data from simple endpoint with periods when using normalized name | 587 | pending | — | — | —|
| process data from simple endpoint for snowflake-legacy | 607 | pending | — | — | —|
| ignores invalid distribution file name formats | 633 | pending | — | — | —|
| process data from simple endpoint with non normalized name | 649 | pending | — | — | —|
| process data from simple endpoint with extra whitespaces in html | 674 | pending | — | — | —|
| returns null for empty response | 694 | pending | — | — | —|
| returns null for 404 response from simple endpoint | 712 | pending | — | — | —|
| returns null for response with no versions | 730 | pending | — | — | —|
| fall back from json and process data from simple endpoint | 748 | pending | — | — | —|
| parses data-requires-python and respects constraints from simple endpoint | 771 | pending | — | — | —|

### `modules/datasource/pypi/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports Google Auth with simple endpoint | 791 | pending | — | — | —|
| sanitizes GAR userinfo when Google auth is used | 822 | pending | — | — | —|
| ignores an invalid URL when checking for auth headers | 853 | pending | — | — | —|
| uses https://pypi.org/pypi/ instead of https://pypi.org/simple/ | 865 | pending | — | — | —|

---
