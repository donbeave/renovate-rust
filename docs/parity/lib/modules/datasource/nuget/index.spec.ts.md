# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/datasource/nuget/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/nuget/index.spec.ts
**Total tests:** 36 | **Ported:** 5 | **Actionable:** 31 | **Status:** partial

### `modules/datasource/nuget/index › parseRegistryUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts feed version from registry URL hash (v3) | 127 | pending | — | — | —|
| extracts feed version from registry URL hash (v2) | 134 | pending | — | — | —|
| defaults to v2 | 141 | pending | — | — | —|
| returns null for unparseable | 148 | pending | — | — | —|

### `modules/datasource/nuget/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| can't detect nuget feed version | 162 | pending | — | — | —|
| extracts feed version from registry URL hash | 177 | pending | — | — | —|
| can't get packages list (v3) | 192 | pending | — | — | —|
| empty packages list (v3) | 207 | pending | — | — | —|
| returns null for empty result (v3v2) | 222 | pending | — | — | —|
| returns null for empty result (v2) | 240 | pending | — | — | —|
| returns null for empty result (v3) | 254 | pending | — | — | —|
| logs instead of triggering a TypeError when PackageBaseAddress is missing from service index | 265 | pending | — | — | —|

### `modules/datasource/nuget/index › getReleases › determine source URL from nupkg`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| can determine source URL from nupkg when PackageBaseAddress is missing | 336 | pending | — | — | —|
| can handle nupkg without repository metadata | 408 | pending | — | — | —|

### `modules/datasource/nuget/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for non 200 (v3v2) | 479 | pending | — | — | —|
| returns null for non 200 (v3) | 494 | ported | `nuget.rs` | `fetch_latest_non_success_returns_none` | — |
| returns null for non 200 (v2) | 503 | pending | — | — | —|
| returns null for unknown error (v3v2) | 517 | pending | — | — | —|
| returns deduplicated results | 535 | pending | — | — | —|
| returns null for unknown error in getReleasesFromV3Feed (v3) | 561 | pending | — | — | —|
| returns null for unknown error in getQueryUrlForV3Feed  (v3) | 573 | pending | — | — | —|
| returns null for unknown error (v2) | 587 | pending | — | — | —|
| processes real data (v3) feed is a nuget.org | 601 | pending | — | — | —|
| captures release notes | 619 | pending | — | — | —|
| processes real data (v3) feed is azure devops | 639 | pending | — | — | —|
| processes real data (v3) for several catalog pages | 684 | pending | — | — | —|
| processes real data (v3) feed is not a nuget.org | 702 | pending | — | — | —|
| processes real data (v3) nuspec fetch error | 731 | pending | — | — | —|
| processes real data (v3) nuspec fetch 404 error | 749 | pending | — | — | —|
| processes real data (v2) | 767 | pending | — | — | —|
| processes real data no release (v2) | 782 | pending | — | — | —|
| processes real data without project url (v2) | 795 | pending | — | — | —|
| processes real data with no github project url (v2) | 810 | pending | — | — | —|
| extracts latest tag (v2) | 824 | pending | — | — | —|
| handles paginated results (v2) | 838 | pending | — | — | —|
| should return deprecated | 856 | pending | — | — | —|

---
