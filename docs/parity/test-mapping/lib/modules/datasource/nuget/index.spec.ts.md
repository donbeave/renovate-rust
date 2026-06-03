# `lib/modules/datasource/nuget/index.spec.ts`

[← `datasource/nuget`](../../../../_by-module/datasource/nuget.md) · [all modules](../../../../README.md)

**1/36 ported** (35 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 127 | extracts feed version from registry url hash (v3) | pending | — |
| 134 | extracts feed version from registry url hash (v2) | pending | — |
| 141 | defaults to v2 | pending | — |
| 148 | returns null for unparseable | pending | — |
| 162 | can't detect nuget feed version | pending | — |
| 177 | extracts feed version from registry url hash | pending | — |
| 192 | can't get packages list (v3) | pending | — |
| 207 | empty packages list (v3) | pending | — |
| 222 | returns null for empty result (v3v2) | pending | — |
| 240 | returns null for empty result (v2) | pending | — |
| 254 | returns null for empty result (v3) | pending | — |
| 265 | logs instead of triggering a typeerror when packagebaseaddress is missing from service index | pending | — |
| 336 | can determine source url from nupkg when packagebaseaddress is missing | pending | — |
| 408 | can handle nupkg without repository metadata | pending | — |
| 479 | returns null for non 200 (v3v2) | pending | — |
| 494 | returns null for non 200 (v3) | ported | [`crates/renovate-core/src/datasources/nuget.rs:300`](../../../../../../../crates/renovate-core/src/datasources/nuget.rs#L300) |
| 503 | returns null for non 200 (v2) | pending | — |
| 517 | returns null for unknown error (v3v2) | pending | — |
| 535 | returns deduplicated results | pending | — |
| 561 | returns null for unknown error in getreleasesfromv3feed (v3) | pending | — |
| 573 | returns null for unknown error in getqueryurlforv3feed (v3) | pending | — |
| 587 | returns null for unknown error (v2) | pending | — |
| 601 | processes real data (v3) feed is a nuget.org | pending | — |
| 619 | captures release notes | pending | — |
| 639 | processes real data (v3) feed is azure devops | pending | — |
| 684 | processes real data (v3) for several catalog pages | pending | — |
| 702 | processes real data (v3) feed is not a nuget.org | pending | — |
| 731 | processes real data (v3) nuspec fetch error | pending | — |
| 749 | processes real data (v3) nuspec fetch 404 error | pending | — |
| 767 | processes real data (v2) | pending | — |
| 782 | processes real data no release (v2) | pending | — |
| 795 | processes real data without project url (v2) | pending | — |
| 810 | processes real data with no github project url (v2) | pending | — |
| 824 | extracts latest tag (v2) | pending | — |
| 838 | handles paginated results (v2) | pending | — |
| 856 | should return deprecated | pending | — |

