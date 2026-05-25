# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/dart/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/dart/index.spec.ts
**Total tests:** 6 | **Ported:** 1 | **Actionable:** 6 | **Status:** partial

### `modules/datasource/dart/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 13 | pending | — | — | — |
| returns null for empty fields | 23 | pending | — | — | — |
| returns null for 404 | 55 | ported | `pub_dev.rs` | `fetch_latest_404_returns_none` | — |
| throws for 5xx | 65 | pending | — | — | — |
| returns null for unknown error | 75 | pending | — | — | — |
| processes real data | 85 | pending | — | — | — |

---

