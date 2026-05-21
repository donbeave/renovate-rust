# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/pep621/processors/uv.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pep621/processors/uv.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `process()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns initial dependencies if there is no tool.uv section | 38 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |
| includes uv dev dependencies if there is a tool.uv section | 50 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |
| applies git sources | 81 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |
| pinned to non-default index | 150 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |
| index with optional name | 222 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |
| override implicit default index | 257 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |
| override explicit default index | 303 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |

### `extractLockedVersions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no lockfile found | 345 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws TEMPORARY_ERROR | 362 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |
| returns if no lockfile found | 371 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |
| returns null if there is no lock file | 385 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |
| returns null if the lock file is unchanged | 400 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |
| returns artifact error | 464 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |
| return update dep update | 488 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |
| performs update on private package registry | 539 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |
| dont propagate uv.tool.index into UV_EXTRA_INDEX_URL | 674 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |
| continues if Google auth is not configured | 786 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |
| return update on lockfileMaintenance | 850 | not-applicable | — | — | out of scope: artifact management; invokes uv external package manager |

---

