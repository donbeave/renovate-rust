# `lib/util/cache/package/ttl.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**18/31 ported** (13 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 12 | returns undefined when no cachettloverride config exists | ported | [`crates/renovate-core/src/cache/package.rs:1328`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1328) |
| 20 | returns undefined when cachettloverride is empty | pending | — |
| 30 | returns exact match when namespace exists in config | ported | [`crates/renovate-core/src/cache/package.rs:1336`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1336) |
| 45 | returns undefined when exact match is not a number | pending | — |
| 58 | returns undefined when no matching namespace found | pending | — |
| 72 | matches simple glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1348`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1348) |
| 88 | matches wildcard pattern for all namespaces | ported | [`crates/renovate-core/src/cache/package.rs:1361`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1361) |
| 108 | matches complex glob patterns with braces | ported | [`crates/renovate-core/src/cache/package.rs:1377`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1377) |
| 124 | handles special characters in namespace patterns | pending | — |
| 143 | matches regex patterns | ported | [`crates/renovate-core/src/cache/package.rs:1390`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1390) |
| 161 | matches patterns with regex escape sequences | ported | [`crates/renovate-core/src/cache/package.rs:1435`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1435) |
| 179 | prioritizes exact match over glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1402`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1402) |
| 195 | returns longest matching pattern when multiple patterns apply | ported | [`crates/renovate-core/src/cache/package.rs:1419`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1419) |
| 209 | selects longest matching pattern across all configs | ported | [`crates/renovate-core/src/cache/package.rs:1447`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1447) |
| 228 | skips non-numeric values and selects next longest matching pattern | pending | — |
| 243 | returns undefined when no patterns match | pending | — |
| 256 | applies patterns consistently regardless of case in config order | ported | [`crates/renovate-core/src/cache/package.rs:1566`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1566) |
| 271 | handles empty string pattern | ported | [`crates/renovate-core/src/cache/package.rs:1593`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1593) |
| 286 | treats null and undefined values as invalid | pending | — |
| 306 | handles very large numbers | ported | [`crates/renovate-core/src/cache/package.rs:1474`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1474) |
| 318 | handles negative numbers | ported | [`crates/renovate-core/src/cache/package.rs:1464`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1464) |
| 330 | treats string numbers as invalid, only accepts number types | pending | — |
| 350 | returns default values when no overrides set | pending | — |
| 363 | uses override for softttlminutes when available | pending | — |
| 378 | applies custom cachehardttlminutes from config | pending | — |
| 391 | resolves ttl with glob pattern overrides | ported | [`crates/renovate-core/src/cache/package.rs:1484`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1484) |
| 407 | resolves ttl correctly with multiple overlapping overrides | ported | [`crates/renovate-core/src/cache/package.rs:1497`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1497) |
| 427 | uses maximum of softttlminutes and cachehardttlminutes for hardttlminutes | pending | — |
| 443 | handles negative cachehardttlminutes config | ported | [`crates/renovate-core/src/cache/package.rs:1515`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1515) |
| 461 | handles zero as valid override value | ported | [`crates/renovate-core/src/cache/package.rs:1529`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1529) |
| 477 | uses fallback when override is not a number | pending | — |

