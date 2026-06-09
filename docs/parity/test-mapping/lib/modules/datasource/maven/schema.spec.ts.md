# `lib/modules/datasource/maven/schema.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | trims release metadata to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1607`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1607) |
| 30 | trims snapshot metadata to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1760`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1760) |
| 47 | trims pom files to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1787`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1787) |
| 99 | preserves empty relocation tags | ported | [`crates/renovate-core/src/datasources/maven.rs:1840`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1840) |
| 120 | passes through unknown xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1859`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1859) |
| 125 | passes through prefixed pom xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1866`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1866) |
| 131 | passes through pom xml when no retained fields are present | ported | [`crates/renovate-core/src/datasources/maven.rs:1873`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1873) |
| 136 | passes through metadata xml when no retained fields are present | ported | [`crates/renovate-core/src/datasources/maven.rs:1880`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1880) |
| 141 | passes through invalid xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1887`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1887) |

