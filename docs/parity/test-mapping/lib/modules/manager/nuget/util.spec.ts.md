# `lib/modules/manager/nuget/util.spec.ts`

[← `manager/nuget`](../../../../_by-module/manager/nuget.md) · [all modules](../../../../README.md)

**18/18 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 17 | finds the version in a later property group | ported | [`crates/renovate-core/src/extractors/nuget.rs:2645`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2645) |
| 28 | picks version over versionprefix | ported | [`crates/renovate-core/src/extractors/nuget.rs:2653`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2653) |
| 41 | reads nuget config file | ported | [`crates/renovate-core/src/extractors/nuget.rs:2661`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2661) |
| 78 | deduplicates registries | ported | [`crates/renovate-core/src/extractors/nuget.rs:2671`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2671) |
| 99 | reads nuget config file with default registry | ported | [`crates/renovate-core/src/extractors/nuget.rs:2683`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2683) |
| 134 | reads nuget config file with default registry disabled and added sources | ported | [`crates/renovate-core/src/extractors/nuget.rs:2693`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2693) |
| 157 | reads nuget config file with default registry disabled given default registry added | ported | [`crates/renovate-core/src/extractors/nuget.rs:2702`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2702) |
| 181 | reads nuget config file with unknown disabled source | ported | [`crates/renovate-core/src/extractors/nuget.rs:2711`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2711) |
| 208 | reads nuget config file with disabled source with value false | ported | [`crates/renovate-core/src/extractors/nuget.rs:2719`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2719) |
| 237 | reads nuget config file without packagesources and ignores disabledpackagesources | ported | [`crates/renovate-core/src/extractors/nuget.rs:2727`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2727) |
| 254 | applies registry to package name via source mapping | ported | [`crates/renovate-core/src/extractors/nuget.rs:2735`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2735) |
| 323 | applies registry to package name case insensitive | ported | [`crates/renovate-core/src/extractors/nuget.rs:2754`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2754) |
| 343 | applies all registries to package name | ported | [`crates/renovate-core/src/extractors/nuget.rs:2773`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2773) |
| 371 | applies nothing | ported | [`crates/renovate-core/src/extractors/nuget.rs:2802`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2802) |
| 386 | not found | ported | [`crates/renovate-core/src/extractors/nuget.rs:2813`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2813) |
| 392 | no content | ported | [`crates/renovate-core/src/extractors/nuget.rs:2819`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2819) |
| 398 | fails to parse | ported | [`crates/renovate-core/src/extractors/nuget.rs:2825`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2825) |
| 405 | parses | ported | [`crates/renovate-core/src/extractors/nuget.rs:2831`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2831) |

