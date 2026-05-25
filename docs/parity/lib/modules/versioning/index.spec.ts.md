# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/versioning/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/index.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/versioning/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return versioning list | 8 | not-applicable | — | — | Tests TypeScript module registry (`getVersioningList()`/`get()`); no equivalent dynamic module registry exists in Rust |
| should fallback to semver-coerced | 12 | not-applicable | — | — | Tests TypeScript module registry fallback behavior |
| should accept config | 18 | not-applicable | — | — | Tests TypeScript module registry config parsing |
| matches the API contract | 22 | not-applicable | — | — | Tests TypeScript OOP API contract via dynamic import and Zod schema; TypeScript-specific reflection |

---

