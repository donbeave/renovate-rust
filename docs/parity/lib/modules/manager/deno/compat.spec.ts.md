# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/deno/compat.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/deno/compat.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable-applicable

### `extractDenoCompatiblePackageJson()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if invalid package.json | 17 | not-applicable | — | — | TS-specific library (find-packages) + filesystem mocking |
| handles null response | 24 | not-applicable | — | — | TS-specific library (find-packages) + filesystem mocking |

### `collectPackageJson()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| node-compat package.json | 44 | not-applicable | — | — | TS-specific library (find-packages) + filesystem mocking |
| handles workspaces with valid workspace member | 77 | not-applicable | — | — | TS-specific library (find-packages) + filesystem mocking |
| returns empty array when rootPackageFile is null | 141 | not-applicable | — | — | TS-specific library (find-packages) + filesystem mocking |
| handles null packageFile in workspace members | 148 | not-applicable | — | — | TS-specific library (find-packages) + filesystem mocking |

---

