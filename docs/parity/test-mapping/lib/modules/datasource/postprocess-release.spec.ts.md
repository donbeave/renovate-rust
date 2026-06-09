# `lib/modules/datasource/postprocess-release.spec.ts`

[← `datasource/_common`](../../../_by-module/datasource/_common.md) · [all modules](../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 27 | returns original release for empty datasource field | ported | [`crates/renovate-core/src/datasources.rs:1957`](../../../../../../crates/renovate-core/src/datasources.rs#L1957) |
| 36 | returns original release for missing datasource | ported | [`crates/renovate-core/src/datasources.rs:1969`](../../../../../../crates/renovate-core/src/datasources.rs#L1969) |
| 48 | returns original release for datasource with missing `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:1981`](../../../../../../crates/renovate-core/src/datasources.rs#L1981) |
| 60 | returns original release for datasource with missing `packagename` field | ported | [`crates/renovate-core/src/datasources.rs:1993`](../../../../../../crates/renovate-core/src/datasources.rs#L1993) |
| 81 | updates release via `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:2005`](../../../../../../crates/renovate-core/src/datasources.rs#L2005) |
| 110 | rejects release via `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:2020`](../../../../../../crates/renovate-core/src/datasources.rs#L2020) |
| 131 | falls back when error was thrown | ported | [`crates/renovate-core/src/datasources.rs:2031`](../../../../../../crates/renovate-core/src/datasources.rs#L2031) |

