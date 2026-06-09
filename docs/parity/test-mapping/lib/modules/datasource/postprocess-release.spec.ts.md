# `lib/modules/datasource/postprocess-release.spec.ts`

[← `datasource/_common`](../../../_by-module/datasource/_common.md) · [all modules](../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 27 | returns original release for empty datasource field | ported | [`crates/renovate-core/src/datasources.rs:1884`](../../../../../../crates/renovate-core/src/datasources.rs#L1884) |
| 36 | returns original release for missing datasource | ported | [`crates/renovate-core/src/datasources.rs:1896`](../../../../../../crates/renovate-core/src/datasources.rs#L1896) |
| 48 | returns original release for datasource with missing `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:1908`](../../../../../../crates/renovate-core/src/datasources.rs#L1908) |
| 60 | returns original release for datasource with missing `packagename` field | ported | [`crates/renovate-core/src/datasources.rs:1920`](../../../../../../crates/renovate-core/src/datasources.rs#L1920) |
| 81 | updates release via `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:1932`](../../../../../../crates/renovate-core/src/datasources.rs#L1932) |
| 110 | rejects release via `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:1947`](../../../../../../crates/renovate-core/src/datasources.rs#L1947) |
| 131 | falls back when error was thrown | ported | [`crates/renovate-core/src/datasources.rs:1958`](../../../../../../crates/renovate-core/src/datasources.rs#L1958) |

