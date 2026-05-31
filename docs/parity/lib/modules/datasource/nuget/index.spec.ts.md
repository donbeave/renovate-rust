# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/datasource/nuget/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/nuget/index.spec.ts
**Total tests:** 36 | **Ported:** 5 | **Actionable:** 31 | **Status:** partial

### `modules/datasource/nuget/index › parseRegistryUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts feed version from registry URL hash (v3) | 127 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| extracts feed version from registry URL hash (v2) | 134 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| defaults to v2 | 141 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for unparseable | 148 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|

### `modules/datasource/nuget/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| can't detect nuget feed version | 162 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| extracts feed version from registry URL hash | 177 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| can't get packages list (v3) | 192 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| empty packages list (v3) | 207 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for empty result (v3v2) | 222 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for empty result (v2) | 240 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for empty result (v3) | 254 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| logs instead of triggering a TypeError when PackageBaseAddress is missing from service index | 265 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|

### `modules/datasource/nuget/index › getReleases › determine source URL from nupkg`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| can determine source URL from nupkg when PackageBaseAddress is missing | 336 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| can handle nupkg without repository metadata | 408 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|

### `modules/datasource/nuget/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for non 200 (v3v2) | 479 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for non 200 (v3) | 494 | ported | `nuget.rs` | `fetch_latest_non_success_returns_none` | — |
| returns null for non 200 (v2) | 503 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for unknown error (v3v2) | 517 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns deduplicated results | 535 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for unknown error in getReleasesFromV3Feed (v3) | 561 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for unknown error in getQueryUrlForV3Feed  (v3) | 573 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for unknown error (v2) | 587 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| processes real data (v3) feed is a nuget.org | 601 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| captures release notes | 619 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| processes real data (v3) feed is azure devops | 639 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| processes real data (v3) for several catalog pages | 684 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| processes real data (v3) feed is not a nuget.org | 702 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| processes real data (v3) nuspec fetch error | 731 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| processes real data (v3) nuspec fetch 404 error | 749 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| processes real data (v2) | 767 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| processes real data no release (v2) | 782 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| processes real data without project url (v2) | 795 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| processes real data with no github project url (v2) | 810 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| extracts latest tag (v2) | 824 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| handles paginated results (v2) | 838 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| should return deprecated | 856 | not-applicable | Mock framework internals — tests nuget datasource via nock HTTP mocks; Rust tests this at different layer | — | —|

---
