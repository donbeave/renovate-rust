# `lib/util/cache/package/ttl.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**25/31 in-scope tests ported** (6 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | returns undefined when no cachettloverride config exists | ported | [`crates/renovate-core/src/cache/package.rs:1430`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1430) |
| 20 | returns undefined when cachettloverride is empty | ported | [`crates/renovate-core/src/cache/package.rs:1450`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1450) |
| 30 | returns exact match when namespace exists in config | ported | [`crates/renovate-core/src/cache/package.rs:1438`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1438) |
| 45 | returns undefined when exact match is not a number | ported | [`crates/renovate-core/src/cache/package.rs:1460`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1460) |
| 58 | returns undefined when no matching namespace found | ported | [`crates/renovate-core/src/cache/package.rs:1473`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1473) |
| 72 | matches simple glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1547`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1547) |
| 88 | matches wildcard pattern for all namespaces | ported | [`crates/renovate-core/src/cache/package.rs:1560`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1560) |
| 108 | matches complex glob patterns with braces | ported | [`crates/renovate-core/src/cache/package.rs:1576`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1576) |
| 124 | handles special characters in namespace patterns | ported | [`crates/renovate-core/src/cache/package.rs:1483`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1483) |
| 143 | matches regex patterns | ported | [`crates/renovate-core/src/cache/package.rs:1589`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1589) |
| 161 | matches patterns with regex escape sequences | ported | [`crates/renovate-core/src/cache/package.rs:1634`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1634) |
| 179 | prioritizes exact match over glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1601`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1601) |
| 195 | returns longest matching pattern when multiple patterns apply | ported | [`crates/renovate-core/src/cache/package.rs:1618`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1618) |
| 209 | selects longest matching pattern across all configs | ported | [`crates/renovate-core/src/cache/package.rs:1646`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1646) |
| 228 | skips non-numeric values and selects next longest matching pattern | ported | [`crates/renovate-core/src/cache/package.rs:1504`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1504) |
| 243 | returns undefined when no patterns match | ported | [`crates/renovate-core/src/cache/package.rs:1520`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1520) |
| 256 | applies patterns consistently regardless of case in config order | ported | [`crates/renovate-core/src/cache/package.rs:1765`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1765) |
| 271 | handles empty string pattern | ported | [`crates/renovate-core/src/cache/package.rs:1792`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1792) |
| 286 | treats null and undefined values as invalid | ported | [`crates/renovate-core/src/cache/package.rs:1534`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1534) |
| 306 | handles very large numbers | ported | [`crates/renovate-core/src/cache/package.rs:1673`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1673) |
| 318 | handles negative numbers | ported | [`crates/renovate-core/src/cache/package.rs:1663`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1663) |
| 330 | treats string numbers as invalid, only accepts number types | pending | — |
| 350 | returns default values when no overrides set | pending | — |
| 363 | uses override for softttlminutes when available | pending | — |
| 378 | applies custom cachehardttlminutes from config | pending | — |
| 391 | resolves ttl with glob pattern overrides | ported | [`crates/renovate-core/src/cache/package.rs:1683`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1683) |
| 407 | resolves ttl correctly with multiple overlapping overrides | ported | [`crates/renovate-core/src/cache/package.rs:1696`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1696) |
| 427 | uses maximum of softttlminutes and cachehardttlminutes for hardttlminutes | pending | — |
| 443 | handles negative cachehardttlminutes config | ported | [`crates/renovate-core/src/cache/package.rs:1714`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1714) |
| 461 | handles zero as valid override value | ported | [`crates/renovate-core/src/cache/package.rs:1728`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1728) |
| 477 | uses fallback when override is not a number | pending | — |

