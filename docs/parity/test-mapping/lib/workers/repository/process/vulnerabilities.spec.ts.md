# `lib/workers/repository/process/vulnerabilities.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**0/41 ported** (41 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 29 | works, and is a singleton | pending | — |
| 38 | throws when osv-offline error | pending | — |
| 62 | return list of vulnerabilities | pending | — |
| 129 | are marked for dependencies with a mal- advisory id against their current version with malicious-version-in-use | pending | — |
| 251 | are logged | pending | — |
| 369 | are not counted if the affected versions do not match | pending | — |
| 426 | handles a mal- advisory with no affected field | pending | — |
| 477 | handles a malicious dependency where updates is undefined | pending | — |
| 526 | applies to dependency updates, and sets malicious-update-proposed | pending | — |
| 590 | logs | pending | — |
| 662 | falls back to update.newvalue when newversion is missing, and skips updates that are not malicious | pending | — |
| 760 | unsupported datasource | pending | — |
| 780 | package found but no vulnerabilities | pending | — |
| 801 | vulnerability without affected field | pending | — |
| 826 | withdrawn vulnerability | pending | — |
| 855 | invalid dep version | pending | — |
| 882 | exception while fetching vulnerabilities | pending | — |
| 911 | log event with invalid version | pending | — |
| 960 | no version or range affected | pending | — |
| 990 | no fixed version available | pending | — |
| 1024 | does not accidentally downgrade versions due to fixed version for other range | pending | — |
| 1067 | vulnerability with multiple unsorted events | pending | — |
| 1135 | creates vulnerability alert for go toolchain directive using stdlib | pending | — |
| 1196 | skips vulnerability lookup for go module directive | pending | — |
| 1221 | sets default datasource versioning to align with allowedversions on packagerule | pending | — |
| 1292 | vulnerability with multiple affected entries and version ranges | pending | — |
| 1364 | package rules are sorted by fixed version even if affected is unsorted | pending | — |
| 1433 | filters not applicable vulnerability | pending | — |
| 1453 | returns a single packagerule for regex manager | pending | — |
| 1557 | returns multiple packagerules for different vulnerabilities | pending | — |
| 1617 | returns packagerules for hackage | pending | — |
| 1674 | filters not applicable vulnerability based on last_affected version | pending | — |
| 1717 | describe fixed version as ecosystem-specific version constraint | pending | — |
| 1842 | describe last_affected version as ecosystem-specific version constraint | pending | — |
| 1896 | returns packagerule for deps-edn package using osv maven ecosystem | pending | — |
| 1948 | returns packagerule based on last_affected version | pending | — |
| 2025 | handles invalid cvss scores gracefully | pending | — |
| 2109 | prefer cvss_v4 scores over cvss_v3 | pending | — |
| 2201 | show severity text in ghsa advisories without cvss score | pending | — |
| 2263 | formats headings of vulnerability details | pending | — |
| 2361 | _(it.each / template — verify manually)_ | ? | — |

