# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/init/inherited.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/init/inherited.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** pending

### `workers/repository/init/inherited`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the same config if repository or inheritConfig is not defined | 38 | pending | — | — | — |
| should return the same config if inheritConfigRepoName or inheritConfigFileName is not a string | 44 | pending | — | — | — |
| should throw an error if getting the raw file fails and inheritConfigStrict is true | 50 | pending | — | — | — |
| should return the same config if getting the raw file fails and inheritConfigStrict is false | 58 | pending | — | — | — |
| should throw an error if parsing the inherited config fails | 64 | pending | — | — | — |
| should throw an error if config includes an invalid option | 71 | pending | — | — | — |
| should throw an error if config includes an invalid value | 78 | pending | — | — | — |
| should warn if validateConfig returns warnings | 85 | pending | — | — | — |
| should merge inherited config | 92 | pending | — | — | — |
| should set hostRules from inherited config | 102 | pending | — | — | — |
| should decrypt encrypted values from inherited config | 123 | pending | — | — | — |
| should apply secrets to inherited config | 158 | pending | — | — | — |
| should resolve presets found in inherited config | 182 | pending | — | — | — |
| should warn if presets fails validation with warnings | 207 | pending | — | — | — |
| should throw error if presets fails validation with errors | 252 | pending | — | — | — |
| should remove global config from presets found in inherited config | 297 | pending | — | — | — |
| overwrites configFileNames set by admin config | 336 | pending | — | — | — |
| does not modify configFileNames set by admin config if configFileNames is not present in inherited config | 349 | pending | — | — | — |

---
