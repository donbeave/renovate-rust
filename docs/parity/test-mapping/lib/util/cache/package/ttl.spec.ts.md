# `lib/util/cache/package/ttl.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**31/31 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | returns undefined when no cachettloverride config exists | ported | [`crates/renovate-core/src/cache/package.rs:1487`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1487) |
| 20 | returns undefined when cachettloverride is empty | ported | [`crates/renovate-core/src/cache/package.rs:1507`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1507) |
| 30 | returns exact match when namespace exists in config | ported | [`crates/renovate-core/src/cache/package.rs:1495`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1495) |
| 45 | returns undefined when exact match is not a number | ported | [`crates/renovate-core/src/cache/package.rs:1517`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1517) |
| 58 | returns undefined when no matching namespace found | ported | [`crates/renovate-core/src/cache/package.rs:1530`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1530) |
| 72 | matches simple glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1619`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1619) |
| 88 | matches wildcard pattern for all namespaces | ported | [`crates/renovate-core/src/cache/package.rs:1632`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1632) |
| 108 | matches complex glob patterns with braces | ported | [`crates/renovate-core/src/cache/package.rs:1648`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1648) |
| 124 | handles special characters in namespace patterns | ported | [`crates/renovate-core/src/cache/package.rs:1540`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1540) |
| 143 | matches regex patterns | ported | [`crates/renovate-core/src/cache/package.rs:1661`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1661) |
| 161 | matches patterns with regex escape sequences | ported | [`crates/renovate-core/src/cache/package.rs:1706`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1706) |
| 179 | prioritizes exact match over glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1673`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1673) |
| 195 | returns longest matching pattern when multiple patterns apply | ported | [`crates/renovate-core/src/cache/package.rs:1690`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1690) |
| 209 | selects longest matching pattern across all configs | ported | [`crates/renovate-core/src/cache/package.rs:1718`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1718) |
| 228 | skips non-numeric values and selects next longest matching pattern | ported | [`crates/renovate-core/src/cache/package.rs:1561`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1561) |
| 243 | returns undefined when no patterns match | ported | [`crates/renovate-core/src/cache/package.rs:1592`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1592) |
| 256 | applies patterns consistently regardless of case in config order | ported | [`crates/renovate-core/src/cache/package.rs:1837`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1837) |
| 271 | handles empty string pattern | ported | [`crates/renovate-core/src/cache/package.rs:1864`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1864) |
| 286 | treats null and undefined values as invalid | ported | [`crates/renovate-core/src/cache/package.rs:1606`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1606) |
| 306 | handles very large numbers | ported | [`crates/renovate-core/src/cache/package.rs:1745`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1745) |
| 318 | handles negative numbers | ported | [`crates/renovate-core/src/cache/package.rs:1735`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1735) |
| 330 | treats string numbers as invalid, only accepts number types | ported | [`crates/renovate-core/src/cache/package.rs:1577`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1577) |
| 350 | returns default values when no overrides set | ported | [`crates/renovate-core/src/cache/package.rs:1428`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1428) |
| 363 | uses override for softttlminutes when available | ported | [`crates/renovate-core/src/cache/package.rs:1437`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1437) |
| 378 | applies custom cachehardttlminutes from config | ported | [`crates/renovate-core/src/cache/package.rs:1450`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1450) |
| 391 | resolves ttl with glob pattern overrides | ported | [`crates/renovate-core/src/cache/package.rs:1755`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1755) |
| 407 | resolves ttl correctly with multiple overlapping overrides | ported | [`crates/renovate-core/src/cache/package.rs:1768`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1768) |
| 427 | uses maximum of softttlminutes and cachehardttlminutes for hardttlminutes | ported | [`crates/renovate-core/src/cache/package.rs:1462`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1462) |
| 443 | handles negative cachehardttlminutes config | ported | [`crates/renovate-core/src/cache/package.rs:1786`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1786) |
| 461 | handles zero as valid override value | ported | [`crates/renovate-core/src/cache/package.rs:1800`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1800) |
| 477 | uses fallback when override is not a number | ported | [`crates/renovate-core/src/cache/package.rs:1475`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1475) |

