# `lib/modules/datasource/postprocess-release.spec.ts`

[← `datasource/_common`](../../../_by-module/datasource/_common.md) · [all modules](../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 27 | returns original release for empty datasource field | ported | [`crates/renovate-core/src/datasources.rs:1783`](../../../../../../crates/renovate-core/src/datasources.rs#L1783) |
| 36 | returns original release for missing datasource | ported | [`crates/renovate-core/src/datasources.rs:1795`](../../../../../../crates/renovate-core/src/datasources.rs#L1795) |
| 48 | returns original release for datasource with missing `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:1807`](../../../../../../crates/renovate-core/src/datasources.rs#L1807) |
| 60 | returns original release for datasource with missing `packagename` field | ported | [`crates/renovate-core/src/datasources.rs:1819`](../../../../../../crates/renovate-core/src/datasources.rs#L1819) |
| 81 | updates release via `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:1831`](../../../../../../crates/renovate-core/src/datasources.rs#L1831) |
| 110 | rejects release via `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:1846`](../../../../../../crates/renovate-core/src/datasources.rs#L1846) |
| 131 | falls back when error was thrown | ported | [`crates/renovate-core/src/datasources.rs:1857`](../../../../../../crates/renovate-core/src/datasources.rs#L1857) |

