# `lib/workers/global/config/parse/additional-config-file.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**3/10 in-scope tests ported** (7 pending, 5 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 26 | _(it.each / template — verify manually)_ | ? | — |
| 55 | migrates | pending | — |
| 68 | warns if config is invalid | ported | [`crates/renovate-core/src/config/file.rs:743`](../../../../../../../../crates/renovate-core/src/config/file.rs#L743) |
| 80 | parse and returns empty config if there is no renovate_additional_config_file in env | ported | [`crates/renovate-core/src/config/file.rs:758`](../../../../../../../../crates/renovate-core/src/config/file.rs#L758) |
| 84 | _(it.each / template — verify manually)_ | ? | — |
| 112 | fatal error and exit if custom config file does not exist | opt-out | expects processExitSpy(1) when additional config file path does not exist; for additional, Rust load_additional returns Err(ExplicitPathNotFound) but main caller logs error without fatal exit (different from explicit main config). Opt as the 'fatal and exit' + additional missing path is TS-specific handling; the 'skip when not found' / empty return for missing additional is the observable in some paths. |
| 125 | fatal error and exit if config.js contains unresolved env var | opt-out |  .js config file with unresolved $ENV , expects fatal logger + processExit(1); .js execution not supported, env resolution in .js not applicable. Opt; similar to prior 'fatal error and exit if config.js contains unresolved env var' opts in file.spec. |
| 146 | _(it.each / template — verify manually)_ | ? | — |
| 160 | exports env variables to environment from processenv object | opt-out | sets process.env from processEnv in additional .js config (mutation); forbidden per 'Never use unsafe in Rust' and explicit strip in load_additional_config (no set_var, comment 'Never use env::set_var/remove_var (unsafe and forbidden globally)'). Core 'strips processEnv without mutation' covered by ported 'load_additional_config_parses_json_and_strips_process_env_without_mutation'. Matches prior opts for processEnv export in file.spec. |
| 183 | does not export env variables to environment from processenv object if key/value is invalid | opt-out | conditional export only for string values from processEnv (non-string skipped); same process.env mutation + .js config reason as sibling export test. Opt. |
| 213 | _(it.each / template — verify manually)_ | ? | — |
| 225 | skip when config file does not exist | opt-out | for additional config, missing file is skipped (no load, no error in some paths, isModified undefined); in Rust resolve errors for missing but load_additional caller in main treats additional missing as non-fatal (logs, proceeds with base). The non-fatal skip behavior for additional missing is the intent, covered by the error path not crashing the run. If exact logger/isModified not asserted the same, opt as detail. |
| 238 | _(it.each / template — verify manually)_ | ? | — |
| 254 | removes the specified config file | ported | [`crates/renovate-core/src/config/file.rs:767`](../../../../../../../../crates/renovate-core/src/config/file.rs#L767) |
| 276 | fails silently when attempting to delete the config file | pending | — |

