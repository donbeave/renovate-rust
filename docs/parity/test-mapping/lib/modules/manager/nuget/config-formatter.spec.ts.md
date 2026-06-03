# `lib/modules/manager/nuget/config-formatter.spec.ts`

[← `manager/nuget`](../../../../_by-module/manager/nuget.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | returns xml with registries | ported | [`crates/renovate-core/src/extractors/nuget.rs:2471`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2471) |
| 58 | returns xml with authenticated registries | ported | [`crates/renovate-core/src/extractors/nuget.rs:2579`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2579) |
| 138 | escapes registry credential names containing special characters | ported | [`crates/renovate-core/src/extractors/nuget.rs:2602`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2602) |
| 181 | strips protocol version from feed url | ported | [`crates/renovate-core/src/extractors/nuget.rs:2501`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2501) |
| 202 | includes packagesourcemapping when defined | ported | [`crates/renovate-core/src/extractors/nuget.rs:2540`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2540) |
| 245 | excludes packagesourcemapping when undefined | ported | [`crates/renovate-core/src/extractors/nuget.rs:2567`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2567) |
| 265 | skips duplicate registry urls | ported | [`crates/renovate-core/src/extractors/nuget.rs:2514`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2514) |

