# `lib/util/cache/package/ttl.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**18/31 in-scope tests ported** (13 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | returns undefined when no cachettloverride config exists | ported | [`crates/renovate-core/src/cache/package.rs:1341`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1341) |
| 20 | returns undefined when cachettloverride is empty | pending | — |
| 30 | returns exact match when namespace exists in config | ported | [`crates/renovate-core/src/cache/package.rs:1349`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1349) |
| 45 | returns undefined when exact match is not a number | pending | — |
| 58 | returns undefined when no matching namespace found | pending | — |
| 72 | matches simple glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1361`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1361) |
| 88 | matches wildcard pattern for all namespaces | ported | [`crates/renovate-core/src/cache/package.rs:1374`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1374) |
| 108 | matches complex glob patterns with braces | ported | [`crates/renovate-core/src/cache/package.rs:1390`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1390) |
| 124 | handles special characters in namespace patterns | pending | — |
| 143 | matches regex patterns | ported | [`crates/renovate-core/src/cache/package.rs:1403`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1403) |
| 161 | matches patterns with regex escape sequences | ported | [`crates/renovate-core/src/cache/package.rs:1448`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1448) |
| 179 | prioritizes exact match over glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1415`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1415) |
| 195 | returns longest matching pattern when multiple patterns apply | ported | [`crates/renovate-core/src/cache/package.rs:1432`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1432) |
| 209 | selects longest matching pattern across all configs | ported | [`crates/renovate-core/src/cache/package.rs:1460`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1460) |
| 228 | skips non-numeric values and selects next longest matching pattern | pending | — |
| 243 | returns undefined when no patterns match | pending | — |
| 256 | applies patterns consistently regardless of case in config order | ported | [`crates/renovate-core/src/cache/package.rs:1579`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1579) |
| 271 | handles empty string pattern | ported | [`crates/renovate-core/src/cache/package.rs:1606`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1606) |
| 286 | treats null and undefined values as invalid | pending | — |
| 306 | handles very large numbers | ported | [`crates/renovate-core/src/cache/package.rs:1487`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1487) |
| 318 | handles negative numbers | ported | [`crates/renovate-core/src/cache/package.rs:1477`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1477) |
| 330 | treats string numbers as invalid, only accepts number types | pending | — |
| 350 | returns default values when no overrides set | pending | — |
| 363 | uses override for softttlminutes when available | pending | — |
| 378 | applies custom cachehardttlminutes from config | pending | — |
| 391 | resolves ttl with glob pattern overrides | ported | [`crates/renovate-core/src/cache/package.rs:1497`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1497) |
| 407 | resolves ttl correctly with multiple overlapping overrides | ported | [`crates/renovate-core/src/cache/package.rs:1510`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1510) |
| 427 | uses maximum of softttlminutes and cachehardttlminutes for hardttlminutes | pending | — |
| 443 | handles negative cachehardttlminutes config | ported | [`crates/renovate-core/src/cache/package.rs:1528`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1528) |
| 461 | handles zero as valid override value | ported | [`crates/renovate-core/src/cache/package.rs:1542`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1542) |
| 477 | uses fallback when override is not a number | pending | — |

