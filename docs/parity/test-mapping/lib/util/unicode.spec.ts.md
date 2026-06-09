# `lib/util/unicode.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | logs a warning for hidden unicode characters in text files | ported | [`crates/renovate-core/src/util.rs:11529`](../../../../../crates/renovate-core/src/util.rs#L11529) |
| 16 | logs a trace message for bom character only | ported | [`crates/renovate-core/src/util.rs:11540`](../../../../../crates/renovate-core/src/util.rs#L11540) |
| 30 | does not log a warning for binary files with null bytes but no hidden unicode | ported | [`crates/renovate-core/src/util.rs:11549`](../../../../../crates/renovate-core/src/util.rs#L11549) |
| 43 | logs a trace message (not warning) for binary files with hidden unicode characters | ported | [`crates/renovate-core/src/util.rs:11563`](../../../../../crates/renovate-core/src/util.rs#L11563) |
| 63 | does not log a warning when no hidden characters are present | ported | [`crates/renovate-core/src/util.rs:11576`](../../../../../crates/renovate-core/src/util.rs#L11576) |

