# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/global/config/parse/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/global/config/parse/index.spec.ts
**Total tests:** 35 | **Ported:** 0 | **Actionable:** 35 | **Status:** done

### `workers/global/config/parse/index › .parseConfigs(env, defaultArgv)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports token in env | 44 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| supports token in CLI options | 51 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| supports forceCli | 69 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| sets customEnvVariables | 84 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| supports config.force | 98 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| reads private key from file | 120 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| supports Bitbucket username/password | 145 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| massages trailing slash into endpoint | 163 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| parses global manager config | 172 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| parses host rules from env | 179 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| env dryRun = true replaced to full | 187 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| cli dryRun = true replaced to full | 197 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| resolves global presets | 204 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| throws exception if global presets cannot be resolved | 232 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| cli dryRun replaced to full | 247 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| env dryRun = false replaced to null | 254 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| cli dryRun = false replaced to null | 264 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| only initializes the file when the env var LOG_FILE is properly set | 271 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| massage onboardingNoDeps when autodiscover is false | 278 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| does not massage onboardingNoDeps when autodiscover is true | 289 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| apply secrets to global config | 299 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| overrides file config with additional file config | 319 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| merges extends from file config with additional file config | 334 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| adds extends from fileConfig only | 352 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| appends files from configFileNames to config filenames list | 363 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| supports setting configFileNames through cli | 380 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| supports setting configFileNames through env | 391 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |

### `workers/global/config/parse/index › .parseConfigs(env, defaultArgv) › when `repositories` is being overridden`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| warns when CLI config overrides repositories from file config | 405 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| warns when CLI config overrides repositories from env config | 416 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| does not warn when CLI config sets repositories without override | 429 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| does not warn when CLI config has no repositories | 438 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| does not warn when CLI config has same repositories as file config | 448 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| warns when CLI overrides repositories with repo-specific configuration | 459 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| does not warn when both values are the same | 475 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |
| warns when both values are effectively the same | 487 | not-applicable | — | — | Requires httpMock + multiple vi.mock infrastructure |

---
