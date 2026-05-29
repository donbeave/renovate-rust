# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/extract.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 14 | **Status:** not-applicable

### `getLockFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| found lock file | 22 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno extraction with filesystem mock|
| not found lock file | 29 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno extraction with filesystem mock|

### `processImportMap()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| importMap | 37 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno extraction with filesystem mock|
| remote importMap | 81 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno extraction with filesystem mock|
| importMap path specified but not exists | 91 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno extraction with filesystem mock|
| invalid importMap file | 98 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno extraction with filesystem mock|

### `processDenoExtract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| importMap | 107 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno extraction with filesystem mock|

### `extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| invalid deno.json file | 133 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno extraction with filesystem mock|
| multiple matched files with deno.json only | 139 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno extraction with filesystem mock|
| deno.lock without package.json | 157 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno extraction with filesystem mock|
| deno.lock when collectPackageJson returns null | 163 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno extraction with filesystem mock|
| deno.lock when collectPackageJson returns empty array | 168 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno extraction with filesystem mock|
| complex config with imports, scopes, tasks and lint | 173 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno extraction with filesystem mock|

### `extractAllPackageFiles() › workspaces`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| npm workspace compatible | 362 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno extraction with filesystem mock|

---

