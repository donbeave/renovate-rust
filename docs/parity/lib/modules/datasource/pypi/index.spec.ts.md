# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/pypi/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/pypi/index.spec.ts
**Total tests:** 39 | **Ported:** 2 | **Actionable:** 39 | **Status:** done

### `modules/datasource/pypi/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 90 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for 404 | 100 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| processes real data | 111 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports custom datasource url | 121 | ported | `pypi.rs` | `fetch_versions_returns_sorted` | Rust verifies lookup through the supplied API base URL. |
| sets private if authorization privided | 142 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |
| supports multiple custom datasource urls | 159 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports Google Auth | 194 | not-applicable | — | — | Requires httpMock + Google auth mock infrastructure |
| supports Google Auth not being configured | 222 | not-applicable | — | — | Requires httpMock + Google auth mock infrastructure |
| returns non-github home_page | 246 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| find url from project_urls | 267 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| excludes gh sponsors url from project_urls | 291 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| does not mistake sponsors in project name as sponsors url | 310 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| normalizes the package name according to PEP 503 | 329 | ported | `pypi.rs` | `fetch_versions_normalizes_name` | — |
| normalizes the package name according to PEP 503 when falling back to simple endpoint | 349 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| normalizes the package name according to PEP 503 querying a simple endpoint | 368 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| respects constraints | 384 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| process data from simple endpoint | 413 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| process data from +simple endpoint | 431 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| sets private simple if authorization provided | 449 | not-applicable | — | — | Requires httpMock + hostRules mock infrastructure |
| process data from simple endpoint with hyphens | 470 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| process data from simple endpoint with zip archives | 490 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| process data from simple endpoint with hyphens replaced with underscores | 509 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| process data from simple endpoint with mixed-case characters | 527 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| process data from simple endpoint with mixed-case characters when using lower case dependency name | 547 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| process data from simple endpoint with periods | 567 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| process data from simple endpoint with periods when using normalized name | 587 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| process data from simple endpoint for snowflake-legacy | 607 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| ignores invalid distribution file name formats | 633 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| process data from simple endpoint with non normalized name | 649 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| process data from simple endpoint with extra whitespaces in html | 674 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for empty response | 694 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for 404 response from simple endpoint | 712 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for response with no versions | 730 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| fall back from json and process data from simple endpoint | 748 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| parses data-requires-python and respects constraints from simple endpoint | 771 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `modules/datasource/pypi/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports Google Auth with simple endpoint | 791 | not-applicable | — | — | Requires httpMock + Google auth mock infrastructure |
| sanitizes GAR userinfo when Google auth is used | 822 | not-applicable | — | — | Requires httpMock + Google auth mock infrastructure |
| ignores an invalid URL when checking for auth headers | 853 | not-applicable | — | — | Requires httpMock + Google auth mock infrastructure |
| uses https://pypi.org/pypi/ instead of https://pypi.org/simple/ | 865 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

---
