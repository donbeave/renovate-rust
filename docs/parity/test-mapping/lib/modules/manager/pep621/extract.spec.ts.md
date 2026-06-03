# `lib/modules/manager/pep621/extract.spec.ts`

[← `manager/pep621`](../../../../_by-module/manager/pep621.md) · [all modules](../../../../README.md)

**14/14 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 16 | should return null for empty content | ported | [`crates/renovate-core/src/extractors/pep621.rs:1117`](../../../../../../../crates/renovate-core/src/extractors/pep621.rs#L1117) |
| 21 | should return null for invalid toml | ported | [`crates/renovate-core/src/extractors/pep621.rs:1124`](../../../../../../../crates/renovate-core/src/extractors/pep621.rs#L1124) |
| 32 | should return dependencies for valid content | ported | [`crates/renovate-core/src/extractors/pep621.rs:660`](../../../../../../../crates/renovate-core/src/extractors/pep621.rs#L660) |
| 233 | should return dependencies with overwritten pypi registryurl | ported | [`crates/renovate-core/src/extractors/pep621.rs:991`](../../../../../../../crates/renovate-core/src/extractors/pep621.rs#L991) |
| 309 | should return dependencies with original pypi registryurl | ported | [`crates/renovate-core/src/extractors/pep621.rs:963`](../../../../../../../crates/renovate-core/src/extractors/pep621.rs#L963) |
| 340 | should skip dependencies with unsupported uv sources | ported | [`crates/renovate-core/src/extractors/pep621.rs:787`](../../../../../../../crates/renovate-core/src/extractors/pep621.rs#L787) |
| 412 | should handle ssh git urls correctly for github sources | ported | [`crates/renovate-core/src/extractors/pep621.rs:860`](../../../../../../../crates/renovate-core/src/extractors/pep621.rs#L860) |
| 446 | should extract dependencies from hatch environments | ported | [`crates/renovate-core/src/extractors/pep621.rs:897`](../../../../../../../crates/renovate-core/src/extractors/pep621.rs#L897) |
| 498 | should extract project version | ported | [`crates/renovate-core/src/extractors/pep621.rs:1131`](../../../../../../../crates/renovate-core/src/extractors/pep621.rs#L1131) |
| 510 | should extract dependencies from build-system.requires | ported | [`crates/renovate-core/src/extractors/pep621.rs:1148`](../../../../../../../crates/renovate-core/src/extractors/pep621.rs#L1148) |
| 551 | should resolve lockedversions from pdm.lock | ported | [`crates/renovate-core/src/extractors/pep621.rs:1178`](../../../../../../../crates/renovate-core/src/extractors/pep621.rs#L1178) |
| 595 | should resolve lockedversions from uv.lock | ported | [`crates/renovate-core/src/extractors/pep621.rs:1218`](../../../../../../../crates/renovate-core/src/extractors/pep621.rs#L1218) |
| 661 | should resolve dependencies without locked versions on invalid uv.lock | ported | [`crates/renovate-core/src/extractors/pep621.rs:1254`](../../../../../../../crates/renovate-core/src/extractors/pep621.rs#L1254) |
| 694 | should resolve dependencies with template | ported | [`crates/renovate-core/src/extractors/pep621.rs:1270`](../../../../../../../crates/renovate-core/src/extractors/pep621.rs#L1270) |

