# `lib/modules/manager/homebrew/handlers/github.spec.ts`

[← `manager/homebrew`](../../../../../_by-module/manager/homebrew.md) · [all modules](../../../../../README.md)

**8/8 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 8 | returns null for empty string | ported | `crates/renovate-core/src/extractors/homebrew.rs:883` |
| 12 | _(it.each / template — verify manually)_ | ? | — |
| 19 | parses valid releases url | ported | `crates/renovate-core/src/extractors/homebrew.rs:889` |
| 33 | parses valid archive url | ported | `crates/renovate-core/src/extractors/homebrew.rs:902` |
| 49 | uses original version when semver.coerce fails | ported | `crates/renovate-core/src/extractors/homebrew.rs:915` |
| 66 | uses coerced version for filename when semver succeeds | ported | `crates/renovate-core/src/extractors/homebrew.rs:936` |
| 85 | creates dependency with github-releases datasource for releases url | ported | `crates/renovate-core/src/extractors/homebrew.rs:955` |
| 107 | creates dependency with github-tags datasource for archive url | ported | `crates/renovate-core/src/extractors/homebrew.rs:974` |

