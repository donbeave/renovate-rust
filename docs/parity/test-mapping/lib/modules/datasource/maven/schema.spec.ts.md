# `lib/modules/datasource/maven/schema.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | trims release metadata to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1607`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1607) |
| 30 | trims snapshot metadata to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1741`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1741) |
| 47 | trims pom files to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1768`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1768) |
| 99 | preserves empty relocation tags | ported | [`crates/renovate-core/src/datasources/maven.rs:1821`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1821) |
| 120 | passes through unknown xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1840`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1840) |
| 125 | passes through prefixed pom xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1847`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1847) |
| 131 | passes through pom xml when no retained fields are present | ported | [`crates/renovate-core/src/datasources/maven.rs:1854`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1854) |
| 136 | passes through metadata xml when no retained fields are present | ported | [`crates/renovate-core/src/datasources/maven.rs:1861`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1861) |
| 141 | passes through invalid xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1868`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1868) |

