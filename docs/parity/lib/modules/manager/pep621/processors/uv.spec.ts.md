# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/pep621/processors/uv.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pep621/processors/uv.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** done

### `process()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns initial dependencies if there is no tool.uv section | 38 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |
| includes uv dev dependencies if there is a tool.uv section | 50 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |
| applies git sources | 81 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |
| pinned to non-default index | 150 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |
| index with optional name | 222 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |
| override implicit default index | 257 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |
| override explicit default index | 303 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |

### `extractLockedVersions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no lockfile found | 345 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws TEMPORARY_ERROR | 362 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |
| returns if no lockfile found | 371 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |
| returns null if there is no lock file | 385 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |
| returns null if the lock file is unchanged | 400 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |
| returns artifact error | 464 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |
| return update dep update | 488 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |
| performs update on private package registry | 539 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |
| dont propagate uv.tool.index into UV_EXTRA_INDEX_URL | 674 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |
| continues if Google auth is not configured | 786 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |
| return update on lockfileMaintenance | 850 | not-applicable | — | — | Requires vi.mock exec/fs mock infrastructure |

---

