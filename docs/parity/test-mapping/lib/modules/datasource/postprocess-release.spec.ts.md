# `lib/modules/datasource/postprocess-release.spec.ts`

[← `datasource/_common`](../../../_by-module/datasource/_common.md) · [all modules](../../../README.md)

**7/7 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 27 | returns original release for empty datasource field | ported | `crates/renovate-core/src/datasources.rs:1647` |
| 36 | returns original release for missing datasource | ported | `crates/renovate-core/src/datasources.rs:1659` |
| 48 | returns original release for datasource with missing `postprocessrelease` method | ported | `crates/renovate-core/src/datasources.rs:1671` |
| 60 | returns original release for datasource with missing `packagename` field | ported | `crates/renovate-core/src/datasources.rs:1683` |
| 81 | updates release via `postprocessrelease` method | ported | `crates/renovate-core/src/datasources.rs:1695` |
| 110 | rejects release via `postprocessrelease` method | ported | `crates/renovate-core/src/datasources.rs:1710` |
| 131 | falls back when error was thrown | ported | `crates/renovate-core/src/datasources.rs:1721` |

