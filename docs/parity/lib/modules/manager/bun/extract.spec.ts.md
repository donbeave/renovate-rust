# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bun/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bun/extract.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 13 | **Status:** pending

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores non-bun files | 8 | pending | — | — | —|

### `extractAllPackageFiles() › when using the .lockb lockfile format`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores missing package.json file | 13 | pending | — | — | —|
| ignores invalid package.json file | 17 | pending | — | — | —|
| handles null response | 22 | pending | — | — | —|
| parses valid package.json file | 35 | pending | — | — | —|

### `extractAllPackageFiles() › when using the .lock lockfile format`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores missing package.json file | 72 | pending | — | — | —|
| ignores invalid package.json file | 76 | pending | — | — | —|
| handles null response | 81 | pending | — | — | —|
| parses valid package.json file | 95 | pending | — | — | —|

### `workspaces`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| processes workspace package files when workspaces are detected | 132 | pending | — | — | —|
| skips workspace processing when workspaces is not a valid array | 178 | pending | — | — | —|
| processes workspace package files when workspaces is an object with packages property | 218 | pending | — | — | —|
| extracts .npmrc from sibling or parent directory | 267 | pending | — | — | —|

---

