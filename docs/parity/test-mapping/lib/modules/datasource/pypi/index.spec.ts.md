# `lib/modules/datasource/pypi/index.spec.ts`

[← `datasource/pypi`](../../../../_by-module/datasource/pypi.md) · [all modules](../../../../README.md)

**2/39 ported** (37 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 90 | returns null for empty result | pending | — |
| 100 | returns null for 404 | pending | — |
| 111 | processes real data | pending | — |
| 121 | supports custom datasource url | ported | [`crates/renovate-core/src/datasources/pypi.rs:336`](../../../../../../../crates/renovate-core/src/datasources/pypi.rs#L336) |
| 142 | sets private if authorization privided | pending | — |
| 159 | supports multiple custom datasource urls | pending | — |
| 194 | supports google auth | pending | — |
| 222 | supports google auth not being configured | pending | — |
| 246 | returns non-github home_page | pending | — |
| 267 | find url from project_urls | pending | — |
| 291 | excludes gh sponsors url from project_urls | pending | — |
| 310 | does not mistake sponsors in project name as sponsors url | pending | — |
| 329 | normalizes the package name according to pep 503 | ported | [`crates/renovate-core/src/datasources/pypi.rs:355`](../../../../../../../crates/renovate-core/src/datasources/pypi.rs#L355) |
| 349 | normalizes the package name according to pep 503 when falling back to simple endpoint | pending | — |
| 368 | normalizes the package name according to pep 503 querying a simple endpoint | pending | — |
| 384 | respects constraints | pending | — |
| 413 | process data from simple endpoint | pending | — |
| 431 | process data from +simple endpoint | pending | — |
| 449 | sets private simple if authorization provided | pending | — |
| 470 | process data from simple endpoint with hyphens | pending | — |
| 490 | process data from simple endpoint with zip archives | pending | — |
| 509 | process data from simple endpoint with hyphens replaced with underscores | pending | — |
| 527 | process data from simple endpoint with mixed-case characters | pending | — |
| 547 | process data from simple endpoint with mixed-case characters when using lower case dependency name | pending | — |
| 567 | process data from simple endpoint with periods | pending | — |
| 587 | process data from simple endpoint with periods when using normalized name | pending | — |
| 607 | process data from simple endpoint for snowflake-legacy | pending | — |
| 633 | ignores invalid distribution file name formats | pending | — |
| 649 | process data from simple endpoint with non normalized name | pending | — |
| 674 | process data from simple endpoint with extra whitespaces in html | pending | — |
| 694 | returns null for empty response | pending | — |
| 712 | returns null for 404 response from simple endpoint | pending | — |
| 730 | returns null for response with no versions | pending | — |
| 748 | _(it.each / template — verify manually)_ | ? | — |
| 771 | parses data-requires-python and respects constraints from simple endpoint | pending | — |
| 791 | supports google auth with simple endpoint | pending | — |
| 822 | sanitizes gar userinfo when google auth is used | pending | — |
| 853 | ignores an invalid url when checking for auth headers | pending | — |
| 865 | uses https://pypi.org/pypi/ instead of https://pypi.org/simple/ | pending | — |

