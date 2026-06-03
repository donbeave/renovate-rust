# `lib/modules/datasource/postprocess-release.spec.ts`

[← `datasource/_common`](../../../_by-module/datasource/_common.md) · [all modules](../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 27 | returns original release for empty datasource field | ported | [`crates/renovate-core/src/datasources.rs:1647`](../../../../../../crates/renovate-core/src/datasources.rs#L1647) |
| 36 | returns original release for missing datasource | ported | [`crates/renovate-core/src/datasources.rs:1659`](../../../../../../crates/renovate-core/src/datasources.rs#L1659) |
| 48 | returns original release for datasource with missing `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:1671`](../../../../../../crates/renovate-core/src/datasources.rs#L1671) |
| 60 | returns original release for datasource with missing `packagename` field | ported | [`crates/renovate-core/src/datasources.rs:1683`](../../../../../../crates/renovate-core/src/datasources.rs#L1683) |
| 81 | updates release via `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:1695`](../../../../../../crates/renovate-core/src/datasources.rs#L1695) |
| 110 | rejects release via `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:1710`](../../../../../../crates/renovate-core/src/datasources.rs#L1710) |
| 131 | falls back when error was thrown | ported | [`crates/renovate-core/src/datasources.rs:1721`](../../../../../../crates/renovate-core/src/datasources.rs#L1721) |

