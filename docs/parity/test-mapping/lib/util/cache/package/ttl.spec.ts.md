# `lib/util/cache/package/ttl.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**21/31 in-scope tests ported** (10 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | returns undefined when no cachettloverride config exists | ported | [`crates/renovate-core/src/cache/package.rs:1430`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1430) |
| 20 | returns undefined when cachettloverride is empty | ported | [`crates/renovate-core/src/cache/package.rs:1450`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1450) |
| 30 | returns exact match when namespace exists in config | ported | [`crates/renovate-core/src/cache/package.rs:1438`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1438) |
| 45 | returns undefined when exact match is not a number | ported | [`crates/renovate-core/src/cache/package.rs:1460`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1460) |
| 58 | returns undefined when no matching namespace found | ported | [`crates/renovate-core/src/cache/package.rs:1473`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1473) |
| 72 | matches simple glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1483`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1483) |
| 88 | matches wildcard pattern for all namespaces | ported | [`crates/renovate-core/src/cache/package.rs:1496`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1496) |
| 108 | matches complex glob patterns with braces | ported | [`crates/renovate-core/src/cache/package.rs:1512`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1512) |
| 124 | handles special characters in namespace patterns | pending | — |
| 143 | matches regex patterns | ported | [`crates/renovate-core/src/cache/package.rs:1525`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1525) |
| 161 | matches patterns with regex escape sequences | ported | [`crates/renovate-core/src/cache/package.rs:1570`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1570) |
| 179 | prioritizes exact match over glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1537`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1537) |
| 195 | returns longest matching pattern when multiple patterns apply | ported | [`crates/renovate-core/src/cache/package.rs:1554`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1554) |
| 209 | selects longest matching pattern across all configs | ported | [`crates/renovate-core/src/cache/package.rs:1582`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1582) |
| 228 | skips non-numeric values and selects next longest matching pattern | pending | — |
| 243 | returns undefined when no patterns match | pending | — |
| 256 | applies patterns consistently regardless of case in config order | ported | [`crates/renovate-core/src/cache/package.rs:1701`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1701) |
| 271 | handles empty string pattern | ported | [`crates/renovate-core/src/cache/package.rs:1728`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1728) |
| 286 | treats null and undefined values as invalid | pending | — |
| 306 | handles very large numbers | ported | [`crates/renovate-core/src/cache/package.rs:1609`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1609) |
| 318 | handles negative numbers | ported | [`crates/renovate-core/src/cache/package.rs:1599`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1599) |
| 330 | treats string numbers as invalid, only accepts number types | pending | — |
| 350 | returns default values when no overrides set | pending | — |
| 363 | uses override for softttlminutes when available | pending | — |
| 378 | applies custom cachehardttlminutes from config | pending | — |
| 391 | resolves ttl with glob pattern overrides | ported | [`crates/renovate-core/src/cache/package.rs:1619`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1619) |
| 407 | resolves ttl correctly with multiple overlapping overrides | ported | [`crates/renovate-core/src/cache/package.rs:1632`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1632) |
| 427 | uses maximum of softttlminutes and cachehardttlminutes for hardttlminutes | pending | — |
| 443 | handles negative cachehardttlminutes config | ported | [`crates/renovate-core/src/cache/package.rs:1650`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1650) |
| 461 | handles zero as valid override value | ported | [`crates/renovate-core/src/cache/package.rs:1664`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1664) |
| 477 | uses fallback when override is not a number | pending | — |

