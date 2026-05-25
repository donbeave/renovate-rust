# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/pod/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/pod/index.spec.ts
**Total tests:** 19 | **Ported:** 2 | **Actionable:** 19 | **Status:** partial

### `modules/datasource/pod/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid inputs | 26 | pending | — | — | — |
| returns null disabled host | 41 | pending | — | — | — |
| returns null for empty result | 51 | pending | — | — | — |
| returns null for 404 | 60 | ported | `cocoapods.rs` | `fetch_latest_404_returns_none` | Rust verifies the equivalent trunk API missing-pod behavior. |
| returns null for 404 Github enterprise | 78 | pending | — | — | — |
| returns null for 404 Github enterprise with different url style | 99 | pending | — | — | — |
| returns null for 401 | 117 | ported | `cocoapods.rs` | `fetch_latest_401_returns_none` | Rust verifies the equivalent non-success trunk API behavior. |
| throws for 429 | 125 | pending | — | — | — |
| throws for 500 | 133 | pending | — | — | — |
| returns null for unknown error | 141 | pending | — | — | — |
| processes real data from CDN | 149 | pending | — | — | — |
| processes real data from Github with shard with specs | 169 | pending | — | — | — |
| processes real data from Github with shard without specs | 188 | pending | — | — | — |
| processes real data from Github with specs without shard | 209 | pending | — | — | — |
| processes real data from Github without specs without shard | 232 | pending | — | — | — |
| processes real data from Github Enterprise with shard with specs | 257 | pending | — | — | — |
| processes real data from Github Enterprise with shard without specs | 276 | pending | — | — | — |
| processes real data from Github Enterprise with specs without shard | 297 | pending | — | — | — |
| processes real data from Github Enterprise without specs without shard | 320 | pending | — | — | — |

---

