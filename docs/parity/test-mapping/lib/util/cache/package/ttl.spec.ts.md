# `lib/util/cache/package/ttl.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**18/31 in-scope tests ported** (13 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | returns undefined when no cachettloverride config exists | ported | [`crates/renovate-core/src/cache/package.rs:1347`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1347) |
| 20 | returns undefined when cachettloverride is empty | pending | — |
| 30 | returns exact match when namespace exists in config | ported | [`crates/renovate-core/src/cache/package.rs:1355`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1355) |
| 45 | returns undefined when exact match is not a number | pending | — |
| 58 | returns undefined when no matching namespace found | pending | — |
| 72 | matches simple glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1367`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1367) |
| 88 | matches wildcard pattern for all namespaces | ported | [`crates/renovate-core/src/cache/package.rs:1380`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1380) |
| 108 | matches complex glob patterns with braces | ported | [`crates/renovate-core/src/cache/package.rs:1396`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1396) |
| 124 | handles special characters in namespace patterns | pending | — |
| 143 | matches regex patterns | ported | [`crates/renovate-core/src/cache/package.rs:1409`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1409) |
| 161 | matches patterns with regex escape sequences | ported | [`crates/renovate-core/src/cache/package.rs:1454`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1454) |
| 179 | prioritizes exact match over glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1421`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1421) |
| 195 | returns longest matching pattern when multiple patterns apply | ported | [`crates/renovate-core/src/cache/package.rs:1438`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1438) |
| 209 | selects longest matching pattern across all configs | ported | [`crates/renovate-core/src/cache/package.rs:1466`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1466) |
| 228 | skips non-numeric values and selects next longest matching pattern | pending | — |
| 243 | returns undefined when no patterns match | pending | — |
| 256 | applies patterns consistently regardless of case in config order | ported | [`crates/renovate-core/src/cache/package.rs:1585`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1585) |
| 271 | handles empty string pattern | ported | [`crates/renovate-core/src/cache/package.rs:1612`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1612) |
| 286 | treats null and undefined values as invalid | pending | — |
| 306 | handles very large numbers | ported | [`crates/renovate-core/src/cache/package.rs:1493`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1493) |
| 318 | handles negative numbers | ported | [`crates/renovate-core/src/cache/package.rs:1483`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1483) |
| 330 | treats string numbers as invalid, only accepts number types | pending | — |
| 350 | returns default values when no overrides set | pending | — |
| 363 | uses override for softttlminutes when available | pending | — |
| 378 | applies custom cachehardttlminutes from config | pending | — |
| 391 | resolves ttl with glob pattern overrides | ported | [`crates/renovate-core/src/cache/package.rs:1503`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1503) |
| 407 | resolves ttl correctly with multiple overlapping overrides | ported | [`crates/renovate-core/src/cache/package.rs:1516`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1516) |
| 427 | uses maximum of softttlminutes and cachehardttlminutes for hardttlminutes | pending | — |
| 443 | handles negative cachehardttlminutes config | ported | [`crates/renovate-core/src/cache/package.rs:1534`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1534) |
| 461 | handles zero as valid override value | ported | [`crates/renovate-core/src/cache/package.rs:1548`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1548) |
| 477 | uses fallback when override is not a number | pending | — |

