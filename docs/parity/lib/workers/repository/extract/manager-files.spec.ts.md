# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/extract/manager-files.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/extract/manager-files.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 5 | **Status:** not-applicable

### `workers/repository/extract/manager-files › getManagerPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty of manager is disabled | 22 | not-applicable | — | — | mocking framework internals — vi.mock on file-match/html/fs; TypeScript manager file extraction pipeline|
| returns empty of manager is not enabled | 28 | not-applicable | — | — | mocking framework internals — vi.mock on file-match/html/fs; TypeScript manager file extraction pipeline|
| skips files if null content returned | 35 | not-applicable | — | — | mocking framework internals — vi.mock on file-match/html/fs; TypeScript manager file extraction pipeline|
| returns files with extractPackageFile | 46 | not-applicable | — | — | mocking framework internals — vi.mock on file-match/html/fs; TypeScript manager file extraction pipeline|
| returns files with extractAllPackageFiles | 66 | not-applicable | — | — | mocking framework internals — vi.mock on file-match/html/fs; TypeScript manager file extraction pipeline|

---

