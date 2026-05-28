# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pixi/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pixi/artifacts.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 10 | **Status:** not-applicable

### `updateArtifacts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no pixi.lock found | 69 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns null if updatedDeps is empty | 82 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns null if unchanged | 95 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| handle TEMPORARY_ERROR | 121 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns updated pixi.lock using docker | 139 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns updated pixi.lock using install mode | 195 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns updated pixi.lock using install mode for old version lock file | 234 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns pixi version defined in requires-pixi | 272 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| catches errors | 327 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns updated pixi.lock when doing lockfile maintenance | 347 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |

---

