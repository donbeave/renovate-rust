# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/automerge.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/automerge.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/update/branch/automerge › tryBranchAutomerge`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false if not configured for automerge | 19 | not-applicable | — | — | Platform automerge integration |
| returns false if automergeType is pr | 24 | not-applicable | — | — | Platform automerge integration |
| returns false if off schedule | 30 | not-applicable | — | — | Platform automerge integration |
| returns false if branch status is not success | 37 | not-applicable | — | — | Platform automerge integration |
| returns branch status error if branch status is failure | 44 | not-applicable | — | — | Platform automerge integration |
| returns false if PR exists | 51 | not-applicable | — | — | Platform automerge integration |
| returns false if automerge fails | 61 | not-applicable | — | — | Platform automerge integration |
| returns true if automerge succeeds | 76 | not-applicable | — | — | Platform automerge integration |
| returns true if automerge succeeds (dry-run) | 88 | not-applicable | — | — | Platform automerge integration |

---

