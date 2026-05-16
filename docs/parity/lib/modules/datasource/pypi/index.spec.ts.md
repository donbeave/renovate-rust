# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/pypi/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/pypi/index.spec.ts
**Total tests:** 39 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `modules/datasource/pypi/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 90 | not-applicable | — | — | Renovate's PyPI null-on-empty-body contract is not implemented in Rust; Rust returns a parse error for malformed JSON API responses. |
| returns null for 404 | 100 | not-applicable | — | — | Renovate's PyPI 404-to-simple-endpoint fallback and null result contract are not implemented in Rust; Rust returns an HTTP error from JSON API lookups. |
| processes real data | 111 | not-applicable | — | — | Renovate's PyPI full release-list, timestamps, homepage, sourceUrl, and changelog mapping are not implemented in Rust; Rust returns version cache entries for update summaries. |
| supports custom datasource url | 121 | ported | `pypi.rs` | `fetch_versions_returns_sorted` | Rust verifies lookup through the supplied API base URL. |
| sets private if authorization privided | 142 | not-applicable | — | — | Renovate's PyPI hostRules authorization and isPrivate result flag are not implemented in Rust. |
| supports multiple custom datasource urls | 159 | not-applicable | — | — | Renovate's PyPI multi-registry aggregation/fallback is not implemented in Rust; Rust fetches one configured API base at a time. |
| supports Google Auth | 194 | not-applicable | — | — | Renovate's PyPI Google Artifact Registry authentication is not implemented in Rust. |
| supports Google Auth not being configured | 222 | not-applicable | — | — | Renovate's PyPI Google Artifact Registry authentication fallback is not implemented in Rust. |
| returns non-github home_page | 246 | not-applicable | — | — | Renovate's PyPI homepage/source URL metadata mapping is not implemented in Rust. |
| find url from project_urls | 267 | not-applicable | — | — | Renovate's PyPI project_urls source/changelog metadata mapping is not implemented in Rust. |
| excludes gh sponsors url from project_urls | 291 | not-applicable | — | — | Renovate's PyPI project_urls filtering is not implemented in Rust. |
| does not mistake sponsors in project name as sponsors url | 310 | not-applicable | — | — | Renovate's PyPI project_urls filtering is not implemented in Rust. |
| normalizes the package name according to PEP 503 | 329 | ported | `pypi.rs` | `fetch_versions_normalizes_name` | — |
| normalizes the package name according to PEP 503 when falling back to simple endpoint | 349 | not-applicable | — | — | Renovate's PyPI JSON-to-simple-endpoint fallback is not implemented in Rust. |
| normalizes the package name according to PEP 503 querying a simple endpoint | 368 | not-applicable | — | — | Renovate's PyPI simple repository endpoint parser is not implemented in Rust; Rust uses the JSON API. |
| respects constraints | 384 | not-applicable | — | — | Renovate's PyPI datasource-level Python constraints filtering is not implemented in Rust; Rust applies PEP 440 update summaries after JSON API fetches. |
| process data from simple endpoint | 413 | not-applicable | — | — | Renovate's PyPI simple repository HTML parser is not implemented in Rust. |
| process data from +simple endpoint | 431 | not-applicable | — | — | Renovate's PyPI +simple repository HTML parser is not implemented in Rust. |
| sets private simple if authorization provided | 449 | not-applicable | — | — | Renovate's PyPI simple endpoint hostRules authorization and isPrivate flag are not implemented in Rust. |
| process data from simple endpoint with hyphens | 470 | not-applicable | — | — | Renovate's PyPI simple repository filename parser is not implemented in Rust. |
| process data from simple endpoint with zip archives | 490 | not-applicable | — | — | Renovate's PyPI simple repository archive filename parser is not implemented in Rust. |
| process data from simple endpoint with hyphens replaced with underscores | 509 | not-applicable | — | — | Renovate's PyPI simple repository filename normalization is not implemented in Rust. |
| process data from simple endpoint with mixed-case characters | 527 | not-applicable | — | — | Renovate's PyPI simple repository HTML parser is not implemented in Rust. |
| process data from simple endpoint with mixed-case characters when using lower case dependency name | 547 | not-applicable | — | — | Renovate's PyPI simple repository HTML parser is not implemented in Rust. |
| process data from simple endpoint with periods | 567 | not-applicable | — | — | Renovate's PyPI simple repository HTML parser is not implemented in Rust. |
| process data from simple endpoint with periods when using normalized name | 587 | not-applicable | — | — | Renovate's PyPI simple repository HTML parser is not implemented in Rust. |
| process data from simple endpoint for snowflake-legacy | 607 | not-applicable | — | — | Renovate's PyPI simple repository HTML parser is not implemented in Rust. |
| ignores invalid distribution file name formats | 633 | not-applicable | — | — | Renovate's PyPI simple repository distribution filename parser is not implemented in Rust. |
| process data from simple endpoint with non normalized name | 649 | not-applicable | — | — | Renovate's PyPI simple repository HTML parser is not implemented in Rust. |
| process data from simple endpoint with extra whitespaces in html | 674 | not-applicable | — | — | Renovate's PyPI simple repository HTML parser is not implemented in Rust. |
| returns null for empty response | 694 | not-applicable | — | — | Renovate's PyPI simple endpoint empty-response contract is not implemented in Rust. |
| returns null for 404 response from simple endpoint | 712 | not-applicable | — | — | Renovate's PyPI simple endpoint null-on-error contract is not implemented in Rust. |
| returns null for response with no versions | 730 | not-applicable | — | — | Renovate's PyPI simple endpoint no-version contract is not implemented in Rust. |
| fall back from json and process data from simple endpoint | 748 | not-applicable | — | — | Renovate's PyPI JSON-to-simple-endpoint fallback is not implemented in Rust. |
| parses data-requires-python and respects constraints from simple endpoint | 771 | not-applicable | — | — | Renovate's PyPI simple endpoint `data-requires-python` parser and datasource-level constraints filtering are not implemented in Rust. |

### `modules/datasource/pypi/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports Google Auth with simple endpoint | 791 | not-applicable | — | — | Renovate's PyPI Google Artifact Registry authentication and simple endpoint parser are not implemented in Rust. |
| sanitizes GAR userinfo when Google auth is used | 822 | not-applicable | — | — | Renovate's PyPI Google Artifact Registry auth URL sanitization is not implemented in Rust. |
| ignores an invalid URL when checking for auth headers | 853 | not-applicable | — | — | Renovate's PyPI registry URL auth-header validation is not implemented in Rust. |
| uses https://pypi.org/pypi/ instead of https://pypi.org/simple/ | 865 | not-applicable | — | — | Renovate's PyPI registry URL rewriting from simple to JSON API is not implemented in Rust; Rust expects the JSON API base URL. |

---

