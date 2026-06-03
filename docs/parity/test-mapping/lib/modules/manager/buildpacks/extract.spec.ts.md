# `lib/modules/manager/buildpacks/extract.spec.ts`

[← `manager/buildpacks`](../../../../_by-module/manager/buildpacks.md) · [all modules](../../../../README.md)

**3/3 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | returns null for invalid files | ported | [`crates/renovate-core/src/extractors/buildpacks.rs:266`](../../../../../../../crates/renovate-core/src/extractors/buildpacks.rs#L266) |
| 11 | returns null for empty package.toml | ported | [`crates/renovate-core/src/extractors/buildpacks.rs:272`](../../../../../../../crates/renovate-core/src/extractors/buildpacks.rs#L272) |
| 20 | extracts builder and buildpack images | ported | [`crates/renovate-core/src/extractors/buildpacks.rs:221`](../../../../../../../crates/renovate-core/src/extractors/buildpacks.rs#L221) |

