# `lib/modules/datasource/java-version/index.spec.ts`

[← `datasource/java-version`](../../../../_by-module/datasource/java-version.md) · [all modules](../../../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 16 | throws for error | ported | [`crates/renovate-core/src/datasources/java_version.rs:207`](../../../../../../../crates/renovate-core/src/datasources/java_version.rs#L207) |
| 29 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/java_version.rs:224`](../../../../../../../crates/renovate-core/src/datasources/java_version.rs#L224) |
| 39 | returns null for empty result | ported | [`crates/renovate-core/src/datasources/java_version.rs:238`](../../../../../../../crates/renovate-core/src/datasources/java_version.rs#L238) |
| 49 | returns null for empty 200 ok | ported | [`crates/renovate-core/src/datasources/java_version.rs:252`](../../../../../../../crates/renovate-core/src/datasources/java_version.rs#L252) |
| 62 | throws for 5xx | ported | [`crates/renovate-core/src/datasources/java_version.rs:268`](../../../../../../../crates/renovate-core/src/datasources/java_version.rs#L268) |
| 72 | processes real data | ported | [`crates/renovate-core/src/datasources/java_version.rs:282`](../../../../../../../crates/renovate-core/src/datasources/java_version.rs#L282) |
| 85 | processes real data (jre) | ported | [`crates/renovate-core/src/datasources/java_version.rs:303`](../../../../../../../crates/renovate-core/src/datasources/java_version.rs#L303) |
| 98 | processes real data (jre,windows,x64) | ported | [`crates/renovate-core/src/datasources/java_version.rs:323`](../../../../../../../crates/renovate-core/src/datasources/java_version.rs#L323) |
| 110 | pages | ported | [`crates/renovate-core/src/datasources/java_version.rs:332`](../../../../../../../crates/renovate-core/src/datasources/java_version.rs#L332) |
| 128 | processes real data (jre,system) | ported | [`crates/renovate-core/src/datasources/java_version.rs:362`](../../../../../../../crates/renovate-core/src/datasources/java_version.rs#L362) |

