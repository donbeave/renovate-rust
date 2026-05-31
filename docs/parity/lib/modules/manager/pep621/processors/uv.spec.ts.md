# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/pep621/processors/uv.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pep621/processors/uv.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `process()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns initial dependencies if there is no tool.uv section | 38 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| includes uv dev dependencies if there is a tool.uv section | 50 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| applies git sources | 81 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| pinned to non-default index | 150 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| index with optional name | 222 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| override implicit default index | 257 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| override explicit default index | 303 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `extractLockedVersions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no lockfile found | 345 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws TEMPORARY_ERROR | 362 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns if no lockfile found | 371 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if there is no lock file | 385 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if the lock file is unchanged | 400 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns artifact error | 464 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| return update dep update | 488 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs update on private package registry | 539 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| dont propagate uv.tool.index into UV_EXTRA_INDEX_URL | 674 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| continues if Google auth is not configured | 786 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| return update on lockfileMaintenance | 850 | not-applicable | Mock framework internals — tests pep621 uv via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

---

