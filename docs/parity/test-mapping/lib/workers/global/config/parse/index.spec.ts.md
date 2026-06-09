# `lib/workers/global/config/parse/index.spec.ts`

[← `worker/global`](../../../../../_by-module/worker/global.md) · [all modules](../../../../../README.md)

**1/35 in-scope tests ported** (34 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 44 | supports token in env | pending | — |
| 51 | supports token in cli options | pending | — |
| 69 | supports forcecli | pending | — |
| 84 | sets customenvvariables | pending | — |
| 98 | supports config.force | pending | — |
| 120 | reads private key from file | pending | — |
| 145 | supports bitbucket username/password | pending | — |
| 163 | massages trailing slash into endpoint | pending | — |
| 172 | parses global manager config | pending | — |
| 179 | parses host rules from env | ported | [`crates/renovate-core/src/workers/global/config/parse/index.rs:220`](../../../../../../../../crates/renovate-core/src/workers/global/config/parse/index.rs#L220) |
| 187 | env dryrun = true replaced to full | pending | — |
| 197 | cli dryrun = true replaced to full | pending | — |
| 204 | resolves global presets | pending | — |
| 232 | throws exception if global presets cannot be resolved | pending | — |
| 247 | cli dryrun replaced to full | pending | — |
| 254 | env dryrun = false replaced to null | pending | — |
| 264 | cli dryrun = false replaced to null | pending | — |
| 271 | only initializes the file when the env var log_file is properly set | pending | — |
| 278 | massage onboardingnodeps when autodiscover is false | pending | — |
| 289 | does not massage onboardingnodeps when autodiscover is true | pending | — |
| 299 | apply secrets to global config | pending | — |
| 319 | overrides file config with additional file config | pending | — |
| 334 | merges extends from file config with additional file config | pending | — |
| 352 | adds extends from fileconfig only | pending | — |
| 363 | appends files from configfilenames to config filenames list | pending | — |
| 380 | supports setting configfilenames through cli | pending | — |
| 391 | supports setting configfilenames through env | pending | — |
| 405 | warns when cli config overrides repositories from file config | pending | — |
| 416 | warns when cli config overrides repositories from env config | pending | — |
| 429 | does not warn when cli config sets repositories without override | pending | — |
| 438 | does not warn when cli config has no repositories | pending | — |
| 448 | does not warn when cli config has same repositories as file config | pending | — |
| 459 | warns when cli overrides repositories with repo-specific configuration | pending | — |
| 475 | does not warn when both values are the same | pending | — |
| 487 | warns when both values are effectively the same | pending | — |

