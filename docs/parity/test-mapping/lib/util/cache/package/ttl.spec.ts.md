# `lib/util/cache/package/ttl.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**30/31 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | returns undefined when no cachettloverride config exists | ported | [`crates/renovate-core/src/cache/package.rs:1487`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1487) |
| 20 | returns undefined when cachettloverride is empty | ported | [`crates/renovate-core/src/cache/package.rs:1507`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1507) |
| 30 | returns exact match when namespace exists in config | ported | [`crates/renovate-core/src/cache/package.rs:1495`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1495) |
| 45 | returns undefined when exact match is not a number | ported | [`crates/renovate-core/src/cache/package.rs:1517`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1517) |
| 58 | returns undefined when no matching namespace found | ported | [`crates/renovate-core/src/cache/package.rs:1530`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1530) |
| 72 | matches simple glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1604`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1604) |
| 88 | matches wildcard pattern for all namespaces | ported | [`crates/renovate-core/src/cache/package.rs:1617`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1617) |
| 108 | matches complex glob patterns with braces | ported | [`crates/renovate-core/src/cache/package.rs:1633`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1633) |
| 124 | handles special characters in namespace patterns | ported | [`crates/renovate-core/src/cache/package.rs:1540`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1540) |
| 143 | matches regex patterns | ported | [`crates/renovate-core/src/cache/package.rs:1646`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1646) |
| 161 | matches patterns with regex escape sequences | ported | [`crates/renovate-core/src/cache/package.rs:1691`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1691) |
| 179 | prioritizes exact match over glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1658`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1658) |
| 195 | returns longest matching pattern when multiple patterns apply | ported | [`crates/renovate-core/src/cache/package.rs:1675`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1675) |
| 209 | selects longest matching pattern across all configs | ported | [`crates/renovate-core/src/cache/package.rs:1703`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1703) |
| 228 | skips non-numeric values and selects next longest matching pattern | ported | [`crates/renovate-core/src/cache/package.rs:1561`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1561) |
| 243 | returns undefined when no patterns match | ported | [`crates/renovate-core/src/cache/package.rs:1577`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1577) |
| 256 | applies patterns consistently regardless of case in config order | ported | [`crates/renovate-core/src/cache/package.rs:1822`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1822) |
| 271 | handles empty string pattern | ported | [`crates/renovate-core/src/cache/package.rs:1849`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1849) |
| 286 | treats null and undefined values as invalid | ported | [`crates/renovate-core/src/cache/package.rs:1591`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1591) |
| 306 | handles very large numbers | ported | [`crates/renovate-core/src/cache/package.rs:1730`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1730) |
| 318 | handles negative numbers | ported | [`crates/renovate-core/src/cache/package.rs:1720`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1720) |
| 330 | treats string numbers as invalid, only accepts number types | pending | — |
| 350 | returns default values when no overrides set | ported | [`crates/renovate-core/src/cache/package.rs:1428`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1428) |
| 363 | uses override for softttlminutes when available | ported | [`crates/renovate-core/src/cache/package.rs:1437`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1437) |
| 378 | applies custom cachehardttlminutes from config | ported | [`crates/renovate-core/src/cache/package.rs:1450`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1450) |
| 391 | resolves ttl with glob pattern overrides | ported | [`crates/renovate-core/src/cache/package.rs:1740`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1740) |
| 407 | resolves ttl correctly with multiple overlapping overrides | ported | [`crates/renovate-core/src/cache/package.rs:1753`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1753) |
| 427 | uses maximum of softttlminutes and cachehardttlminutes for hardttlminutes | ported | [`crates/renovate-core/src/cache/package.rs:1462`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1462) |
| 443 | handles negative cachehardttlminutes config | ported | [`crates/renovate-core/src/cache/package.rs:1771`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1771) |
| 461 | handles zero as valid override value | ported | [`crates/renovate-core/src/cache/package.rs:1785`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1785) |
| 477 | uses fallback when override is not a number | ported | [`crates/renovate-core/src/cache/package.rs:1475`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1475) |

