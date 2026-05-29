# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/global/config/parse/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/config/parse/index.spec.ts
**Total tests:** 35 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/global/config/parse/index › .parseConfigs(env, defaultArgv)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports token in env | 44 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| supports token in CLI options | 51 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| supports forceCli | 69 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| sets customEnvVariables | 84 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| supports config.force | 98 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| reads private key from file | 120 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| supports Bitbucket username/password | 145 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| massages trailing slash into endpoint | 163 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| parses global manager config | 172 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| parses host rules from env | 179 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| env dryRun = true replaced to full | 187 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| cli dryRun = true replaced to full | 197 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| resolves global presets | 204 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| throws exception if global presets cannot be resolved | 232 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| cli dryRun replaced to full | 247 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| env dryRun = false replaced to null | 254 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| cli dryRun = false replaced to null | 264 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| only initializes the file when the env var LOG_FILE is properly set | 271 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| massage onboardingNoDeps when autodiscover is false | 278 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| does not massage onboardingNoDeps when autodiscover is true | 289 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| apply secrets to global config | 299 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| overrides file config with additional file config | 319 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| merges extends from file config with additional file config | 334 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| adds extends from fileConfig only | 352 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| appends files from configFileNames to config filenames list | 363 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| supports setting configFileNames through cli | 380 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| supports setting configFileNames through env | 391 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|

### `workers/global/config/parse/index › .parseConfigs(env, defaultArgv) › when `repositories` is being overridden`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| warns when CLI config overrides repositories from file config | 405 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| warns when CLI config overrides repositories from env config | 416 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| does not warn when CLI config sets repositories without override | 429 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| does not warn when CLI config has no repositories | 438 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| does not warn when CLI config has same repositories as file config | 448 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| warns when CLI overrides repositories with repo-specific configuration | 459 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| does not warn when both values are the same | 475 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|
| warns when both values are effectively the same | 487 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on all config subsystems; TypeScript config parse pipeline|

---
