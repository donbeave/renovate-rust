# `lib/util/string-match.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**24/25 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 10 | returns false if empty patterns | ported | `crates/renovate-core/src/string_match.rs:248` |
| 14 | returns false if no match | ported | `crates/renovate-core/src/string_match.rs:254` |
| 18 | returns true if star | ported | `crates/renovate-core/src/string_match.rs:261` |
| 22 | returns true if any match | ported | `crates/renovate-core/src/string_match.rs:268` |
| 26 | returns true if one match with negative patterns | ported | `crates/renovate-core/src/string_match.rs:301` |
| 30 | returns true if every match with negative patterns | ported | `crates/renovate-core/src/string_match.rs:308` |
| 34 | returns true if matching positive and negative patterns | ported | `crates/renovate-core/src/string_match.rs:451` |
| 38 | returns true case insensitive for glob | ported | `crates/renovate-core/src/string_match.rs:426` |
| 42 | returns true if matching every negative pattern (regex) | ported | `crates/renovate-core/src/string_match.rs:467` |
| 48 | returns false if not matching every negative pattern (regex) | ported | `crates/renovate-core/src/string_match.rs:435` |
| 52 | returns true if matching every negative pattern (glob) | ported | `crates/renovate-core/src/string_match.rs:459` |
| 58 | returns false if not matching every negative pattern (glob) | ported | `crates/renovate-core/src/string_match.rs:443` |
| 64 | returns false if empty patterns | ported | `crates/renovate-core/src/string_match.rs:248` |
| 68 | returns false if empty inputs | ported | `crates/renovate-core/src/string_match.rs:483` |
| 72 | returns true if both empty | ported | `crates/renovate-core/src/string_match.rs:490` |
| 76 | returns true if any match with positive | ported | `crates/renovate-core/src/string_match.rs:496` |
| 80 | returns true if any match with negative | ported | `crates/renovate-core/src/string_match.rs:503` |
| 86 | allows valid regex pattern | ported | `crates/renovate-core/src/string_match.rs:513` |
| 90 | invalidates invalid regex pattern | ported | `crates/renovate-core/src/string_match.rs:519` |
| 94 | allows the i flag in regex pattern | ported | `crates/renovate-core/src/string_match.rs:525` |
| 98 | allows negative regex pattern | ported | `crates/renovate-core/src/string_match.rs:531` |
| 102 | does not allow non-regex input | ported | `crates/renovate-core/src/string_match.rs:539` |
| 108 | returns true if positive regex pattern matched | ported | `crates/renovate-core/src/string_match.rs:545` |
| 112 | returns true if negative regex is not matched | ported | `crates/renovate-core/src/string_match.rs:551` |
| 116 | returns false if negative pattern is matched | ported | `crates/renovate-core/src/string_match.rs:557` |

