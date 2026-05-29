# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/compat.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/compat.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 6 | **Status:** not-applicable

### `extractDenoCompatiblePackageJson()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if invalid package.json | 17 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno Node.js compatibility pipeline|
| handles null response | 24 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno Node.js compatibility pipeline|

### `collectPackageJson()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| node-compat package.json | 44 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno Node.js compatibility pipeline|
| handles workspaces with valid workspace member | 77 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno Node.js compatibility pipeline|
| returns empty array when rootPackageFile is null | 141 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno Node.js compatibility pipeline|
| handles null packageFile in workspace members | 148 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript Deno Node.js compatibility pipeline|

---

