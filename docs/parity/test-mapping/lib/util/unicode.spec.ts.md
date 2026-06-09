# `lib/util/unicode.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | logs a warning for hidden unicode characters in text files | ported | [`crates/renovate-core/src/util.rs:11528`](../../../../../crates/renovate-core/src/util.rs#L11528) |
| 16 | logs a trace message for bom character only | ported | [`crates/renovate-core/src/util.rs:11539`](../../../../../crates/renovate-core/src/util.rs#L11539) |
| 30 | does not log a warning for binary files with null bytes but no hidden unicode | ported | [`crates/renovate-core/src/util.rs:11548`](../../../../../crates/renovate-core/src/util.rs#L11548) |
| 43 | logs a trace message (not warning) for binary files with hidden unicode characters | ported | [`crates/renovate-core/src/util.rs:11562`](../../../../../crates/renovate-core/src/util.rs#L11562) |
| 63 | does not log a warning when no hidden characters are present | ported | [`crates/renovate-core/src/util.rs:11575`](../../../../../crates/renovate-core/src/util.rs#L11575) |

