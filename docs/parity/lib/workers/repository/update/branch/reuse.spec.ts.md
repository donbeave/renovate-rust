# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/reuse.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/reuse.spec.ts
**Total tests:** 26 | **Ported:** 0 | **Actionable:** 26 | **Status:** done

### `workers/repository/update/branch/reuse › shouldReuseExistingBranch(config)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if branch does not exist | 28 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns true if no PR | 34 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns true if does not need rebasing | 41 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns false if does not need rebasing but has upgrades that need lockfile maintenance along with upgrades that do not | 49 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns true if does not need rebasing and lockfile update is on different packages | 77 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns true if unmergeable and cannot rebase | 99 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns true if unmergeable and can rebase, but rebaseWhen is never | 108 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns false if unmergeable and can rebase | 118 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns true if automerge branch and not stale | 127 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns false if automerge branch and stale | 135 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns true if rebaseWhen=behind-base-branch but cannot rebase | 145 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns false if automerge pr and stale | 156 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns false if getBranchForceRebase and stale | 166 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns true if automerge, rebaseWhen=never and stale | 175 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns true if automerge, rebaseWhen=conflicted and stale | 185 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns false if rebaseWhen=never, keepUpdatedLabel and stale | 194 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns false if rebaseWhen=conflicted, keepUpdatedLabel and modified | 204 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| returns true if rebaseWhen=never, miss-match keepUpdatedLabel and stale | 216 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| converts rebaseWhen=auto to behind-base-branch if automerge | 226 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| converts rebaseWhen=auto to behind-base-branch if getBranchForceRebase | 236 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| converts rebaseWhen=auto to behind-base-branch if keepUpdatedLabel | 246 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| converts rebaseWhen=auto to conflicted | 257 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| converts rebaseWhen=automerging to behind-base-branch | 266 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| converts rebaseWhen=automerging to behind-base-branch if keep-updated | 278 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| converts rebaseWhen=automerging to never | 292 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |
| converts rebaseWhen=auto to behind-base-branch if automerge is true AND branch is new | 303 | not-applicable | — | — | Requires platform + scm mock infrastructure from test util |

---
