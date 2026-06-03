# `lib/workers/global/index.spec.ts`

[← `worker/global`](../../../_by-module/worker/global.md) · [all modules](../../../README.md)

**2/15 ported** (13 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 56 | should generate correct toplevelorg/parentorg with multiple levels | ported | [`crates/renovate-core/src/util.rs:10122`](../../../../../../crates/renovate-core/src/util.rs#L10122) |
| 67 | should generate correct toplevelorg/parentorg with two levels | ported | [`crates/renovate-core/src/util.rs:10131`](../../../../../../crates/renovate-core/src/util.rs#L10131) |
| 78 | stores repositoryentryconfig for repositories[] object entries | pending | — |
| 91 | does not store repositoryentryconfig for repositories[] string entries | pending | — |
| 101 | handles config warnings and errors | pending | — |
| 114 | handles zero repos | pending | — |
| 125 | handles local | pending | — |
| 134 | processes repositories | pending | — |
| 152 | processes repositories break | pending | — |
| 173 | exits with non-zero when errors are logged | pending | — |
| 189 | exits with zero when warnings are logged | pending | — |
| 206 | does not log info message when log level is not info | pending | — |
| 220 | github | pending | — |
| 231 | gitlab | pending | — |
| 244 | successfully write file | pending | — |

