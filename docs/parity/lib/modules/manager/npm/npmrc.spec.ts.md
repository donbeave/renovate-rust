# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/npm/npmrc.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/npmrc.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `resolveNpmrc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined if no .npmrc exists and no config.npmrc | 19 | not-applicable | — | — | tests .npmrc file parsing using Node.js fs and GlobalConfig; needs separate integration test |
| uses config.npmrc if no .npmrc is found | 24 | not-applicable | — | — | tests .npmrc file parsing using Node.js fs and GlobalConfig; needs separate integration test |
| finds and filters .npmrc | 31 | not-applicable | — | — | tests .npmrc file parsing using Node.js fs and GlobalConfig; needs separate integration test |
| uses config.npmrc if .npmrc does exist but npmrcMerge=false | 53 | not-applicable | — | — | tests .npmrc file parsing using Node.js fs and GlobalConfig; needs separate integration test |
| uses config.npmrc if no .npmrc file is found | 81 | not-applicable | — | — | tests .npmrc file parsing using Node.js fs and GlobalConfig; needs separate integration test |
| merges config.npmrc and repo .npmrc when npmrcMerge=true | 98 | not-applicable | — | — | tests .npmrc file parsing using Node.js fs and GlobalConfig; needs separate integration test |
| does not add a newline between config.npmrc and repo .npmrc when npmrcMerge is true, if a newline already exists | 123 | not-applicable | — | — | tests .npmrc file parsing using Node.js fs and GlobalConfig; needs separate integration test |
| finds and filters .npmrc with variables | 156 | not-applicable | — | — | tests .npmrc file parsing using Node.js fs and GlobalConfig; needs separate integration test |
| keeps variables when exposeAllEnv is true | 180 | not-applicable | — | — | tests .npmrc file parsing using Node.js fs and GlobalConfig; needs separate integration test |

---

