# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/jsonnet-bundler/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/jsonnet-bundler/artifacts.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 5 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if jsonnetfile.lock does not exist | 28 | not-applicable | — | — | Uses mockExecAll + vi.mock env + fs mock + git mock; exec/fs/git infrastructure not portable |
| returns null if there are no changes | 40 | not-applicable | — | — | Uses mockExecAll + vi.mock env + fs mock + git mock; exec/fs/git infrastructure not portable |
| updates the vendor dir when dependencies change | 64 | not-applicable | — | — | Uses mockExecAll + vi.mock env + fs mock + git mock; exec/fs/git infrastructure not portable |
| performs lock file maintenance | 139 | not-applicable | — | — | Uses mockExecAll + vi.mock env + fs mock + git mock; exec/fs/git infrastructure not portable |
| returns error when jb update fails | 173 | not-applicable | — | — | Uses mockExecAll + vi.mock env + fs mock + git mock; exec/fs/git infrastructure not portable |

---

