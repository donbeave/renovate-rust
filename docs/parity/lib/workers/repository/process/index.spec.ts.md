# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/index.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 13 | **Status:** done

### `workers/repository/process/index › processRepo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| processes single branches | 28 | not-applicable | — | — | Requires vi.mock platform/scm mock infrastructure |
| processes baseBranchPatterns | 33 | not-applicable | — | — | Requires vi.mock platform/scm mock infrastructure |
| reads config from default branch if useBaseBranchConfig not specified | 49 | not-applicable | — | — | Requires vi.mock platform/scm mock infrastructure |
| reads config from branches in baseBranchPatterns if useBaseBranchConfig specified | 68 | not-applicable | — | — | Requires vi.mock platform/scm mock infrastructure |
| throws if base branch config is invalid | 92 | not-applicable | — | — | Requires vi.mock platform/scm mock infrastructure |
| handles config name mismatch between baseBranches if useBaseBranchConfig specified | 107 | not-applicable | — | — | Requires vi.mock platform/scm mock infrastructure |
| processes baseBranchPatterns dryRun extract | 127 | not-applicable | — | — | Requires vi.mock platform/scm mock infrastructure |
| finds baseBranches via regular expressions | 140 | not-applicable | — | — | Requires vi.mock platform/scm mock infrastructure |
| maps $default to defaultBranch | 191 | not-applicable | — | — | Requires vi.mock platform/scm mock infrastructure |

### `workers/repository/process/index › getBaseBranchConfig`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds base branch name to branchPrefix if multiple base branches expected - more than one base branch configured | 212 | not-applicable | — | — | Requires vi.mock platform/scm mock infrastructure |
| adds base branch name to branchPrefix if multiple base branches expected - base branch regex configured | 222 | not-applicable | — | — | Requires vi.mock platform/scm mock infrastructure |
| does not add base branch name to branchPrefix if multiple base branches are not expected - only one base branch configured | 232 | not-applicable | — | — | Requires vi.mock platform/scm mock infrastructure |
| does not add base branch name to branchPrefix if multiple base branches are not expected - baseBranchPatterns undefined | 242 | not-applicable | — | — | Requires vi.mock platform/scm mock infrastructure |

---

