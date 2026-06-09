# `lib/util/cache/package/ttl.spec.ts`

[← `util/cache`](../../../../_by-module/util/cache.md) · [all modules](../../../../README.md)

**18/31 in-scope tests ported** (13 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | returns undefined when no cachettloverride config exists | ported | [`crates/renovate-core/src/cache/package.rs:1345`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1345) |
| 20 | returns undefined when cachettloverride is empty | pending | — |
| 30 | returns exact match when namespace exists in config | ported | [`crates/renovate-core/src/cache/package.rs:1353`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1353) |
| 45 | returns undefined when exact match is not a number | pending | — |
| 58 | returns undefined when no matching namespace found | pending | — |
| 72 | matches simple glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1365`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1365) |
| 88 | matches wildcard pattern for all namespaces | ported | [`crates/renovate-core/src/cache/package.rs:1378`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1378) |
| 108 | matches complex glob patterns with braces | ported | [`crates/renovate-core/src/cache/package.rs:1394`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1394) |
| 124 | handles special characters in namespace patterns | pending | — |
| 143 | matches regex patterns | ported | [`crates/renovate-core/src/cache/package.rs:1407`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1407) |
| 161 | matches patterns with regex escape sequences | ported | [`crates/renovate-core/src/cache/package.rs:1452`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1452) |
| 179 | prioritizes exact match over glob patterns | ported | [`crates/renovate-core/src/cache/package.rs:1419`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1419) |
| 195 | returns longest matching pattern when multiple patterns apply | ported | [`crates/renovate-core/src/cache/package.rs:1436`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1436) |
| 209 | selects longest matching pattern across all configs | ported | [`crates/renovate-core/src/cache/package.rs:1464`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1464) |
| 228 | skips non-numeric values and selects next longest matching pattern | pending | — |
| 243 | returns undefined when no patterns match | pending | — |
| 256 | applies patterns consistently regardless of case in config order | ported | [`crates/renovate-core/src/cache/package.rs:1583`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1583) |
| 271 | handles empty string pattern | ported | [`crates/renovate-core/src/cache/package.rs:1610`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1610) |
| 286 | treats null and undefined values as invalid | pending | — |
| 306 | handles very large numbers | ported | [`crates/renovate-core/src/cache/package.rs:1491`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1491) |
| 318 | handles negative numbers | ported | [`crates/renovate-core/src/cache/package.rs:1481`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1481) |
| 330 | treats string numbers as invalid, only accepts number types | pending | — |
| 350 | returns default values when no overrides set | pending | — |
| 363 | uses override for softttlminutes when available | pending | — |
| 378 | applies custom cachehardttlminutes from config | pending | — |
| 391 | resolves ttl with glob pattern overrides | ported | [`crates/renovate-core/src/cache/package.rs:1501`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1501) |
| 407 | resolves ttl correctly with multiple overlapping overrides | ported | [`crates/renovate-core/src/cache/package.rs:1514`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1514) |
| 427 | uses maximum of softttlminutes and cachehardttlminutes for hardttlminutes | pending | — |
| 443 | handles negative cachehardttlminutes config | ported | [`crates/renovate-core/src/cache/package.rs:1532`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1532) |
| 461 | handles zero as valid override value | ported | [`crates/renovate-core/src/cache/package.rs:1546`](../../../../../../../crates/renovate-core/src/cache/package.rs#L1546) |
| 477 | uses fallback when override is not a number | pending | — |

