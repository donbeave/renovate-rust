# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/modified-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/modified-cache.spec.ts
**Total tests:** 9 | **Ported:** 0 | **Actionable:** 9 | **Status:** not-applicable

### `util/git/modified-cache › getCachedModifiedResult`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if cache is not populated | 21 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git modified-files cache|
| returns null if branch not found | 25 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git modified-files cache|
| returns null if branch SHA has changed | 32 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git modified-files cache|
| returns null if cached value is undefined | 39 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git modified-files cache|
| returns null if branch sha is null | 46 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git modified-files cache|
| returns cached value | 53 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git modified-files cache|

### `util/git/modified-cache › setCachedModifiedResult`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns without updating when cache not populated | 66 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git modified-files cache|
| returns without updating when branch not found | 75 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git modified-files cache|
| handles multiple branches | 84 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git modified-files cache|

---

