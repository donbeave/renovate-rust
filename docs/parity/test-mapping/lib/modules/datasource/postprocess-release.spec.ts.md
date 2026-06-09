# `lib/modules/datasource/postprocess-release.spec.ts`

[← `datasource/_common`](../../../_by-module/datasource/_common.md) · [all modules](../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 27 | returns original release for empty datasource field | ported | [`crates/renovate-core/src/datasources.rs:1685`](../../../../../../crates/renovate-core/src/datasources.rs#L1685) |
| 36 | returns original release for missing datasource | ported | [`crates/renovate-core/src/datasources.rs:1697`](../../../../../../crates/renovate-core/src/datasources.rs#L1697) |
| 48 | returns original release for datasource with missing `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:1709`](../../../../../../crates/renovate-core/src/datasources.rs#L1709) |
| 60 | returns original release for datasource with missing `packagename` field | ported | [`crates/renovate-core/src/datasources.rs:1721`](../../../../../../crates/renovate-core/src/datasources.rs#L1721) |
| 81 | updates release via `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:1733`](../../../../../../crates/renovate-core/src/datasources.rs#L1733) |
| 110 | rejects release via `postprocessrelease` method | ported | [`crates/renovate-core/src/datasources.rs:1748`](../../../../../../crates/renovate-core/src/datasources.rs#L1748) |
| 131 | falls back when error was thrown | ported | [`crates/renovate-core/src/datasources.rs:1759`](../../../../../../crates/renovate-core/src/datasources.rs#L1759) |

