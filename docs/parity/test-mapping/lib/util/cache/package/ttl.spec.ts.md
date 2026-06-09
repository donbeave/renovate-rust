# `lib/util/cache/package/ttl.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**31/31 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | returns undefined when no cachettloverride config exists | ported | [`crates/renovate-core/src/cache/package.rs:1488`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1488) |
| 20 | returns undefined when cachettloverride is empty | ported | [`crates/renovate-core/src/cache/package.rs:1508`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1508) |
| 30 | returns exact match when namespace exists in config | ported | [`crates/renovate-core/src/cache/package.rs:1496`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1496) |
| 45 | returns undefined when exact match is not a number | ported | [`crates/renovate-core/src/cache/package.rs:1518`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1518) |
| 58 | returns undefined when no matching namespace found | ported | [`crates/renovate-core/src/cache/package.rs:1531`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1531) |
| 72 | matches simple glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1620`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1620) |
| 88 | matches wildcard pattern for all namespaces | ported | [`crates/renovate-core/src/cache/package.rs:1633`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1633) |
| 108 | matches complex glob patterns with braces | ported | [`crates/renovate-core/src/cache/package.rs:1649`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1649) |
| 124 | handles special characters in namespace patterns | ported | [`crates/renovate-core/src/cache/package.rs:1541`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1541) |
| 143 | matches regex patterns | ported | [`crates/renovate-core/src/cache/package.rs:1662`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1662) |
| 161 | matches patterns with regex escape sequences | ported | [`crates/renovate-core/src/cache/package.rs:1707`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1707) |
| 179 | prioritizes exact match over glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1674`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1674) |
| 195 | returns longest matching pattern when multiple patterns apply | ported | [`crates/renovate-core/src/cache/package.rs:1691`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1691) |
| 209 | selects longest matching pattern across all configs | ported | [`crates/renovate-core/src/cache/package.rs:1719`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1719) |
| 228 | skips non-numeric values and selects next longest matching pattern | ported | [`crates/renovate-core/src/cache/package.rs:1562`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1562) |
| 243 | returns undefined when no patterns match | ported | [`crates/renovate-core/src/cache/package.rs:1593`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1593) |
| 256 | applies patterns consistently regardless of case in config order | ported | [`crates/renovate-core/src/cache/package.rs:1838`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1838) |
| 271 | handles empty string pattern | ported | [`crates/renovate-core/src/cache/package.rs:1865`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1865) |
| 286 | treats null and undefined values as invalid | ported | [`crates/renovate-core/src/cache/package.rs:1607`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1607) |
| 306 | handles very large numbers | ported | [`crates/renovate-core/src/cache/package.rs:1746`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1746) |
| 318 | handles negative numbers | ported | [`crates/renovate-core/src/cache/package.rs:1736`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1736) |
| 330 | treats string numbers as invalid, only accepts number types | ported | [`crates/renovate-core/src/cache/package.rs:1578`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1578) |
| 350 | returns default values when no overrides set | ported | [`crates/renovate-core/src/cache/package.rs:1429`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1429) |
| 363 | uses override for softttlminutes when available | ported | [`crates/renovate-core/src/cache/package.rs:1438`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1438) |
| 378 | applies custom cachehardttlminutes from config | ported | [`crates/renovate-core/src/cache/package.rs:1451`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1451) |
| 391 | resolves ttl with glob pattern overrides | ported | [`crates/renovate-core/src/cache/package.rs:1756`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1756) |
| 407 | resolves ttl correctly with multiple overlapping overrides | ported | [`crates/renovate-core/src/cache/package.rs:1769`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1769) |
| 427 | uses maximum of softttlminutes and cachehardttlminutes for hardttlminutes | ported | [`crates/renovate-core/src/cache/package.rs:1463`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1463) |
| 443 | handles negative cachehardttlminutes config | ported | [`crates/renovate-core/src/cache/package.rs:1787`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1787) |
| 461 | handles zero as valid override value | ported | [`crates/renovate-core/src/cache/package.rs:1801`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1801) |
| 477 | uses fallback when override is not a number | ported | [`crates/renovate-core/src/cache/package.rs:1476`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1476) |

