# `lib/workers/global/config/parse/file.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**8/15 in-scope tests ported** (7 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 27 | _(it.each / template — verify manually)_ | ? | — |
| 61 | migrates | pending | — |
| 73 | warns if config is invalid | pending | — |
| 85 | parse and returns empty config if there is no renovate_config_file in env | ported | [`crates/renovate-core/src/config/file.rs:536`](../../../../../../../../crates/renovate-core/src/config/file.rs#L536) |
| 89 | _(it.each / template — verify manually)_ | ? | — |
| 118 | fatal error and exit if custom config file does not exist | ported | [`crates/renovate-core/src/config/file.rs:564`](../../../../../../../../crates/renovate-core/src/config/file.rs#L564) |
| 132 | fatal error and exit if config.js contains unresolved env var | pending | — |
| 153 | _(it.each / template — verify manually)_ | ? | — |
| 167 | exports env variables to environment from processenv object | pending | — |
| 190 | does not export env variables to environment from processenv object if key/value is invalid | pending | — |
| 220 | _(it.each / template — verify manually)_ | ? | — |
| 232 | skip when config file does not exist | ported | [`crates/renovate-core/src/config/file.rs:581`](../../../../../../../../crates/renovate-core/src/config/file.rs#L581) |
| 245 | _(it.each / template — verify manually)_ | ? | — |
| 261 | removes the specified config file | ported | [`crates/renovate-core/src/config/file.rs:597`](../../../../../../../../crates/renovate-core/src/config/file.rs#L597) |
| 284 | fails silently when attempting to delete the config file | ported | [`crates/renovate-core/src/config/file.rs:605`](../../../../../../../../crates/renovate-core/src/config/file.rs#L605) |

