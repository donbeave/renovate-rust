# `lib/modules/datasource/npm/npmrc.spec.ts`

[← `datasource/npm`](../../../../_by-module/datasource/npm.md) · [all modules](../../../../README.md)

**15/15 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 22 | parses //host | ported | [`crates/renovate-core/src/datasources/npm_npmrc.rs:531`](../../../../../../../crates/renovate-core/src/datasources/npm_npmrc.rs#L531) |
| 28 | parses //host/path | ported | [`crates/renovate-core/src/datasources/npm_npmrc.rs:522`](../../../../../../../crates/renovate-core/src/datasources/npm_npmrc.rs#L522) |
| 34 | parses https://host | ported | [`crates/renovate-core/src/datasources/npm_npmrc.rs:540`](../../../../../../../crates/renovate-core/src/datasources/npm_npmrc.rs#L540) |
| 42 | rejects invalid registries | ported | [`crates/renovate-core/src/datasources/npm_npmrc.rs:551`](../../../../../../../crates/renovate-core/src/datasources/npm_npmrc.rs#L551) |
| 50 | handles naked auth | ported | [`crates/renovate-core/src/datasources/npm_npmrc.rs:560`](../../../../../../../crates/renovate-core/src/datasources/npm_npmrc.rs#L560) |
| 66 | handles host, path and auth | ported | [`crates/renovate-core/src/datasources/npm_npmrc.rs:578`](../../../../../../../crates/renovate-core/src/datasources/npm_npmrc.rs#L578) |
| 84 | handles host, path, port and auth | ported | [`crates/renovate-core/src/datasources/npm_npmrc.rs:596`](../../../../../../../crates/renovate-core/src/datasources/npm_npmrc.rs#L596) |
| 103 | handles naked authtoken | ported | [`crates/renovate-core/src/datasources/npm_npmrc.rs:616`](../../../../../../../crates/renovate-core/src/datasources/npm_npmrc.rs#L616) |
| 118 | handles host authtoken | ported | [`crates/renovate-core/src/datasources/npm_npmrc.rs:634`](../../../../../../../crates/renovate-core/src/datasources/npm_npmrc.rs#L634) |
| 151 | handles username and _password | ported | [`crates/renovate-core/src/datasources/npm_npmrc.rs:664`](../../../../../../../crates/renovate-core/src/datasources/npm_npmrc.rs#L664) |
| 174 | sanitize _auth | ported | [`crates/renovate-core/src/datasources/npm_npmrc.rs:714`](../../../../../../../crates/renovate-core/src/datasources/npm_npmrc.rs#L714) |
| 181 | sanitize _authtoken | ported | [`crates/renovate-core/src/datasources/npm_npmrc.rs:722`](../../../../../../../crates/renovate-core/src/datasources/npm_npmrc.rs#L722) |
| 191 | sanitize _password | ported | [`crates/renovate-core/src/datasources/npm_npmrc.rs:737`](../../../../../../../crates/renovate-core/src/datasources/npm_npmrc.rs#L737) |
| 203 | sanitize _authtoken with high trust | ported | [`crates/renovate-core/src/datasources/npm_npmrc.rs:753`](../../../../../../../crates/renovate-core/src/datasources/npm_npmrc.rs#L753) |
| 214 | ignores localhost | ported | [`crates/renovate-core/src/datasources/npm_npmrc.rs:770`](../../../../../../../crates/renovate-core/src/datasources/npm_npmrc.rs#L770) |

