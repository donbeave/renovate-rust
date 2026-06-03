# `lib/modules/datasource/bitrise/index.spec.ts`

[← `datasource/bitrise`](../../../../_by-module/datasource/bitrise.md) · [all modules](../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns null for unsupported registryurl | ported | [`crates/renovate-core/src/datasources/bitrise.rs:270`](../../../../../../../crates/renovate-core/src/datasources/bitrise.rs#L270) |
| 19 | support github enterprise api url | ported | [`crates/renovate-core/src/datasources/bitrise.rs:284`](../../../../../../../crates/renovate-core/src/datasources/bitrise.rs#L284) |
| 63 | returns version and filters out the asset folder | ported | [`crates/renovate-core/src/datasources/bitrise.rs:329`](../../../../../../../crates/renovate-core/src/datasources/bitrise.rs#L329) |
| 137 | returns null if there are no releases | ported | [`crates/renovate-core/src/datasources/bitrise.rs:381`](../../../../../../../crates/renovate-core/src/datasources/bitrise.rs#L381) |
| 159 | returns null if the package has an unexpected format | ported | [`crates/renovate-core/src/datasources/bitrise.rs:403`](../../../../../../../crates/renovate-core/src/datasources/bitrise.rs#L403) |
| 179 | returns null if the file object has no content | ported | [`crates/renovate-core/src/datasources/bitrise.rs:425`](../../../../../../../crates/renovate-core/src/datasources/bitrise.rs#L425) |
| 206 | returns null if the file object has an unexpected encoding | ported | [`crates/renovate-core/src/datasources/bitrise.rs:459`](../../../../../../../crates/renovate-core/src/datasources/bitrise.rs#L459) |

