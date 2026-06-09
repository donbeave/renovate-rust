# `lib/modules/datasource/postprocess-release.spec.ts`

[← `datasource/_common`](../../../_by-module/datasource/_common.md) · [all modules](../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 27 | returns original release for empty datasource field | ported | [`crates/renovate-core/src/datasources.rs:1956`](../../../../../../crates/renovate-core/src/datasources.rs#L1956) |
| 36 | returns original release for missing datasource | ported | [`crates/renovate-core/src/datasources.rs:1968`](../../../../../../crates/renovate-core/src/datasources.rs#L1968) |
| 48 | returns original release for datasource with missing `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:1980`](../../../../../../crates/renovate-core/src/datasources.rs#L1980) |
| 60 | returns original release for datasource with missing `packagename` field | ported | [`crates/renovate-core/src/datasources.rs:1992`](../../../../../../crates/renovate-core/src/datasources.rs#L1992) |
| 81 | updates release via `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:2004`](../../../../../../crates/renovate-core/src/datasources.rs#L2004) |
| 110 | rejects release via `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:2019`](../../../../../../crates/renovate-core/src/datasources.rs#L2019) |
| 131 | falls back when error was thrown | ported | [`crates/renovate-core/src/datasources.rs:2030`](../../../../../../crates/renovate-core/src/datasources.rs#L2030) |

