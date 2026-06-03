# `lib/modules/manager/nuget/package-tree.spec.ts`

[← `manager/nuget`](../../../../_by-module/manager/nuget.md) · [all modules](../../../../README.md)

**11/11 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 32 | returns self for single project | ported | [`crates/renovate-core/src/extractors/nuget.rs:2842`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2842) |
| 45 | returns self for two projects with no references | ported | [`crates/renovate-core/src/extractors/nuget.rs:2864`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2864) |
| 60 | returns projects for two projects with one reference | ported | [`crates/renovate-core/src/extractors/nuget.rs:2893`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2893) |
| 77 | returns project for two projects with one reference and central versions | ported | [`crates/renovate-core/src/extractors/nuget.rs:2918`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2918) |
| 99 | returns projects for two projects with one reference and directory.build.props | ported | [`crates/renovate-core/src/extractors/nuget.rs:2948`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2948) |
| 121 | returns only projects under nested directory.build.props directory | ported | [`crates/renovate-core/src/extractors/nuget.rs:2978`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L2978) |
| 143 | returns project for two projects with one reference and global.json | ported | [`crates/renovate-core/src/extractors/nuget.rs:3005`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L3005) |
| 163 | returns projects for three projects with two linear references | ported | [`crates/renovate-core/src/extractors/nuget.rs:3030`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L3030) |
| 197 | returns projects for three projects with two tree-like references | ported | [`crates/renovate-core/src/extractors/nuget.rs:3098`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L3098) |
| 229 | throws error on circular reference | ported | [`crates/renovate-core/src/extractors/nuget.rs:3160`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L3160) |
| 245 | skips on invalid xml file | ported | [`crates/renovate-core/src/extractors/nuget.rs:3173`](../../../../../../../crates/renovate-core/src/extractors/nuget.rs#L3173) |

