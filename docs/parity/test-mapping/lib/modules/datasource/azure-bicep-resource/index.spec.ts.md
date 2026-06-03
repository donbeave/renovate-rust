# `lib/modules/datasource/azure-bicep-resource/index.spec.ts`

[← `datasource/azure-bicep-resource`](../../../../_by-module/datasource/azure-bicep-resource.md) · [all modules](../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | should return null when no version is found | ported | [`crates/renovate-core/src/datasources/azure_bicep.rs:211`](../../../../../../../crates/renovate-core/src/datasources/azure_bicep.rs#L211) |
| 32 | should return null when package is a function | ported | [`crates/renovate-core/src/datasources/azure_bicep.rs:224`](../../../../../../../crates/renovate-core/src/datasources/azure_bicep.rs#L224) |
| 67 | should return versions when package is a resource | ported | [`crates/renovate-core/src/datasources/azure_bicep.rs:238`](../../../../../../../crates/renovate-core/src/datasources/azure_bicep.rs#L238) |
| 109 | should return versions when package is a resource and a function | ported | [`crates/renovate-core/src/datasources/azure_bicep.rs:266`](../../../../../../../crates/renovate-core/src/datasources/azure_bicep.rs#L266) |

