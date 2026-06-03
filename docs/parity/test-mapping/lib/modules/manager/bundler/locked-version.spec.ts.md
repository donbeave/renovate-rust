# `lib/modules/manager/bundler/locked-version.spec.ts`

[← `manager/bundler`](../../../../_by-module/manager/bundler.md) · [all modules](../../../../README.md)

**12/12 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 13 | parse rails gem lock file | ported | `crates/renovate-core/src/extractors/bundler.rs:1036` |
| 19 | parse webpacker gem lock file | ported | `crates/renovate-core/src/extractors/bundler.rs:1044` |
| 25 | parse mastodon gem lock file | ported | `crates/renovate-core/src/extractors/bundler.rs:1051` |
| 31 | parse ruby ci gem lock file | ported | `crates/renovate-core/src/extractors/bundler.rs:1058` |
| 37 | parse gitlab foss gem lock file | ported | `crates/renovate-core/src/extractors/bundler.rs:1065` |
| 43 | returns empty map for empty string | ported | `crates/renovate-core/src/extractors/bundler.rs:1072` |
| 48 | returns empty map when errors occur | ported | `crates/renovate-core/src/extractors/bundler.rs:1078` |
| 54 | strips platform suffixes from dependencies | ported | `crates/renovate-core/src/extractors/bundler.rs:1084` |
| 84 | extracts simple versions from parentheses | ported | `crates/renovate-core/src/extractors/bundler.rs:1094` |
| 98 | extracts complex version formats from parentheses | ported | `crates/renovate-core/src/extractors/bundler.rs:1103` |
| 114 | correctly extracts gem names when versions contain special characters | ported | `crates/renovate-core/src/extractors/bundler.rs:1119` |
| 130 | handles gems with platform-specific versions | ported | `crates/renovate-core/src/extractors/bundler.rs:1132` |

