# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/vendir/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/vendir/artifacts.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 14 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no vendir.lock.yml found | 45 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns null if empty vendir.lock.yml found | 57 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns null if updatedDeps is empty | 71 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns null if unchanged | 82 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns updated vendir.lock | 104 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns updated vendir.yml for lockfile maintenance | 133 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| catches errors | 161 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| rethrows for temporary error | 188 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| add artifacts to file list if vendir.yml exists | 214 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| add artifacts | 288 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| sets GIT_CONFIG variables when Host Rules are configured | 349 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| works explicit global binarySource | 431 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| supports install mode | 461 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |

### `Docker`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns updated vendir.yml for lockfile maintenance | 548 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |

---

