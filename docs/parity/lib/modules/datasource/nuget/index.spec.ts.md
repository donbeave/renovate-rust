# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/nuget/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/nuget/index.spec.ts
**Total tests:** 36 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `modules/datasource/nuget/index › parseRegistryUrl`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts feed version from registry URL hash (v3) | 127 | not-applicable | — | — | Renovate's NuGet registry URL hash parser is not implemented in Rust; Rust accepts an explicit flat-container API base URL. |
| extracts feed version from registry URL hash (v2) | 134 | not-applicable | — | — | Renovate's NuGet registry URL hash parser and v2 feed mode are not implemented in Rust. |
| defaults to v2 | 141 | not-applicable | — | — | Renovate's NuGet v2 default registry behavior is not implemented in Rust; Rust uses the v3 flat-container API. |
| returns null for unparseable | 148 | not-applicable | — | — | Renovate's NuGet registry URL hash parser is not implemented in Rust; Rust accepts an explicit flat-container API base URL. |

### `modules/datasource/nuget/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| can't detect nuget feed version | 162 | not-applicable | — | — | Renovate's NuGet feed-version detection is not implemented in Rust; Rust uses the v3 flat-container API directly. |
| extracts feed version from registry URL hash | 177 | not-applicable | — | — | Renovate's NuGet registry URL hash parser is not implemented in Rust; Rust accepts an explicit flat-container API base URL. |
| can't get packages list (v3) | 192 | not-applicable | — | — | Renovate's NuGet v3 service-index and registration feed request chain is not implemented in Rust; Rust uses flat-container latest-version lookup. |
| empty packages list (v3) | 207 | not-applicable | — | — | Renovate's NuGet v3 registration feed shape is not implemented in Rust; Rust expects flat-container `versions` JSON. |
| returns null for empty result (v3v2) | 222 | not-applicable | — | — | Renovate's NuGet multi-registry v3-to-v2 fallback is not implemented in Rust. |
| returns null for empty result (v2) | 240 | not-applicable | — | — | Renovate's NuGet v2 OData feed parser is not implemented in Rust. |
| returns null for empty result (v3) | 254 | not-applicable | — | — | Renovate's NuGet v3 service-index empty-result contract is not implemented in Rust; Rust expects flat-container `versions` JSON. |
| logs instead of triggering a TypeError when PackageBaseAddress is missing from service index | 265 | not-applicable | — | — | Renovate's NuGet service-index resource discovery and logging path are not implemented in Rust. |

### `modules/datasource/nuget/index › getReleases › determine source URL from nupkg`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| can determine source URL from nupkg when PackageBaseAddress is missing | 336 | not-applicable | — | — | Renovate's NuGet nupkg download, nuspec repository metadata extraction, and package cache are not implemented in Rust. |
| can handle nupkg without repository metadata | 408 | not-applicable | — | — | Renovate's NuGet nupkg download, nuspec repository metadata extraction, and package cache are not implemented in Rust. |

### `modules/datasource/nuget/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for non 200 (v3v2) | 479 | not-applicable | — | — | Renovate's NuGet multi-registry v3-to-v2 fallback is not implemented in Rust. |
| returns null for non 200 (v3) | 494 | ported | `nuget.rs` | `fetch_latest_non_success_returns_none` | Rust verifies the equivalent flat-container non-success response behavior. |
| returns null for non 200 (v2) | 503 | not-applicable | — | — | Renovate's NuGet v2 OData feed parser is not implemented in Rust. |
| returns null for unknown error (v3v2) | 517 | not-applicable | — | — | Renovate's NuGet multi-registry v3-to-v2 fallback and null-on-network-error contract are not implemented in Rust. |
| returns deduplicated results | 535 | not-applicable | — | — | Renovate's NuGet multi-registry release-list deduplication is not implemented in Rust; Rust returns only the latest stable version. |
| returns null for unknown error in getReleasesFromV3Feed (v3) | 561 | not-applicable | — | — | Renovate's NuGet v3 service-index network-error contract is not implemented in Rust; Rust propagates HTTP client errors outside best-effort timestamp fetches. |
| returns null for unknown error in getQueryUrlForV3Feed  (v3) | 573 | not-applicable | — | — | Renovate's NuGet v3 registration feed request chain is not implemented in Rust. |
| returns null for unknown error (v2) | 587 | not-applicable | — | — | Renovate's NuGet v2 OData feed parser is not implemented in Rust. |
| processes real data (v3) feed is a nuget.org | 601 | not-applicable | — | — | Renovate's NuGet full release-list, sourceUrl, homepage, and nuspec mapping are not implemented in Rust; Rust only returns latest stable version and best-effort timestamp. |
| captures release notes | 619 | not-applicable | — | — | Renovate's NuGet nuspec release notes extraction is not implemented in Rust. |
| processes real data (v3) feed is azure devops | 639 | not-applicable | — | — | Renovate's NuGet Azure DevOps service-index and registration feed handling are not implemented in Rust. |
| processes real data (v3) for several catalog pages | 684 | not-applicable | — | — | Renovate's NuGet paged registration catalog traversal is not implemented in Rust; Rust uses flat-container latest-version lookup. |
| processes real data (v3) feed is not a nuget.org | 702 | not-applicable | — | — | Renovate's NuGet service-index handling for arbitrary v3 feeds is not implemented in Rust. |
| processes real data (v3) nuspec fetch error | 731 | not-applicable | — | — | Renovate's NuGet nuspec fetch and sourceUrl fallback behavior are not implemented in Rust. |
| processes real data (v3) nuspec fetch 404 error | 749 | not-applicable | — | — | Renovate's NuGet nuspec fetch and sourceUrl fallback behavior are not implemented in Rust. |
| processes real data (v2) | 767 | not-applicable | — | — | Renovate's NuGet v2 OData feed parser is not implemented in Rust. |
| processes real data no release (v2) | 782 | not-applicable | — | — | Renovate's NuGet v2 OData feed parser is not implemented in Rust. |
| processes real data without project url (v2) | 795 | not-applicable | — | — | Renovate's NuGet v2 OData feed parser and project URL mapping are not implemented in Rust. |
| processes real data with no github project url (v2) | 810 | not-applicable | — | — | Renovate's NuGet v2 OData feed parser and source URL normalization are not implemented in Rust. |
| extracts latest tag (v2) | 824 | not-applicable | — | — | Renovate's NuGet v2 latest-tag extraction is not implemented in Rust. |
| handles paginated results (v2) | 838 | not-applicable | — | — | Renovate's NuGet v2 OData pagination is not implemented in Rust. |
| should return deprecated | 856 | not-applicable | — | — | Renovate's NuGet deprecation metadata mapping is not implemented in Rust. |

---

