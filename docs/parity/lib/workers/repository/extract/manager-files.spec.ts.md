# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/extract/manager-files.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/extract/manager-files.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable-applicable-applicable

### `workers/repository/extract/manager-files › getManagerPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty of manager is disabled  | 22 | not-applicable | Mock framework internals — tests manager-files extraction via vitest-mocked file-match/html/fs; Rust tests this at different layer | — | Worker orchestration / manager file extraction |
| returns empty of manager is not enabled  | 28 | not-applicable | Mock framework internals — tests manager-files extraction via vitest-mocked file-match/html/fs; Rust tests this at different layer | — | Worker orchestration / manager file extraction |
| skips files if null content returned  | 35 | not-applicable | Mock framework internals — tests manager-files extraction via vitest-mocked file-match/html/fs; Rust tests this at different layer | — | Worker orchestration / manager file extraction |
| returns files with extractPackageFile  | 46 | not-applicable | Mock framework internals — tests manager-files extraction via vitest-mocked file-match/html/fs; Rust tests this at different layer | — | Worker orchestration / manager file extraction |
| returns files with extractAllPackageFiles  | 66 | not-applicable | Mock framework internals — tests manager-files extraction via vitest-mocked file-match/html/fs; Rust tests this at different layer | — | Worker orchestration / manager file extraction |

---

