# `lib/modules/datasource/artifactory/index.spec.ts`

[← `datasource/artifactory`](../../../../_by-module/datasource/artifactory.md) · [all modules](../../../../README.md)

**8/8 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 26 | parses real data (folders): with slash at the end | ported | [`crates/renovate-core/src/datasources/artifactory.rs:234`](../../../../../../../crates/renovate-core/src/datasources/artifactory.rs#L234) |
| 42 | parses real data (files): without slash at the end | ported | [`crates/renovate-core/src/datasources/artifactory.rs:248`](../../../../../../../crates/renovate-core/src/datasources/artifactory.rs#L248) |
| 58 | parses real data (merge strategy with 2 registries) | ported | [`crates/renovate-core/src/datasources/artifactory.rs:261`](../../../../../../../crates/renovate-core/src/datasources/artifactory.rs#L261) |
| 80 | returns null without registryurl + warning | ported | [`crates/renovate-core/src/datasources/artifactory.rs:296`](../../../../../../../crates/renovate-core/src/datasources/artifactory.rs#L296) |
| 94 | returns null for empty 200 ok | ported | [`crates/renovate-core/src/datasources/artifactory.rs:303`](../../../../../../../crates/renovate-core/src/datasources/artifactory.rs#L303) |
| 108 | 404 returns null | ported | [`crates/renovate-core/src/datasources/artifactory.rs:324`](../../../../../../../crates/renovate-core/src/datasources/artifactory.rs#L324) |
| 128 | throws for error diff than 404 | ported | [`crates/renovate-core/src/datasources/artifactory.rs:343`](../../../../../../../crates/renovate-core/src/datasources/artifactory.rs#L343) |
| 139 | throws no http error | ported | [`crates/renovate-core/src/datasources/artifactory.rs:366`](../../../../../../../crates/renovate-core/src/datasources/artifactory.rs#L366) |

