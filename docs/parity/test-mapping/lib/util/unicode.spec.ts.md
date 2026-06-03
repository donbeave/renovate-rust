# `lib/util/unicode.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**4/5 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | logs a warning for hidden unicode characters in text files | ported | [`crates/renovate-core/src/util.rs:9925`](../../../../../crates/renovate-core/src/util.rs#L9925) |
| 16 | logs a trace message for bom character only | ported | [`crates/renovate-core/src/util.rs:9936`](../../../../../crates/renovate-core/src/util.rs#L9936) |
| 30 | does not log a warning for binary files with null bytes but no hidden unicode | ported | [`crates/renovate-core/src/util.rs:9945`](../../../../../crates/renovate-core/src/util.rs#L9945) |
| 43 | logs a trace message (not warning) for binary files with hidden unicode characters | pending | — |
| 63 | does not log a warning when no hidden characters are present | ported | [`crates/renovate-core/src/util.rs:9970`](../../../../../crates/renovate-core/src/util.rs#L9970) |

