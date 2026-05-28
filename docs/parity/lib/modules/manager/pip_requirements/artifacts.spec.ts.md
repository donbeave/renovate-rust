# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pip_requirements/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pip_requirements/artifacts.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 8 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no updatedDeps were provided | 50 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns null if no hashes | 61 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns null if unchanged | 73 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns updated file | 97 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| ignores falsy depNames | 129 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| catches and returns errors | 161 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| supports docker mode | 190 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| supports install mode | 244 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |

---

