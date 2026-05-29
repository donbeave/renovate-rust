# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/automerge.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/automerge.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 13 | **Status:** pending

### `workers/repository/update/pr/automerge › checkAutoMerge(pr, config)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should not automerge if not configured | 25 | pending | — | — | —|
| should not automerge if off schedule | 30 | pending | — | — | —|
| should automerge if enabled and pr is mergeable | 36 | pending | — | — | —|
| should indicate if automerge failed | 46 | pending | — | — | —|
| should automerge comment | 58 | pending | — | — | —|
| should remove previous automerge comment when rebasing | 70 | pending | — | — | —|
| should skip branch deletion after automerge if prune is disabled | 83 | pending | — | — | —|
| should not automerge if enabled and pr is mergeable but cannot rebase | 93 | pending | — | — | —|
| should not automerge if enabled and pr is mergeable but branch status is not success | 105 | pending | — | — | —|
| should not automerge if enabled and pr is mergeable but unstable | 116 | pending | — | — | —|
| should not automerge if enabled and pr is unmergeable | 127 | pending | — | — | —|
| dryRun full should not automerge | 138 | pending | — | — | —|
| dryRun full pr-comment | 150 | pending | — | — | —|

---

