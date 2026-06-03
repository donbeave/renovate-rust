# `lib/modules/manager/bicep/extract.spec.ts`

[← `manager/bicep`](../../../../_by-module/manager/bicep.md) · [all modules](../../../../README.md)

**9/9 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | should extract a normal resource | ported | [`crates/renovate-core/src/extractors/bicep.rs:61`](../../../../../../../crates/renovate-core/src/extractors/bicep.rs#L61) |
| 37 | should not extract a commented out resource | ported | [`crates/renovate-core/src/extractors/bicep.rs:91`](../../../../../../../crates/renovate-core/src/extractors/bicep.rs#L91) |
| 58 | should extract a conditional resource | ported | [`crates/renovate-core/src/extractors/bicep.rs:119`](../../../../../../../crates/renovate-core/src/extractors/bicep.rs#L119) |
| 90 | should extract a existing resource | ported | [`crates/renovate-core/src/extractors/bicep.rs:129`](../../../../../../../crates/renovate-core/src/extractors/bicep.rs#L129) |
| 117 | should extract a conditional loop resource | ported | [`crates/renovate-core/src/extractors/bicep.rs:148`](../../../../../../../crates/renovate-core/src/extractors/bicep.rs#L148) |
| 149 | should extract a loop resource | ported | [`crates/renovate-core/src/extractors/bicep.rs:138`](../../../../../../../crates/renovate-core/src/extractors/bicep.rs#L138) |
| 181 | should not extract a nested unversioned resource | ported | [`crates/renovate-core/src/extractors/bicep.rs:160`](../../../../../../../crates/renovate-core/src/extractors/bicep.rs#L160) |
| 217 | should not extract a nested versioned resource | ported | [`crates/renovate-core/src/extractors/bicep.rs:168`](../../../../../../../crates/renovate-core/src/extractors/bicep.rs#L168) |
| 253 | should extract a sub resource | ported | [`crates/renovate-core/src/extractors/bicep.rs:176`](../../../../../../../crates/renovate-core/src/extractors/bicep.rs#L176) |

