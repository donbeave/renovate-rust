# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bun/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bun/extract.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 13 | **Status:** not-applicable

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores non-bun files | 8 | not-applicable | — | — | All tests use vi.mock(fs) + vi.mocked(fs.readLocalFile) async mocks |

### `extractAllPackageFiles() › when using the .lockb lockfile format`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores missing package.json file | 13 | not-applicable | — | — | Requires vi.mocked(fs.readLocalFile) mock |
| ignores invalid package.json file | 17 | not-applicable | — | — | Requires vi.mocked(fs.readLocalFile) mock |
| handles null response | 22 | not-applicable | — | — | Requires vi.mocked(fs.readLocalFile) mock |
| parses valid package.json file | 35 | not-applicable | — | — | Requires vi.mocked(fs.readLocalFile) mock |

### `extractAllPackageFiles() › when using the .lock lockfile format`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores missing package.json file | 72 | not-applicable | — | — | Requires vi.mocked(fs.readLocalFile) mock |
| ignores invalid package.json file | 76 | not-applicable | — | — | Requires vi.mocked(fs.readLocalFile) mock |
| handles null response | 81 | not-applicable | — | — | Requires vi.mocked(fs.readLocalFile) mock |
| parses valid package.json file | 95 | not-applicable | — | — | Requires vi.mocked(fs.readLocalFile) mock |

### `workspaces`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| processes workspace package files when workspaces are detected | 132 | not-applicable | — | — | Requires vi.mocked(fs.readLocalFile) + vi.mocked(fs.getSiblingFileName) |
| skips workspace processing when workspaces is not a valid array | 178 | not-applicable | — | — | Requires vi.mocked(fs.readLocalFile) mock |
| processes workspace package files when workspaces is an object with packages property | 218 | not-applicable | — | — | Requires vi.mocked(fs.readLocalFile) mock |
| extracts .npmrc from sibling or parent directory | 267 | not-applicable | — | — | Requires vi.mocked(fs.getSiblingFileName) + vi.mocked(fs.readLocalFile) |

---

