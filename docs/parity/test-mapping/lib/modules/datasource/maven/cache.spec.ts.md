# `lib/modules/datasource/maven/cache.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 44 | persists trimmed metadata and pom bodies | ported | [`crates/renovate-core/src/datasources/maven.rs:1650`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1650) |
| 90 | serves cached trimmed xml without refetching | ported | [`crates/renovate-core/src/datasources/maven.rs:1762`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1762) |
| 128 | preserves empty relocation markers on cache hits | ported | [`crates/renovate-core/src/datasources/maven.rs:1728`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1728) |
| 169 | revalidates trimmed cached xml after 304 responses | ported | [`crates/renovate-core/src/datasources/maven.rs:1689`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1689) |
| 220 | serves cached trimmed snapshot xml without refetching | ported | [`crates/renovate-core/src/datasources/maven.rs:1699`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1699) |

