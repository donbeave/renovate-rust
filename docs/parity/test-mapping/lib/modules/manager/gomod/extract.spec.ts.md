# `lib/modules/manager/gomod/extract.spec.ts`

[← `manager/gomod`](../../../../_by-module/manager/gomod.md) · [all modules](../../../../README.md)

**23/23 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 14 | returns null for empty | ported | `crates/renovate-core/src/extractors/gomod.rs:1286` |
| 18 | extracts single-line requires | ported | `crates/renovate-core/src/extractors/gomod.rs:1143` |
| 28 | extracts multi-line requires | ported | `crates/renovate-core/src/extractors/gomod.rs:1164` |
| 36 | ignores empty spaces in multi-line requires | ported | `crates/renovate-core/src/extractors/gomod.rs:1318` |
| 50 | extracts replace directives from multi-line and single line | ported | `crates/renovate-core/src/extractors/gomod.rs:1201` |
| 138 | extracts replace directives from non-public module path | ported | `crates/renovate-core/src/extractors/gomod.rs:2154` |
| 195 | ignores exclude directives from multi-line and single line | ported | `crates/renovate-core/src/extractors/gomod.rs:1213` |
| 214 | extracts the toolchain directive | ported | `crates/renovate-core/src/extractors/gomod.rs:1336` |
| 265 | extracts single-line tool directives | ported | `crates/renovate-core/src/extractors/gomod.rs:1394` |
| 284 | extracts multi-line tool directives | ported | `crates/renovate-core/src/extractors/gomod.rs:1411` |
| 306 | extracts tool directives with required modules | ported | `crates/renovate-core/src/extractors/gomod.rs:1426` |
| 325 | extracts tool directives of sub-modules | ported | `crates/renovate-core/src/extractors/gomod.rs:1437` |
| 372 | extracts tool directives with exact match | ported | `crates/renovate-core/src/extractors/gomod.rs:1484` |
| 391 | extracts tool directives with no matching dependencies | ported | `crates/renovate-core/src/extractors/gomod.rs:1497` |
| 404 | ignores directives unrelated to dependencies | ported | `crates/renovate-core/src/extractors/gomod.rs:1305` |
| 428 | marks placeholder pseudo versions with skipreason invalid-version | ported | `crates/renovate-core/src/extractors/gomod.rs:1192` |
| 530 | _(it.each / template — verify manually)_ | ? | — |
| 584 | ${constraint} is a valid constraint | ported | `crates/renovate-core/src/extractors/gomod.rs:2183` |
| 588 | matches version 1.19, even though it is not valid semver | ported | `crates/renovate-core/src/extractors/gomod.rs:2194` |
| 592 | matches the current semver minor | ported | `crates/renovate-core/src/extractors/gomod.rs:2200` |
| 597 | does not match the next semver minor | ported | `crates/renovate-core/src/extractors/gomod.rs:2207` |
| 602 | does not match the previous semver minor | ported | `crates/renovate-core/src/extractors/gomod.rs:2214` |
| 609 | handles undefined go directive | ported | `crates/renovate-core/src/extractors/gomod.rs:2221` |

