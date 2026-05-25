# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/pep621/processors/uv.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pep621/processors/uv.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** pending

### `process()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns initial dependencies if there is no tool.uv section | 38 | pending | — | — | — |
| includes uv dev dependencies if there is a tool.uv section | 50 | pending | — | — | — |
| applies git sources | 81 | pending | — | — | — |
| pinned to non-default index | 150 | pending | — | — | — |
| index with optional name | 222 | pending | — | — | — |
| override implicit default index | 257 | pending | — | — | — |
| override explicit default index | 303 | pending | — | — | — |

### `extractLockedVersions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no lockfile found | 345 | pending | — | — | — |

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws TEMPORARY_ERROR | 362 | pending | — | — | — |
| returns if no lockfile found | 371 | pending | — | — | — |
| returns null if there is no lock file | 385 | pending | — | — | — |
| returns null if the lock file is unchanged | 400 | pending | — | — | — |
| returns artifact error | 464 | pending | — | — | — |
| return update dep update | 488 | pending | — | — | — |
| performs update on private package registry | 539 | pending | — | — | — |
| dont propagate uv.tool.index into UV_EXTRA_INDEX_URL | 674 | pending | — | — | — |
| continues if Google auth is not configured | 786 | pending | — | — | — |
| return update on lockfileMaintenance | 850 | pending | — | — | — |

---

