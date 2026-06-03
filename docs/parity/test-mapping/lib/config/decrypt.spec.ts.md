# `lib/config/decrypt.spec.ts`

[← `config/_root`](../../_by-module/config/_root.md) · [all modules](../../README.md)

**9/15 ported** (6 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 23 | returns empty with no privatekey | pending | — |
| 29 | warns if no privatekey found | pending | — |
| 41 | throws exception if encrypted found but no privatekey | pending | — |
| 51 | throws exception if encrypted found but no privatekey- mend hosted | pending | — |
| 68 | _(it.each / template — verify manually)_ | ? | — |
| 93 | _(it.each / template — verify manually)_ | ? | — |
| 129 | _(it.each / template — verify manually)_ | ? | — |
| 164 | endpoint url invalid | ported | `crates/renovate-core/src/config/decrypt.rs:338` |
| 196 | endpoint url without collection | ported | `crates/renovate-core/src/config/decrypt.rs:378` |
| 235 | no pathname and url ends with slash | ported | `crates/renovate-core/src/config/decrypt.rs:418` |
| 243 | no pathname and no slash at end of url | ported | `crates/renovate-core/src/config/decrypt.rs:424` |
| 251 | pathname no slash at end | ported | `crates/renovate-core/src/config/decrypt.rs:430` |
| 259 | pathname with slash at end | ported | `crates/renovate-core/src/config/decrypt.rs:439` |
| 267 | pathname 2 levels no slash at end | ported | `crates/renovate-core/src/config/decrypt.rs:448` |
| 275 | pathname 2 levels with slash at end | ported | `crates/renovate-core/src/config/decrypt.rs:457` |

