# `lib/modules/datasource/npm/npmrc.spec.ts`

[← `datasource/npm`](../../../../_by-module/datasource/npm.md) · [all modules](../../../../README.md)

**15/15 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 22 | parses //host | ported | `crates/renovate-core/src/datasources/npm_npmrc.rs:531` |
| 28 | parses //host/path | ported | `crates/renovate-core/src/datasources/npm_npmrc.rs:522` |
| 34 | parses https://host | ported | `crates/renovate-core/src/datasources/npm_npmrc.rs:540` |
| 42 | rejects invalid registries | ported | `crates/renovate-core/src/datasources/npm_npmrc.rs:551` |
| 50 | handles naked auth | ported | `crates/renovate-core/src/datasources/npm_npmrc.rs:560` |
| 66 | handles host, path and auth | ported | `crates/renovate-core/src/datasources/npm_npmrc.rs:578` |
| 84 | handles host, path, port and auth | ported | `crates/renovate-core/src/datasources/npm_npmrc.rs:596` |
| 103 | handles naked authtoken | ported | `crates/renovate-core/src/datasources/npm_npmrc.rs:616` |
| 118 | handles host authtoken | ported | `crates/renovate-core/src/datasources/npm_npmrc.rs:634` |
| 151 | handles username and _password | ported | `crates/renovate-core/src/datasources/npm_npmrc.rs:664` |
| 174 | sanitize _auth | ported | `crates/renovate-core/src/datasources/npm_npmrc.rs:714` |
| 181 | sanitize _authtoken | ported | `crates/renovate-core/src/datasources/npm_npmrc.rs:722` |
| 191 | sanitize _password | ported | `crates/renovate-core/src/datasources/npm_npmrc.rs:737` |
| 203 | sanitize _authtoken with high trust | ported | `crates/renovate-core/src/datasources/npm_npmrc.rs:753` |
| 214 | ignores localhost | ported | `crates/renovate-core/src/datasources/npm_npmrc.rs:770` |

