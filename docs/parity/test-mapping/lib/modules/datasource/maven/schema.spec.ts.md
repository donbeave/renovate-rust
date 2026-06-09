# `lib/modules/datasource/maven/schema.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 6 | trims release metadata to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1607`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1607) |
| 30 | trims snapshot metadata to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1786`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1786) |
| 47 | trims pom files to the fields used by renovate | ported | [`crates/renovate-core/src/datasources/maven.rs:1813`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1813) |
| 99 | preserves empty relocation tags | ported | [`crates/renovate-core/src/datasources/maven.rs:1866`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1866) |
| 120 | passes through unknown xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1885`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1885) |
| 125 | passes through prefixed pom xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1892`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1892) |
| 131 | passes through pom xml when no retained fields are present | ported | [`crates/renovate-core/src/datasources/maven.rs:1899`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1899) |
| 136 | passes through metadata xml when no retained fields are present | ported | [`crates/renovate-core/src/datasources/maven.rs:1906`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1906) |
| 141 | passes through invalid xml unchanged | ported | [`crates/renovate-core/src/datasources/maven.rs:1913`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1913) |

