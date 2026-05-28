# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/nuget/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/nuget/artifacts.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 13 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| re-throws TEMPORARY_ERROR | 53 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| aborts if no lock file found | 72 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| aborts if lock file is unchanged | 87 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| runs workload restore and updates lock file | 121 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| does not update lock file when non-proj file is changed | 174 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| updates lock file for Directory.Build.props | 194 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| updates lock file for nested Directory.Build.props | 231 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| does not update lock file when no deps changed | 268 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| performs lock file maintenance | 288 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| supports docker mode | 331 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| supports install mode | 397 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| supports global mode | 451 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| catches errors | 492 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |

---

