# `lib/modules/manager/homebrew/handlers/npm.spec.ts`

[← `manager/homebrew`](../../../../../_by-module/manager/homebrew.md) · [all modules](../../../../../README.md)

**15/15 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | returns null for empty string | ported | `crates/renovate-core/src/extractors/homebrew.rs:996` |
| 12 | _(it.each / template — verify manually)_ | ? | — |
| 19 | returns null for non-npm registry url | ported | `crates/renovate-core/src/extractors/homebrew.rs:1002` |
| 25 | returns null for custom npm registry | ported | `crates/renovate-core/src/extractors/homebrew.rs:1008` |
| 33 | parses scoped package url | ported | `crates/renovate-core/src/extractors/homebrew.rs:1016` |
| 45 | parses unscoped package url | ported | `crates/renovate-core/src/extractors/homebrew.rs:1027` |
| 57 | parses version with prerelease | ported | `crates/renovate-core/src/extractors/homebrew.rs:1036` |
| 69 | parses version with build metadata | ported | `crates/renovate-core/src/extractors/homebrew.rs:1045` |
| 81 | returns null for malformed url | ported | `crates/renovate-core/src/extractors/homebrew.rs:1055` |
| 89 | creates dependency with npm datasource for scoped package | ported | `crates/renovate-core/src/extractors/homebrew.rs:1061` |
| 116 | creates dependency with npm datasource for unscoped package | ported | `crates/renovate-core/src/extractors/homebrew.rs:1084` |
| 145 | builds url for scoped package | ported | `crates/renovate-core/src/extractors/homebrew.rs:1101` |
| 160 | builds url for unscoped package | ported | `crates/renovate-core/src/extractors/homebrew.rs:1119` |
| 175 | builds url with prerelease version | ported | `crates/renovate-core/src/extractors/homebrew.rs:1134` |
| 190 | builds url for deeply scoped package | ported | `crates/renovate-core/src/extractors/homebrew.rs:1149` |

