# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/pep621/processors/pdm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pep621/processors/pdm.spec.ts
**Total tests:** 8 | **Ported:** 0 | **Actionable:** 8 | **Status:** not-applicable

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws TEMPORARY_ERROR | 31 | not-applicable | — | — | Uses mockExecAll + fs mock + GlobalConfig; exec/fs mock infrastructure not portable to Rust |
| return null if there is no lock file | 40 | not-applicable | — | — | Uses mockExecAll + fs mock + GlobalConfig; exec/fs mock infrastructure not portable to Rust |
| return null if the lock file is unchanged | 55 | not-applicable | — | — | Uses mockExecAll + fs mock + GlobalConfig; exec/fs mock infrastructure not portable to Rust |
| returns artifact error | 111 | not-applicable | — | — | Uses mockExecAll + fs mock + GlobalConfig; exec/fs mock infrastructure not portable to Rust |
| return update dep update | 135 | not-applicable | — | — | Uses mockExecAll + fs mock + GlobalConfig; exec/fs mock infrastructure not portable to Rust |
| discard dependencies if the devGroup is missing | 230 | not-applicable | — | — | Uses mockExecAll + fs mock + GlobalConfig; exec/fs mock infrastructure not portable to Rust |
| return update on lockfileMaintenance | 273 | not-applicable | — | — | Uses mockExecAll + fs mock + GlobalConfig; exec/fs mock infrastructure not portable to Rust |
| sets Git environment variables | 318 | not-applicable | — | — | Uses mockExecAll + fs mock + GlobalConfig; exec/fs mock infrastructure not portable to Rust |

---

