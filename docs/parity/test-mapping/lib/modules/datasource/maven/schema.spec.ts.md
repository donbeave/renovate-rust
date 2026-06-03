# `lib/modules/datasource/maven/schema.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**9/9 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 6 | trims release metadata to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1607`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1607) |
| 30 | trims snapshot metadata to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1650`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1650) |
| 47 | trims pom files to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1677`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1677) |
| 99 | preserves empty relocation tags | ported | [`crates/renovate-core/src/datasources/maven.rs:1730`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1730) |
| 120 | passes through unknown xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1749`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1749) |
| 125 | passes through prefixed pom xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1756`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1756) |
| 131 | passes through pom xml when no retained fields are present | ported | [`crates/renovate-core/src/datasources/maven.rs:1763`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1763) |
| 136 | passes through metadata xml when no retained fields are present | ported | [`crates/renovate-core/src/datasources/maven.rs:1770`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1770) |
| 141 | passes through invalid xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1777`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1777) |

