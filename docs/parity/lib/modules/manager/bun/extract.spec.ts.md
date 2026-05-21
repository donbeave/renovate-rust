# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bun/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bun/extract.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores non-bun files | 8 | not-applicable | — | — | tests Bun package.json extraction; Rust bun extractor uses own parsing |

### `extractAllPackageFiles() › when using the .lockb lockfile format`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores missing package.json file | 13 | not-applicable | — | — | tests Bun package.json extraction; Rust bun extractor uses own parsing |
| ignores invalid package.json file | 17 | not-applicable | — | — | tests Bun package.json extraction; Rust bun extractor uses own parsing |
| handles null response | 22 | not-applicable | — | — | tests Bun package.json extraction; Rust bun extractor uses own parsing |
| parses valid package.json file | 35 | not-applicable | — | — | tests Bun package.json extraction; Rust bun extractor uses own parsing |

### `extractAllPackageFiles() › when using the .lock lockfile format`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores missing package.json file | 72 | not-applicable | — | — | tests Bun package.json extraction; Rust bun extractor uses own parsing |
| ignores invalid package.json file | 76 | not-applicable | — | — | tests Bun package.json extraction; Rust bun extractor uses own parsing |
| handles null response | 81 | not-applicable | — | — | tests Bun package.json extraction; Rust bun extractor uses own parsing |
| parses valid package.json file | 95 | not-applicable | — | — | tests Bun package.json extraction; Rust bun extractor uses own parsing |

### `workspaces`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| processes workspace package files when workspaces are detected | 132 | not-applicable | — | — | tests Bun package.json extraction; Rust bun extractor uses own parsing |
| skips workspace processing when workspaces is not a valid array | 178 | not-applicable | — | — | tests Bun package.json extraction; Rust bun extractor uses own parsing |
| processes workspace package files when workspaces is an object with packages property | 218 | not-applicable | — | — | tests Bun package.json extraction; Rust bun extractor uses own parsing |
| extracts .npmrc from sibling or parent directory | 267 | not-applicable | — | — | tests Bun package.json extraction; Rust bun extractor uses own parsing |

---

