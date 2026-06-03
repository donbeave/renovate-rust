# `lib/modules/datasource/azure-bicep-resource/index.spec.ts`

[← `datasource/azure-bicep-resource`](../../../../_by-module/datasource/azure-bicep-resource.md) · [all modules](../../../../README.md)

**4/4 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 10 | should return null when no version is found | ported | `crates/renovate-core/src/datasources/azure_bicep.rs:211` |
| 32 | should return null when package is a function | ported | `crates/renovate-core/src/datasources/azure_bicep.rs:224` |
| 67 | should return versions when package is a resource | ported | `crates/renovate-core/src/datasources/azure_bicep.rs:238` |
| 109 | should return versions when package is a resource and a function | ported | `crates/renovate-core/src/datasources/azure_bicep.rs:266` |

