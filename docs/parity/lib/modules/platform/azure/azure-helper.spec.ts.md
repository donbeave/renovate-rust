# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/azure/azure-helper.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/azure/azure-helper.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 19 | **Status:** pending

### `getRef`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should get the ref with short ref name | 23 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should not get ref | 34 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should get the ref with full ref name | 45 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getAzureBranchObj`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should get the branch object | 58 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should get the branch object when ref missing | 73 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null error GitItemNotFoundException | 86 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return null error GitUnresolvableToCommitException | 115 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return the file content because it is not a json | 144 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return null because the file is not readable | 173 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getCommitDetails`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should get commit details | 193 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getMergeMethod`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should default to NoFastForward | 208 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return NoFastForward when policy explicitly set | 220 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return RebaseMerge | 246 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return Squash | 272 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return Squash when Project wide exact branch policy exists | 298 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return default branch policy | 327 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return most specific exact branch policy | 366 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |
| should return most specific prefix branch policy | 435 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

### `getAllProjectTeams`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should get all teams | 493 | pending | — | — | Helper/orchestration functions not implemented in Rust platform layer |

---

