# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/pod/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/pod/index.spec.ts
**Total tests:** 19 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `modules/datasource/pod/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid inputs | 26 | not-applicable | — | — | Renovate's CocoaPods CDN shard path and invalid-input fallback are not implemented in Rust; Rust uses the trunk REST API with an explicit pod name. |
| returns null disabled host | 41 | not-applicable | — | — | Renovate's hostRules disabled-host behavior is not implemented in the Rust CocoaPods datasource. |
| returns null for empty result | 51 | not-applicable | — | — | Renovate's CocoaPods CDN all_pods_versions shard lookup is not implemented in Rust; Rust uses the trunk REST API. |
| returns null for 404 | 60 | ported | `cocoapods.rs` | `fetch_latest_404_returns_none` | Rust verifies the equivalent trunk API missing-pod behavior. |
| returns null for 404 Github enterprise | 78 | not-applicable | — | — | Renovate's GitHub Enterprise Specs repository content traversal is not implemented in Rust; Rust uses the trunk REST API. |
| returns null for 404 Github enterprise with different url style | 99 | not-applicable | — | — | Renovate's GitHub Enterprise Specs repository URL normalization is not implemented in Rust; Rust uses the trunk REST API. |
| returns null for 401 | 117 | ported | `cocoapods.rs` | `fetch_latest_401_returns_none` | Rust verifies the equivalent non-success trunk API behavior. |
| throws for 429 | 125 | not-applicable | — | — | Renovate's CocoaPods external-host-error contract for rate limits is not implemented in Rust; Rust treats non-success trunk responses as missing latest-version data. |
| throws for 500 | 133 | not-applicable | — | — | Renovate's CocoaPods external-host-error contract for server errors is not implemented in Rust; Rust treats non-success trunk responses as missing latest-version data. |
| returns null for unknown error | 141 | not-applicable | — | — | Renovate's CocoaPods null-on-network-error contract is not implemented in Rust; Rust propagates HTTP client errors. |
| processes real data from CDN | 149 | not-applicable | — | — | Renovate's CocoaPods CDN all_pods_versions response parsing is not implemented in Rust; Rust uses the trunk REST API. |
| processes real data from Github with shard with specs | 169 | not-applicable | — | — | Renovate's GitHub Specs repository content traversal is not implemented in Rust; Rust uses the trunk REST API. |
| processes real data from Github with shard without specs | 188 | not-applicable | — | — | Renovate's GitHub Specs repository fallback path traversal is not implemented in Rust; Rust uses the trunk REST API. |
| processes real data from Github with specs without shard | 209 | not-applicable | — | — | Renovate's GitHub Specs repository fallback path traversal is not implemented in Rust; Rust uses the trunk REST API. |
| processes real data from Github without specs without shard | 232 | not-applicable | — | — | Renovate's GitHub Specs repository fallback path traversal is not implemented in Rust; Rust uses the trunk REST API. |
| processes real data from Github Enterprise with shard with specs | 257 | not-applicable | — | — | Renovate's GitHub Enterprise Specs repository content traversal is not implemented in Rust; Rust uses the trunk REST API. |
| processes real data from Github Enterprise with shard without specs | 276 | not-applicable | — | — | Renovate's GitHub Enterprise Specs repository fallback path traversal is not implemented in Rust; Rust uses the trunk REST API. |
| processes real data from Github Enterprise with specs without shard | 297 | not-applicable | — | — | Renovate's GitHub Enterprise Specs repository fallback path traversal is not implemented in Rust; Rust uses the trunk REST API. |
| processes real data from Github Enterprise without specs without shard | 320 | not-applicable | — | — | Renovate's GitHub Enterprise Specs repository fallback path traversal is not implemented in Rust; Rust uses the trunk REST API. |

---

