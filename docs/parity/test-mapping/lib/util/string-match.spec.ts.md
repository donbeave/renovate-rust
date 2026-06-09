# `lib/util/string-match.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**24/25 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns false if empty patterns | ported | [`crates/renovate-core/src/string_match.rs:257`](../../../../../crates/renovate-core/src/string_match.rs#L257) |
| 14 | returns false if no match | ported | [`crates/renovate-core/src/string_match.rs:263`](../../../../../crates/renovate-core/src/string_match.rs#L263) |
| 18 | returns true if star | ported | [`crates/renovate-core/src/string_match.rs:270`](../../../../../crates/renovate-core/src/string_match.rs#L270) |
| 22 | returns true if any match | ported | [`crates/renovate-core/src/string_match.rs:277`](../../../../../crates/renovate-core/src/string_match.rs#L277) |
| 26 | returns true if one match with negative patterns | ported | [`crates/renovate-core/src/string_match.rs:310`](../../../../../crates/renovate-core/src/string_match.rs#L310) |
| 30 | returns true if every match with negative patterns | ported | [`crates/renovate-core/src/string_match.rs:317`](../../../../../crates/renovate-core/src/string_match.rs#L317) |
| 34 | returns true if matching positive and negative patterns | ported | [`crates/renovate-core/src/string_match.rs:460`](../../../../../crates/renovate-core/src/string_match.rs#L460) |
| 38 | returns true case insensitive for glob | ported | [`crates/renovate-core/src/string_match.rs:435`](../../../../../crates/renovate-core/src/string_match.rs#L435) |
| 42 | returns true if matching every negative pattern (regex) | ported | [`crates/renovate-core/src/string_match.rs:476`](../../../../../crates/renovate-core/src/string_match.rs#L476) |
| 48 | returns false if not matching every negative pattern (regex) | ported | [`crates/renovate-core/src/string_match.rs:444`](../../../../../crates/renovate-core/src/string_match.rs#L444) |
| 52 | returns true if matching every negative pattern (glob) | ported | [`crates/renovate-core/src/string_match.rs:468`](../../../../../crates/renovate-core/src/string_match.rs#L468) |
| 58 | returns false if not matching every negative pattern (glob) | ported | [`crates/renovate-core/src/string_match.rs:452`](../../../../../crates/renovate-core/src/string_match.rs#L452) |
| 64 | returns false if empty patterns | ported | [`crates/renovate-core/src/string_match.rs:257`](../../../../../crates/renovate-core/src/string_match.rs#L257) |
| 68 | returns false if empty inputs | ported | [`crates/renovate-core/src/string_match.rs:492`](../../../../../crates/renovate-core/src/string_match.rs#L492) |
| 72 | returns true if both empty | ported | [`crates/renovate-core/src/string_match.rs:499`](../../../../../crates/renovate-core/src/string_match.rs#L499) |
| 76 | returns true if any match with positive | ported | [`crates/renovate-core/src/string_match.rs:505`](../../../../../crates/renovate-core/src/string_match.rs#L505) |
| 80 | returns true if any match with negative | ported | [`crates/renovate-core/src/string_match.rs:512`](../../../../../crates/renovate-core/src/string_match.rs#L512) |
| 86 | allows valid regex pattern | ported | [`crates/renovate-core/src/string_match.rs:522`](../../../../../crates/renovate-core/src/string_match.rs#L522) |
| 90 | invalidates invalid regex pattern | ported | [`crates/renovate-core/src/string_match.rs:528`](../../../../../crates/renovate-core/src/string_match.rs#L528) |
| 94 | allows the i flag in regex pattern | ported | [`crates/renovate-core/src/string_match.rs:534`](../../../../../crates/renovate-core/src/string_match.rs#L534) |
| 98 | allows negative regex pattern | ported | [`crates/renovate-core/src/string_match.rs:540`](../../../../../crates/renovate-core/src/string_match.rs#L540) |
| 102 | does not allow non-regex input | ported | [`crates/renovate-core/src/string_match.rs:548`](../../../../../crates/renovate-core/src/string_match.rs#L548) |
| 108 | returns true if positive regex pattern matched | ported | [`crates/renovate-core/src/string_match.rs:554`](../../../../../crates/renovate-core/src/string_match.rs#L554) |
| 112 | returns true if negative regex is not matched | ported | [`crates/renovate-core/src/string_match.rs:560`](../../../../../crates/renovate-core/src/string_match.rs#L560) |
| 116 | returns false if negative pattern is matched | ported | [`crates/renovate-core/src/string_match.rs:566`](../../../../../crates/renovate-core/src/string_match.rs#L566) |

