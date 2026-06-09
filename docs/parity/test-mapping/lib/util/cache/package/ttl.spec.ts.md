# `lib/util/cache/package/ttl.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**18/31 in-scope tests ported** (13 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | returns undefined when no cachettloverride config exists | ported | [`crates/renovate-core/src/cache/package.rs:1340`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1340) |
| 20 | returns undefined when cachettloverride is empty | pending | — |
| 30 | returns exact match when namespace exists in config | ported | [`crates/renovate-core/src/cache/package.rs:1348`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1348) |
| 45 | returns undefined when exact match is not a number | pending | — |
| 58 | returns undefined when no matching namespace found | pending | — |
| 72 | matches simple glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1360`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1360) |
| 88 | matches wildcard pattern for all namespaces | ported | [`crates/renovate-core/src/cache/package.rs:1373`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1373) |
| 108 | matches complex glob patterns with braces | ported | [`crates/renovate-core/src/cache/package.rs:1389`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1389) |
| 124 | handles special characters in namespace patterns | pending | — |
| 143 | matches regex patterns | ported | [`crates/renovate-core/src/cache/package.rs:1402`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1402) |
| 161 | matches patterns with regex escape sequences | ported | [`crates/renovate-core/src/cache/package.rs:1447`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1447) |
| 179 | prioritizes exact match over glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1414`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1414) |
| 195 | returns longest matching pattern when multiple patterns apply | ported | [`crates/renovate-core/src/cache/package.rs:1431`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1431) |
| 209 | selects longest matching pattern across all configs | ported | [`crates/renovate-core/src/cache/package.rs:1459`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1459) |
| 228 | skips non-numeric values and selects next longest matching pattern | pending | — |
| 243 | returns undefined when no patterns match | pending | — |
| 256 | applies patterns consistently regardless of case in config order | ported | [`crates/renovate-core/src/cache/package.rs:1578`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1578) |
| 271 | handles empty string pattern | ported | [`crates/renovate-core/src/cache/package.rs:1605`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1605) |
| 286 | treats null and undefined values as invalid | pending | — |
| 306 | handles very large numbers | ported | [`crates/renovate-core/src/cache/package.rs:1486`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1486) |
| 318 | handles negative numbers | ported | [`crates/renovate-core/src/cache/package.rs:1476`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1476) |
| 330 | treats string numbers as invalid, only accepts number types | pending | — |
| 350 | returns default values when no overrides set | pending | — |
| 363 | uses override for softttlminutes when available | pending | — |
| 378 | applies custom cachehardttlminutes from config | pending | — |
| 391 | resolves ttl with glob pattern overrides | ported | [`crates/renovate-core/src/cache/package.rs:1496`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1496) |
| 407 | resolves ttl correctly with multiple overlapping overrides | ported | [`crates/renovate-core/src/cache/package.rs:1509`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1509) |
| 427 | uses maximum of softttlminutes and cachehardttlminutes for hardttlminutes | pending | — |
| 443 | handles negative cachehardttlminutes config | ported | [`crates/renovate-core/src/cache/package.rs:1527`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1527) |
| 461 | handles zero as valid override value | ported | [`crates/renovate-core/src/cache/package.rs:1541`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1541) |
| 477 | uses fallback when override is not a number | pending | — |

