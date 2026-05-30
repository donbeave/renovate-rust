# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/global/config/parse/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/config/parse/index.spec.ts
**Total tests:** 35 | **Ported:** 0 | **Actionable:** 35 | **Status:** pending-applicable

### `workers/global/config/parse/index › .parseConfigs(env, defaultArgv)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports token in env  | 44 | pending | — | — | — |
| supports token in CLI options  | 51 | pending | — | — | — |
| supports forceCli  | 69 | pending | — | — | — |
| sets customEnvVariables  | 84 | pending | — | — | — |
| supports config.force  | 98 | pending | — | — | — |
| reads private key from file  | 120 | pending | — | — | — |
| supports Bitbucket username/password  | 145 | pending | — | — | — |
| massages trailing slash into endpoint  | 163 | pending | — | — | — |
| parses global manager config  | 172 | pending | — | — | — |
| parses host rules from env  | 179 | pending | — | — | — |
| env dryRun = true replaced to full  | 187 | pending | — | — | — |
| cli dryRun = true replaced to full  | 197 | pending | — | — | — |
| resolves global presets  | 204 | pending | — | — | — |
| throws exception if global presets cannot be resolved  | 232 | pending | — | — | — |
| cli dryRun replaced to full  | 247 | pending | — | — | — |
| env dryRun = false replaced to null  | 254 | pending | — | — | — |
| cli dryRun = false replaced to null  | 264 | pending | — | — | — |
| only initializes the file when the env var LOG_FILE is properly set  | 271 | pending | — | — | — |
| massage onboardingNoDeps when autodiscover is false  | 278 | pending | — | — | — |
| does not massage onboardingNoDeps when autodiscover is true  | 289 | pending | — | — | — |
| apply secrets to global config  | 299 | pending | — | — | — |
| overrides file config with additional file config  | 319 | pending | — | — | — |
| merges extends from file config with additional file config  | 334 | pending | — | — | — |
| adds extends from fileConfig only  | 352 | pending | — | — | — |
| appends files from configFileNames to config filenames list  | 363 | pending | — | — | — |
| supports setting configFileNames through cli  | 380 | pending | — | — | — |
| supports setting configFileNames through env  | 391 | pending | — | — | — |

### `workers/global/config/parse/index › .parseConfigs(env, defaultArgv) › when `repositories` is being overridden`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| warns when CLI config overrides repositories from file config  | 405 | pending | — | — | — |
| warns when CLI config overrides repositories from env config  | 416 | pending | — | — | — |
| does not warn when CLI config sets repositories without override  | 429 | pending | — | — | — |
| does not warn when CLI config has no repositories  | 438 | pending | — | — | — |
| does not warn when CLI config has same repositories as file config  | 448 | pending | — | — | — |
| warns when CLI overrides repositories with repo-specific configuration  | 459 | pending | — | — | — |
| does not warn when both values are the same  | 475 | pending | — | — | — |
| warns when both values are effectively the same  | 487 | pending | — | — | — |

---
