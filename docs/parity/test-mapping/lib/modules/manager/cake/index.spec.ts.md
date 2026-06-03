# `lib/modules/manager/cake/index.spec.ts`

[← `manager/cake`](../../../../_by-module/manager/cake.md) · [all modules](../../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 21 | extracts | ported | [`crates/renovate-core/src/extractors/cake.rs:301`](../../../../../../../crates/renovate-core/src/extractors/cake.rs#L301) |
| 45 | extracts dotnet tools from single sdk style build file | ported | [`crates/renovate-core/src/extractors/cake.rs:364`](../../../../../../../crates/renovate-core/src/extractors/cake.rs#L364) |
| 101 | skips invalid entries in installtools | ported | [`crates/renovate-core/src/extractors/cake.rs:408`](../../../../../../../crates/renovate-core/src/extractors/cake.rs#L408) |
| 124 | calls applyregistries to honor nuget.config files if present for .cake files | ported | [`crates/renovate-core/src/extractors/cake.rs:429`](../../../../../../../crates/renovate-core/src/extractors/cake.rs#L429) |
| 141 | calls applyregistries to honor nuget.config files if present for installtools | ported | [`crates/renovate-core/src/extractors/cake.rs:453`](../../../../../../../crates/renovate-core/src/extractors/cake.rs#L453) |

