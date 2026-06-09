# `lib/workers/global/config/parse/additional-config-file.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**1/15 in-scope tests ported** (14 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | _(it.each / template — verify manually)_ | ? | — |
| 55 | migrates | pending | — |
| 68 | warns if config is invalid | ported | [`crates/renovate-core/src/config/file.rs:743`](../../../../../../../../crates/renovate-core/src/config/file.rs#L743) |
| 80 | parse and returns empty config if there is no renovate_additional_config_file in env | pending | — |
| 84 | _(it.each / template — verify manually)_ | ? | — |
| 112 | fatal error and exit if custom config file does not exist | pending | — |
| 125 | fatal error and exit if config.js contains unresolved env var | pending | — |
| 146 | _(it.each / template — verify manually)_ | ? | — |
| 160 | exports env variables to environment from processenv object | pending | — |
| 183 | does not export env variables to environment from processenv object if key/value is invalid | pending | — |
| 213 | _(it.each / template — verify manually)_ | ? | — |
| 225 | skip when config file does not exist | pending | — |
| 238 | _(it.each / template — verify manually)_ | ? | — |
| 254 | removes the specified config file | pending | — |
| 276 | fails silently when attempting to delete the config file | pending | — |

