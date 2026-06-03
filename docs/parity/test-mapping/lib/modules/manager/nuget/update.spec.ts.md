# `lib/modules/manager/nuget/update.spec.ts`

[← `manager/nuget`](../../../../_by-module/manager/nuget.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 17 | bumps csproj version | ported | [`crates/renovate-core/src/extractors/nuget.rs:2401`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2401) |
| 28 | does not bump version twice | ported | [`crates/renovate-core/src/extractors/nuget.rs:2408`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2408) |
| 43 | issue 23526 does not bump version incorrectly | ported | [`crates/renovate-core/src/extractors/nuget.rs:2416`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2416) |
| 58 | does not bump version if version is not a semantic version | ported | [`crates/renovate-core/src/extractors/nuget.rs:2424`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2424) |
| 69 | does not bump version if extract found no version | ported | [`crates/renovate-core/src/extractors/nuget.rs:2432`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2432) |
| 75 | does not bump version if csproj has no version | ported | [`crates/renovate-core/src/extractors/nuget.rs:2439`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2439) |
| 87 | returns content if bumping errors | ported | [`crates/renovate-core/src/extractors/nuget.rs:2447`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2447) |
| 96 | bumps csproj version with prerelease semver level | ported | [`crates/renovate-core/src/extractors/nuget.rs:2454`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2454) |
| 107 | bumps csproj version prefix | ported | [`crates/renovate-core/src/extractors/nuget.rs:2461`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2461) |

