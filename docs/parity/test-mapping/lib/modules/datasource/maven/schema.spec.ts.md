# `lib/modules/datasource/maven/schema.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | trims release metadata to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1607`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1607) |
| 30 | trims snapshot metadata to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1780`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1780) |
| 47 | trims pom files to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1807`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1807) |
| 99 | preserves empty relocation tags | ported | [`crates/renovate-core/src/datasources/maven.rs:1860`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1860) |
| 120 | passes through unknown xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1879`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1879) |
| 125 | passes through prefixed pom xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1886`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1886) |
| 131 | passes through pom xml when no retained fields are present | ported | [`crates/renovate-core/src/datasources/maven.rs:1893`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1893) |
| 136 | passes through metadata xml when no retained fields are present | ported | [`crates/renovate-core/src/datasources/maven.rs:1900`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1900) |
| 141 | passes through invalid xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1907`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1907) |

