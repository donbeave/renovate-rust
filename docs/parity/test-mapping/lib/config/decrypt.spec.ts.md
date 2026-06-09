# `lib/config/decrypt.spec.ts`

[← `config/_root`](../../_by-module/config/_root.md) · [all modules](../../README.md)

**9/14 in-scope tests ported** (5 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 23 | returns empty with no privatekey | pending | — |
| 29 | warns if no privatekey found | opt-out | asserts TypeScript logger spy (logger.logger.once.warn called with the encryptedWarning text) + side effect that encrypted is cleared and unknown keys dropped; the spy + GlobalConfig + test setup for 'warn' path when encrypted present but no privateKey has no direct Rust equivalent (tracing, no 'once' spy harness). The core 'no privateKey + encrypted present -> clear/ignore' behavior may be covered when the high-level decryptConfig is wired in config load/CLI; left for impl or future if pure business emerges in core. |
| 41 | throws exception if encrypted found but no privatekey | pending | — |
| 51 | throws exception if encrypted found but no privatekey- mend hosted | pending | — |
| 68 | _(it.each / template — verify manually)_ | ? | — |
| 93 | _(it.each / template — verify manually)_ | ? | — |
| 129 | _(it.each / template — verify manually)_ | ? | — |
| 164 | endpoint url invalid | ported | [`crates/renovate-core/src/config/decrypt.rs:338`](../../../../../crates/renovate-core/src/config/decrypt.rs#L338) |
| 196 | endpoint url without collection | ported | [`crates/renovate-core/src/config/decrypt.rs:378`](../../../../../crates/renovate-core/src/config/decrypt.rs#L378) |
| 235 | no pathname and url ends with slash | ported | [`crates/renovate-core/src/config/decrypt.rs:418`](../../../../../crates/renovate-core/src/config/decrypt.rs#L418) |
| 243 | no pathname and no slash at end of url | ported | [`crates/renovate-core/src/config/decrypt.rs:424`](../../../../../crates/renovate-core/src/config/decrypt.rs#L424) |
| 251 | pathname no slash at end | ported | [`crates/renovate-core/src/config/decrypt.rs:430`](../../../../../crates/renovate-core/src/config/decrypt.rs#L430) |
| 259 | pathname with slash at end | ported | [`crates/renovate-core/src/config/decrypt.rs:439`](../../../../../crates/renovate-core/src/config/decrypt.rs#L439) |
| 267 | pathname 2 levels no slash at end | ported | [`crates/renovate-core/src/config/decrypt.rs:448`](../../../../../crates/renovate-core/src/config/decrypt.rs#L448) |
| 275 | pathname 2 levels with slash at end | ported | [`crates/renovate-core/src/config/decrypt.rs:457`](../../../../../crates/renovate-core/src/config/decrypt.rs#L457) |

