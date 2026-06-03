# `lib/modules/datasource/hermit/index.spec.ts`

[← `datasource/hermit`](../../../../_by-module/datasource/hermit.md) · [all modules](../../../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 14 | should return result from hermit list | ported | [`crates/renovate-core/src/datasources/hermit.rs:234`](../../../../../../../crates/renovate-core/src/datasources/hermit.rs#L234) |
| 79 | should fail on no result found | ported | [`crates/renovate-core/src/datasources/hermit.rs:280`](../../../../../../../crates/renovate-core/src/datasources/hermit.rs#L280) |
| 106 | should fail on network error | ported | [`crates/renovate-core/src/datasources/hermit.rs:309`](../../../../../../../crates/renovate-core/src/datasources/hermit.rs#L309) |
| 133 | should get null result on non github url given | ported | [`crates/renovate-core/src/datasources/hermit.rs:337`](../../../../../../../crates/renovate-core/src/datasources/hermit.rs#L337) |
| 142 | should get null result on missing repo or owner | ported | [`crates/renovate-core/src/datasources/hermit.rs:352`](../../../../../../../crates/renovate-core/src/datasources/hermit.rs#L352) |
| 157 | should get null for extra path provided in registry url | ported | [`crates/renovate-core/src/datasources/hermit.rs:366`](../../../../../../../crates/renovate-core/src/datasources/hermit.rs#L366) |
| 166 | should get null result on empty registryurl | ported | [`crates/renovate-core/src/datasources/hermit.rs:381`](../../../../../../../crates/renovate-core/src/datasources/hermit.rs#L381) |
| 174 | should fail on missing index.json asset | ported | [`crates/renovate-core/src/datasources/hermit.rs:389`](../../../../../../../crates/renovate-core/src/datasources/hermit.rs#L389) |
| 195 | should get null on invalid index.json asset | ported | [`crates/renovate-core/src/datasources/hermit.rs:414`](../../../../../../../crates/renovate-core/src/datasources/hermit.rs#L414) |
| 221 | should get null on invalid registry url | ported | [`crates/renovate-core/src/datasources/hermit.rs:443`](../../../../../../../crates/renovate-core/src/datasources/hermit.rs#L443) |

