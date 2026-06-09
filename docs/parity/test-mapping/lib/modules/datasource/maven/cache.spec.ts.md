# `lib/modules/datasource/maven/cache.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**3/5 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 44 | persists trimmed metadata and pom bodies | ported | [`crates/renovate-core/src/datasources/maven.rs:1650`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1650) |
| 90 | serves cached trimmed xml without refetching | ported | [`crates/renovate-core/src/datasources/maven.rs:1723`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1723) |
| 128 | preserves empty relocation markers on cache hits | ported | [`crates/renovate-core/src/datasources/maven.rs:1689`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1689) |
| 169 | revalidates trimmed cached xml after 304 responses | pending | — |
| 220 | serves cached trimmed snapshot xml without refetching | pending | — |

