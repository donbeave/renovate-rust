# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/index.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 13 | **Status:** pending

### `workers/repository/process/index › processRepo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| processes single branches | 28 | pending | — | — | — |
| processes baseBranchPatterns | 33 | pending | — | — | — |
| reads config from default branch if useBaseBranchConfig not specified | 49 | pending | — | — | — |
| reads config from branches in baseBranchPatterns if useBaseBranchConfig specified | 68 | pending | — | — | — |
| throws if base branch config is invalid | 92 | pending | — | — | — |
| handles config name mismatch between baseBranches if useBaseBranchConfig specified | 107 | pending | — | — | — |
| processes baseBranchPatterns dryRun extract | 127 | pending | — | — | — |
| finds baseBranches via regular expressions | 140 | pending | — | — | — |
| maps $default to defaultBranch | 191 | pending | — | — | — |

### `workers/repository/process/index › getBaseBranchConfig`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds base branch name to branchPrefix if multiple base branches expected - more than one base branch configured | 212 | pending | — | — | — |
| adds base branch name to branchPrefix if multiple base branches expected - base branch regex configured | 222 | pending | — | — | — |
| does not add base branch name to branchPrefix if multiple base branches are not expected - only one base branch configured | 232 | pending | — | — | — |
| does not add base branch name to branchPrefix if multiple base branches are not expected - baseBranchPatterns undefined | 242 | pending | — | — | — |

---

