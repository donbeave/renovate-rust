# `lib/config/decrypt.spec.ts`

[← `config/_root`](../../_by-module/config/_root.md) · [all modules](../../README.md)

**9/11 in-scope tests ported** (2 pending, 4 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 23 | returns empty with no privatekey | opt-out | basic identity/return-as-is when !config.encrypted (no privateKey involved); the top-level decryptConfig privateKey presence check + early return for no-encrypted case is part of high-level config secret handling in CLI parse/load (not unit exposed in core decrypt, which only has validate/get_azure/low-level bcpgp/openpgp); covered by other decrypt value tests or when high-level is wired. |
| 29 | warns if no privatekey found | opt-out | asserts TypeScript logger spy (logger.logger.once.warn called with the encryptedWarning text) + side effect that encrypted is cleared and unknown keys dropped; the spy + GlobalConfig + test setup for 'warn' path when encrypted present but no privateKey has no direct Rust equivalent (tracing, no 'once' spy harness). The core 'no privateKey + encrypted present -> clear/ignore' behavior may be covered when the high-level decryptConfig is wired in config load/CLI; left for impl or future if pure business emerges in core. |
| 41 | throws exception if encrypted found but no privatekey | opt-out | relies on process.env.RENOVATE_X_ENCRYPTED_STRICT + throw 'config-validation' when encrypted present but no privateKey (strict mode); env access + strict guard in decryptConfig is TS runtime/config-load specific (cf. other opted config env/strict/.js tests noting 'Never use unsafe in Rust' for env mutation); core encrypted value decrypt is in submodules (low-level); high-level strict not yet in Rust config load without broad changes or unsafe env. |
| 51 | throws exception if encrypted found but no privatekey- mend hosted | opt-out | same as sibling 'throws exception if encrypted found but no privateKey' but with process.env.MEND_HOSTED=true + RENOVATE_X_ENCRYPTED_STRICT; MEND hosted special case + env + the decryptConfig throw path is runtime/TS-specific secret handling (high-level in CLI/config load); core value handling ported in decrypt; env/strict not unit exposed without unsafe or broad. |
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

