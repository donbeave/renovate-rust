# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/automerge.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/automerge.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 13 | **Status:** not-applicable

### `workers/repository/update/pr/automerge › checkAutoMerge(pr, config)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should not automerge if not configured | 25 | not-applicable | — | — | tests PR automerge logic via platform API; platform interactions out of scope |
| should not automerge if off schedule | 30 | not-applicable | — | — | tests PR automerge logic via platform API; platform interactions out of scope |
| should automerge if enabled and pr is mergeable | 36 | not-applicable | — | — | tests PR automerge logic via platform API; platform interactions out of scope |
| should indicate if automerge failed | 46 | not-applicable | — | — | tests PR automerge logic via platform API; platform interactions out of scope |
| should automerge comment | 58 | not-applicable | — | — | tests PR automerge logic via platform API; platform interactions out of scope |
| should remove previous automerge comment when rebasing | 70 | not-applicable | — | — | tests PR automerge logic via platform API; platform interactions out of scope |
| should skip branch deletion after automerge if prune is disabled | 83 | not-applicable | — | — | tests PR automerge logic via platform API; platform interactions out of scope |
| should not automerge if enabled and pr is mergeable but cannot rebase | 93 | not-applicable | — | — | tests PR automerge logic via platform API; platform interactions out of scope |
| should not automerge if enabled and pr is mergeable but branch status is not success | 105 | not-applicable | — | — | tests PR automerge logic via platform API; platform interactions out of scope |
| should not automerge if enabled and pr is mergeable but unstable | 116 | not-applicable | — | — | tests PR automerge logic via platform API; platform interactions out of scope |
| should not automerge if enabled and pr is unmergeable | 127 | not-applicable | — | — | tests PR automerge logic via platform API; platform interactions out of scope |
| dryRun full should not automerge | 138 | not-applicable | — | — | tests PR automerge logic via platform API; platform interactions out of scope |
| dryRun full pr-comment | 150 | not-applicable | — | — | tests PR automerge logic via platform API; platform interactions out of scope |

---

