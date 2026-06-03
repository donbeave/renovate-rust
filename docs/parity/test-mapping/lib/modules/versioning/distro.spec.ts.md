# `lib/modules/versioning/distro.spec.ts`

[← `versioning/_common`](../../../_by-module/versioning/_common.md) · [all modules](../../../README.md)

**15/15 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 12 | _(it.each / template — verify manually)_ | ? | — |
| 27 | _(it.each / template — verify manually)_ | ? | — |
| 44 | _(it.each / template — verify manually)_ | ? | — |
| 61 | _(it.each / template — verify manually)_ | ? | — |
| 80 | _(it.each / template — verify manually)_ | ? | — |
| 98 | _(it.each / template — verify manually)_ | ? | — |
| 115 | retrieves schedule of the previous previous release | ported | `crates/renovate-core/src/versioning/ubuntu.rs:1151` |
| 122 | retrieves schedule of the previous release | ported | `crates/renovate-core/src/versioning/ubuntu.rs:1152` |
| 129 | retrieves schedule of the most recent release | ported | `crates/renovate-core/src/versioning/ubuntu.rs:1153` |
| 136 | sends a float as an argument | ported | `crates/renovate-core/src/versioning/ubuntu.rs:1154` |
| 143 | sends an out of bound argument | ported | `crates/renovate-core/src/versioning/ubuntu.rs:1155` |
| 147 | sends another out of bound argument | ported | `crates/renovate-core/src/versioning/ubuntu.rs:1156` |
| 151 | retrieves focal release schedule | ported | `crates/renovate-core/src/versioning/ubuntu.rs:1097` |
| 158 | retrieves non-existent release schedule | ported | `crates/renovate-core/src/versioning/ubuntu.rs:1098` |
| 162 | works with debian | ported | `crates/renovate-core/src/versioning/ubuntu.rs:1224` |

