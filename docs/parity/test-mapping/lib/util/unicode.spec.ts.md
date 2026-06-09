# `lib/util/unicode.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | logs a warning for hidden unicode characters in text files | ported | [`crates/renovate-core/src/util.rs:11625`](../../../../../crates/renovate-core/src/util.rs#L11625) |
| 16 | logs a trace message for bom character only | ported | [`crates/renovate-core/src/util.rs:11636`](../../../../../crates/renovate-core/src/util.rs#L11636) |
| 30 | does not log a warning for binary files with null bytes but no hidden unicode | ported | [`crates/renovate-core/src/util.rs:11645`](../../../../../crates/renovate-core/src/util.rs#L11645) |
| 43 | logs a trace message (not warning) for binary files with hidden unicode characters | ported | [`crates/renovate-core/src/util.rs:11659`](../../../../../crates/renovate-core/src/util.rs#L11659) |
| 63 | does not log a warning when no hidden characters are present | ported | [`crates/renovate-core/src/util.rs:11672`](../../../../../crates/renovate-core/src/util.rs#L11672) |

