# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/init/inherited.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/init/inherited.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/init/inherited`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return the same config if repository or inheritConfig is not defined | 38 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| should return the same config if inheritConfigRepoName or inheritConfigFileName is not a string | 44 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| should throw an error if getting the raw file fails and inheritConfigStrict is true | 50 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| should return the same config if getting the raw file fails and inheritConfigStrict is false | 58 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| should throw an error if parsing the inherited config fails | 64 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| should throw an error if config includes an invalid option | 71 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| should throw an error if config includes an invalid value | 78 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| should warn if validateConfig returns warnings | 85 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| should merge inherited config | 92 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| should set hostRules from inherited config | 102 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| should decrypt encrypted values from inherited config | 123 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| should apply secrets to inherited config | 158 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| should resolve presets found in inherited config | 182 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| should warn if presets fails validation with warnings | 207 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| should throw error if presets fails validation with errors | 252 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| should remove global config from presets found in inherited config | 297 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| overwrites configFileNames set by admin config | 336 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |
| does not modify configFileNames set by admin config if configFileNames is not present in inherited config | 349 | not-applicable | — | — | tests inherited config resolution via platform file fetching; requires platform API layer |

---
