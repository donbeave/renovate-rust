# `lib/modules/manager/nuget/config-formatter.spec.ts`

[← `manager/nuget`](../../../../_by-module/manager/nuget.md) · [all modules](../../../../README.md)

**7/7 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 12 | returns xml with registries | ported | `crates/renovate-core/src/extractors/nuget.rs:2471` |
| 58 | returns xml with authenticated registries | ported | `crates/renovate-core/src/extractors/nuget.rs:2579` |
| 138 | escapes registry credential names containing special characters | ported | `crates/renovate-core/src/extractors/nuget.rs:2602` |
| 181 | strips protocol version from feed url | ported | `crates/renovate-core/src/extractors/nuget.rs:2501` |
| 202 | includes packagesourcemapping when defined | ported | `crates/renovate-core/src/extractors/nuget.rs:2540` |
| 245 | excludes packagesourcemapping when undefined | ported | `crates/renovate-core/src/extractors/nuget.rs:2567` |
| 265 | skips duplicate registry urls | ported | `crates/renovate-core/src/extractors/nuget.rs:2514` |

