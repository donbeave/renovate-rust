# `lib/workers/repository/init/merge.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**8/39 ported** (31 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 76 | returns config if not found | ported | [`crates/renovate-core/src/repo_config.rs:7454`](../../../../../../../crates/renovate-core/src/repo_config.rs#L7454) |
| 82 | returns config if not found - uses cache | pending | — |
| 96 | returns cache config from onboarding cache - package.json | pending | — |
| 111 | clones, if onboarding cache is valid but parsed config is undefined | pending | — |
| 134 | returns cache config from onboarding cache - renovate.json | pending | — |
| 153 | uses package.json config if found | ported | [`crates/renovate-core/src/repo_config.rs:7470`](../../../../../../../crates/renovate-core/src/repo_config.rs#L7470) |
| 174 | massages package.json renovate string | ported | [`crates/renovate-core/src/repo_config.rs:7555`](../../../../../../../crates/renovate-core/src/repo_config.rs#L7555) |
| 188 | returns error if cannot parse | pending | — |
| 200 | throws error if duplicate keys | pending | — |
| 215 | finds and parse renovate.json5 | ported | [`crates/renovate-core/src/repo_config.rs:7348`](../../../../../../../crates/renovate-core/src/repo_config.rs#L7348) |
| 227 | finds .github/renovate.json | ported | [`crates/renovate-core/src/repo_config.rs:7365`](../../../../../../../crates/renovate-core/src/repo_config.rs#L7365) |
| 239 | finds .gitlab/renovate.json | ported | [`crates/renovate-core/src/repo_config.rs:7380`](../../../../../../../crates/renovate-core/src/repo_config.rs#L7380) |
| 251 | finds .renovaterc.json | ported | [`crates/renovate-core/src/repo_config.rs:7395`](../../../../../../../crates/renovate-core/src/repo_config.rs#L7395) |
| 267 | finds .renovaterc.json5 | ported | [`crates/renovate-core/src/repo_config.rs:7409`](../../../../../../../crates/renovate-core/src/repo_config.rs#L7409) |
| 285 | returns if no error | pending | — |
| 289 | throws on error | pending | — |
| 306 | uses onboarding config if silent | pending | — |
| 318 | throws error if misconfigured | pending | — |
| 334 | migrates nested config | pending | — |
| 364 | ignores presets | pending | — |
| 383 | continues if no errors | pending | — |
| 394 | continues if no errors-2 | pending | — |
| 414 | sets npmtoken to npmrc when it is not inside encrypted | pending | — |
| 437 | sets npmtoken to npmrc when it is inside encrypted | pending | — |
| 464 | deletes user conifgured env after setting in mem cache | pending | — |
| 486 | applies repositoryentryconfig between global and repo file config | pending | — |
| 609 | supports repositoryentryconfig without extends or ignorepresets | pending | — |
| 642 | skips in no npmtoken found | pending | — |
| 648 | adds default npmrc registry if it does not exist | pending | — |
| 656 | adds npmtoken at end of npmrc string if ${npm_token} string not found | pending | — |
| 662 | replaces ${npm_token} with npmtoken value | pending | — |
| 673 | does nothing if npmrc is missing after token migration | pending | — |
| 681 | migrates npmtoken and sets npmrc | pending | — |
| 699 | does nothing when hostrules is not configured | pending | — |
| 711 | adds hostrules and clears queue and throttle | pending | — |
| 731 | warns on invalid hostrule and continues applying others | pending | — |
| 797 | _(it.each / template — verify manually)_ | ? | — |
| 821 | _(it.each / template — verify manually)_ | ? | — |
| 841 | should log static config validation errors and warnings | pending | — |

