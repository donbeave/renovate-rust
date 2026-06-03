# `lib/modules/datasource/rpm/index.spec.ts`

[← `datasource/rpm`](../../../../_by-module/datasource/rpm.md) · [all modules](../../../../README.md)

**0/28 in-scope tests ported** (28 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 43 | returns the correct primary.xml url | pending | — |
| 64 | returns the correct primary.xml url when repomd.xml omits xml declaration | pending | — |
| 85 | throws an error if repomd.xml is missing | pending | — |
| 95 | throws an error if http.gettext fails | pending | — |
| 106 | throws an error if repomdxml is not in xml format | pending | — |
| 126 | throws an error if no primary data is found | pending | — |
| 146 | throws an error if no location element is found | pending | — |
| 168 | throws an error if location href is missing | pending | — |
| 211 | returns the correct releases | pending | — |
| 252 | throws an error if somesha256-primary.xml.gz is not found | pending | — |
| 265 | throws an error if response.body is empty | pending | — |
| 276 | rethrows non-error fetch failures | pending | — |
| 292 | reuses the extracted primary.xml file across package lookups | pending | — |
| 336 | re-downloads primary.xml if the freshness check fails | pending | — |
| 380 | throws if extracting primary.xml fails without an existing cache file | pending | — |
| 414 | keeps the previous extracted primary.xml if a refresh extract fails | pending | — |
| 471 | replaces the extracted primary.xml after a successful refresh | pending | — |
| 519 | returns null if no element package is found in primary.xml | pending | — |
| 538 | returns null if the specific packagename is not found in primary.xml | pending | — |
| 557 | returns null if version is not found in a version element | pending | — |
| 576 | returns null if version element is missing the ver attribute | pending | — |
| 595 | returns an array of releases without duplicate versionwithrel | pending | — |
| 621 | handles parser error event in getreleasesbypackagename | pending | — |
| 640 | returns null if registryurl is not provided | pending | — |
| 649 | returns null if packagename is not provided | pending | — |
| 658 | returns the correct releases | pending | — |
| 686 | throws an error if getprimarygzipurl fails | pending | — |
| 699 | throws an error if getreleasesbypackagename fails | pending | — |

