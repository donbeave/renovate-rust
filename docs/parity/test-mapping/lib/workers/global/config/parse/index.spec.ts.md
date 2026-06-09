# `lib/workers/global/config/parse/index.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**17/23 in-scope tests ported** (6 pending, 12 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 44 | supports token in env | ported | [`crates/renovate-cli/src/config_env.rs:849`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L849) |
| 51 | supports token in cli options | ported | [`crates/renovate-cli/src/config_builder.rs:668`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L668) |
| 69 | supports forcecli | pending | — |
| 84 | sets customenvvariables | ported | [`crates/renovate-cli/src/config_env.rs:855`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L855) |
| 98 | supports config.force | ported | [`crates/renovate-core/src/config.rs:857`](../../../../../../../../crates/renovate-core/src/config.rs#L857) |
| 120 | reads private key from file | pending | — |
| 145 | supports bitbucket username/password | ported | [`crates/renovate-cli/src/config_env.rs:1128`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1128) |
| 163 | massages trailing slash into endpoint | ported | [`crates/renovate-cli/src/config_builder.rs:1241`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1241) |
| 172 | parses global manager config | ported | [`crates/renovate-cli/src/config_builder.rs:865`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L865) |
| 179 | parses host rules from env | ported | [`crates/renovate-cli/src/config_builder.rs:1255`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1255) |
| 187 | env dryrun = true replaced to full | ported | [`crates/renovate-cli/src/config_env.rs:1649`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1649) |
| 197 | cli dryrun = true replaced to full | ported | [`crates/renovate-cli/src/config_builder.rs:1208`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1208) |
| 204 | resolves global presets | ported | [`crates/renovate-cli/src/config_builder.rs:866`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L866) |
| 232 | throws exception if global presets cannot be resolved | pending | — |
| 247 | cli dryrun replaced to full | ported | [`crates/renovate-cli/src/config_builder.rs:1219`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1219) |
| 254 | env dryrun = false replaced to null | ported | [`crates/renovate-cli/src/config_env.rs:1664`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1664) |
| 264 | cli dryrun = false replaced to null | ported | [`crates/renovate-cli/src/config_builder.rs:1232`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1232) |
| 271 | only initializes the file when the env var log_file is properly set | opt-out | asserts side-effect fs init (getParentDir not called when LOG_FILE not set); Rust logging (tracing) setup has no direct equivalent conditional dir init in the global parse path; TS-runtime behavior. Opt as pure TS fs side-effect test. |
| 278 | massage onboardingnodeps when autodiscover is false | pending | — |
| 289 | does not massage onboardingnodeps when autodiscover is true | pending | — |
| 299 | apply secrets to global config | pending | — |
| 319 | overrides file config with additional file config | opt-out | relies on executing .js file for additional config (additional-config.js for RENOVATE_ADDITIONAL_CONFIG_FILE); out of scope (no JS runtime/require in Rust; supports json for additional per design, similar .js cases opted in file.spec and other). Opt as .js config exec dependency. |
| 334 | merges extends from file config with additional file config | opt-out | relies on executing .js file for additional config (additional-config.js for RENOVATE_ADDITIONAL_CONFIG_FILE); out of scope (no JS runtime/require in Rust; supports json for additional per design, similar .js cases opted in file.spec and other). Opt as .js config exec dependency. |
| 352 | adds extends from fileconfig only | opt-out | relies on executing .js file for additional config (additional-config.js for RENOVATE_ADDITIONAL_CONFIG_FILE); out of scope (no JS runtime/require in Rust; supports json for additional per design, similar .js cases opted in file.spec and other). Opt as .js config exec dependency. |
| 363 | appends files from configfilenames to config filenames list | ported | [`crates/renovate-cli/src/config_builder.rs:1286`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1286) |
| 380 | supports setting configfilenames through cli | ported | [`crates/renovate-cli/src/config_builder.rs:1268`](../../../../../../../../crates/renovate-cli/src/config_builder.rs#L1268) |
| 391 | supports setting configfilenames through env | ported | [`crates/renovate-cli/src/config_env.rs:1590`](../../../../../../../../crates/renovate-cli/src/config_env.rs#L1590) |
| 405 | warns when cli config overrides repositories from file config | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape with config and message about override) when cli config overrides repositories from file config; no direct Rust equivalent (tracing), core override logic covered by other ports or not the spy part. Opt as pure TS logger spy. |
| 416 | warns when cli config overrides repositories from env config | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape with config and message about override) when cli config overrides repositories from env config; no direct Rust equivalent (tracing), core override logic covered by other ports or not the spy part. Opt as pure TS logger spy. |
| 429 | does not warn when cli config sets repositories without override | opt-out | asserts TypeScript logger does not warn (spy absence) when cli config sets repositories without override; no direct Rust equivalent (tracing), core logic covered elsewhere. Opt as pure TS logger spy absence check. |
| 438 | does not warn when cli config has no repositories | opt-out | asserts TypeScript logger does not warn (spy absence) when cli config has no repositories; no direct Rust equivalent (tracing), core logic covered elsewhere. Opt as pure TS logger spy absence check. |
| 448 | does not warn when cli config has same repositories as file config | opt-out | asserts TypeScript logger does not warn (spy absence) when cli config has same repositories as file config; no direct Rust equivalent (tracing), core logic covered elsewhere. Opt as pure TS logger spy absence check. |
| 459 | warns when cli overrides repositories with repo-specific configuration | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape) when cli overrides repositories with repo-specific configuration; no direct Rust equivalent (tracing), core logic covered by other ports or not the spy part. Opt as pure TS logger spy. |
| 475 | does not warn when both values are the same | opt-out | asserts TypeScript logger does not warn (spy absence) when both values are the same; no direct Rust equivalent (tracing), core logic covered elsewhere. Opt as pure TS logger spy absence check. |
| 487 | warns when both values are effectively the same | opt-out | asserts TypeScript logger.warn spy behavior (exact call shape) when both values are effectively the same; no direct Rust equivalent (tracing), core logic covered by other ports or not the spy part. Opt as pure TS logger spy. |

