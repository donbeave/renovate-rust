# `lib/workers/global/config/parse/file.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**10/12 in-scope tests ported** (2 pending, 3 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 27 | _(it.each / template — verify manually)_ | ? | — |
| 61 | migrates | ported | [`crates/renovate-core/src/repo_config.rs:13576`](../../../../../../../../crates/renovate-core/src/repo_config.rs#L13576) |
| 73 | warns if config is invalid | ported | [`crates/renovate-core/src/config/file.rs:731`](../../../../../../../../crates/renovate-core/src/config/file.rs#L731) |
| 85 | parse and returns empty config if there is no renovate_config_file in env | ported | [`crates/renovate-core/src/config/file.rs:540`](../../../../../../../../crates/renovate-core/src/config/file.rs#L540) |
| 89 | _(it.each / template — verify manually)_ | ? | — |
| 118 | fatal error and exit if custom config file does not exist | ported | [`crates/renovate-core/src/config/file.rs:568`](../../../../../../../../crates/renovate-core/src/config/file.rs#L568) |
| 132 | fatal error and exit if config.js contains unresolved env var | opt-out | depends on .js config file execution (out of scope per docs in crates/renovate-core/src/config/file.rs and CD-0003; no JS runtime/require in Rust) that triggers unresolved env var ref during load; asserts exact logger.fatal message + process.exit(1); the non-.js 'fatal error and exit if custom config file does not exist' and resolve missing path cases are already ported; pure TS .js + spy + exit behavior with no Rust equivalent. |
| 153 | _(it.each / template — verify manually)_ | ? | — |
| 167 | exports env variables to environment from processenv object | opt-out | asserts direct mutation of process.env (global) from processEnv map inside a config file (js), plus returned config omits the processEnv key; Rust's config file loader (for RENOVATE_CONFIG_FILE / additional) intentionally strips processEnv with explicit no-op and comment 'Never use env::set_var/remove_var (unsafe and forbidden globally)' in crates/renovate-core/src/config/file.rs; env mutation from user config is forbidden per 'Never use unsafe in Rust' rule and design (pre-existing unsafe set_var in file.rs untouched, no new introduced); the stripping (processEnv absent from result, no mutation) is already exercised by the ported additional-config test 'load_additional_config_parses_json_and_strips_process_env_without_mutation'; pure Node.js runtime side-effect with no Rust analogue or safe equivalent. |
| 190 | does not export env variables to environment from processenv object if key/value is invalid | opt-out | asserts conditional export from processEnv (only string values; non-string skipped) + side-effect mutation of process.env; same reasons as the sibling 'exports env variables...' test: relies on forbidden env mutation + .js config execution (out of scope); stripping logic covered by existing ported tests. |
| 220 | _(it.each / template — verify manually)_ | ? | — |
| 232 | skip when config file does not exist | ported | [`crates/renovate-core/src/config/file.rs:585`](../../../../../../../../crates/renovate-core/src/config/file.rs#L585) |
| 245 | _(it.each / template — verify manually)_ | ? | — |
| 261 | removes the specified config file | ported | [`crates/renovate-core/src/config/file.rs:601`](../../../../../../../../crates/renovate-core/src/config/file.rs#L601) |
| 284 | fails silently when attempting to delete the config file | ported | [`crates/renovate-core/src/config/file.rs:609`](../../../../../../../../crates/renovate-core/src/config/file.rs#L609) |

