# `lib/modules/datasource/maven/schema.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | trims release metadata to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1607`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1607) |
| 30 | trims snapshot metadata to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1689`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1689) |
| 47 | trims pom files to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1716`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1716) |
| 99 | preserves empty relocation tags | ported | [`crates/renovate-core/src/datasources/maven.rs:1769`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1769) |
| 120 | passes through unknown xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1788`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1788) |
| 125 | passes through prefixed pom xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1795`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1795) |
| 131 | passes through pom xml when no retained fields are present | ported | [`crates/renovate-core/src/datasources/maven.rs:1802`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1802) |
| 136 | passes through metadata xml when no retained fields are present | ported | [`crates/renovate-core/src/datasources/maven.rs:1809`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1809) |
| 141 | passes through invalid xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1816`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1816) |

