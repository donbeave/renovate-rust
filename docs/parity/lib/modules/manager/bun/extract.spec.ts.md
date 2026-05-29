# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bun/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bun/extract.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 13 | **Status:** not-applicable

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores non-bun files | 8 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Bun dependency extraction|

### `extractAllPackageFiles() › when using the .lockb lockfile format`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores missing package.json file | 13 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Bun dependency extraction|
| ignores invalid package.json file | 17 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Bun dependency extraction|
| handles null response | 22 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Bun dependency extraction|
| parses valid package.json file | 35 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Bun dependency extraction|

### `extractAllPackageFiles() › when using the .lock lockfile format`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ignores missing package.json file | 72 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Bun dependency extraction|
| ignores invalid package.json file | 76 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Bun dependency extraction|
| handles null response | 81 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Bun dependency extraction|
| parses valid package.json file | 95 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Bun dependency extraction|

### `workspaces`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| processes workspace package files when workspaces are detected | 132 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Bun dependency extraction|
| skips workspace processing when workspaces is not a valid array | 178 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Bun dependency extraction|
| processes workspace package files when workspaces is an object with packages property | 218 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Bun dependency extraction|
| extracts .npmrc from sibling or parent directory | 267 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Bun dependency extraction|

---

