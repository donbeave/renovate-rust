# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/dart/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/dart/index.spec.ts
**Total tests:** 6 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `modules/datasource/dart/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 13 | not-applicable | — | — | Renovate's Dart `getReleases` full release-list schema validation and null-on-parse-error contract are not implemented in Rust; Rust exposes a latest-version pub.dev lookup. |
| returns null for empty fields | 23 | not-applicable | — | — | Renovate's Dart `getReleases` full release-list schema validation and null-on-parse-error contract are not implemented in Rust; Rust exposes a latest-version pub.dev lookup. |
| returns null for 404 | 55 | ported | `pub_dev.rs` | `fetch_latest_404_returns_none` | — |
| throws for 5xx | 65 | not-applicable | — | — | Renovate's Dart `getReleases` full release-list schema validation and external-host-error contract are not implemented in Rust; Rust exposes a latest-version pub.dev lookup. |
| returns null for unknown error | 75 | not-applicable | — | — | Renovate's Dart `getReleases` full release-list schema validation and null-on-network-error contract are not implemented in Rust; Rust exposes a latest-version pub.dev lookup. |
| processes real data | 85 | not-applicable | — | — | Renovate's Dart `getReleases` full release-list snapshot mapping is not implemented in Rust; Rust exposes a latest-version pub.dev lookup. |

---

