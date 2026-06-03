# `lib/modules/datasource/rust-version/index.spec.ts`

[← `datasource/rust-version`](../../../../_by-module/datasource/rust-version.md) · [all modules](../../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | fetches and parses manifest data | ported | [`crates/renovate-core/src/datasources/rust_version.rs:430`](../../../../../../../crates/renovate-core/src/datasources/rust_version.rs#L430) |
| 47 | deduplicates versions with latest date | ported | [`crates/renovate-core/src/datasources/rust_version.rs:362`](../../../../../../../crates/renovate-core/src/datasources/rust_version.rs#L362) |
| 70 | ignores unexpected urls | ported | [`crates/renovate-core/src/datasources/rust_version.rs:356`](../../../../../../../crates/renovate-core/src/datasources/rust_version.rs#L356) |
| 92 | ignores blank lines silently (no spurious warning) | ported | [`crates/renovate-core/src/datasources/rust_version.rs:349`](../../../../../../../crates/renovate-core/src/datasources/rust_version.rs#L349) |
| 118 | throws for network error | ported | [`crates/renovate-core/src/datasources/rust_version.rs:457`](../../../../../../../crates/renovate-core/src/datasources/rust_version.rs#L457) |

