# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/git/behind-base-branch-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/git/behind-base-branch-cache.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 12 | **Status:** not-applicable

### `util/git/behind-base-branch-cache › getCachedBehindBaseResult`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if cache is not populated | 22 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git behind-base-branch cache|
| returns null if branch not found | 33 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git behind-base-branch cache|
| returns null if base branch SHA is different | 56 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git behind-base-branch cache|
| returns null if branch sha is different | 79 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git behind-base-branch cache|
| returns null if cached value is undefined | 102 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git behind-base-branch cache|
| returns null if base branch SHA is null | 124 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git behind-base-branch cache|
| returns null if branch SHA is null | 147 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git behind-base-branch cache|
| returns cached value | 170 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git behind-base-branch cache|

### `util/git/behind-base-branch-cache › setCachedBehindBasedResult`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns without updating when cache not populated | 195 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git behind-base-branch cache|
| returns without updating when branch not found | 204 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git behind-base-branch cache|
| updates cached value | 213 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git behind-base-branch cache|
| handles multiple branches | 236 | not-applicable | — | — | mocking framework internals — vi.mock on git/cache; TypeScript git behind-base-branch cache|

---

