# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/automerge.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/automerge.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/update/pr/automerge › checkAutoMerge(pr, config)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should not automerge if not configured | 25 | not-applicable | — | — | Platform PR automerge |
| should not automerge if off schedule | 30 | not-applicable | — | — | Platform PR automerge |
| should automerge if enabled and pr is mergeable | 36 | not-applicable | — | — | Platform PR automerge |
| should indicate if automerge failed | 46 | not-applicable | — | — | Platform PR automerge |
| should automerge comment | 58 | not-applicable | — | — | Platform PR automerge |
| should remove previous automerge comment when rebasing | 70 | not-applicable | — | — | Platform PR automerge |
| should skip branch deletion after automerge if prune is disabled | 83 | not-applicable | — | — | Platform PR automerge |
| should not automerge if enabled and pr is mergeable but cannot rebase | 93 | not-applicable | — | — | Platform PR automerge |
| should not automerge if enabled and pr is mergeable but branch status is not success | 105 | not-applicable | — | — | Platform PR automerge |
| should not automerge if enabled and pr is mergeable but unstable | 116 | not-applicable | — | — | Platform PR automerge |
| should not automerge if enabled and pr is unmergeable | 127 | not-applicable | — | — | Platform PR automerge |
| dryRun full should not automerge | 138 | not-applicable | — | — | Platform PR automerge |
| dryRun full pr-comment | 150 | not-applicable | — | — | Platform PR automerge |

---

