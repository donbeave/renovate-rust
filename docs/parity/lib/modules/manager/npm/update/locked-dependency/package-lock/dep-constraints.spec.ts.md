# Renovate Test Detail

[Back to test map](../../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/update/locked-dependency/package-lock/dep-constraints.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/update/locked-dependency/package-lock/dep-constraints.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `findDepConstraints()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| finds indirect dependency | 11 | ported | `extractors/npm.rs` | `dep_constraints_finds_indirect` | — |
| finds direct dependency | 29 | ported | `extractors/npm.rs` | `dep_constraints_finds_direct` | — |
| skips non-matching direct dependency | 41 | ported | `extractors/npm.rs` | `dep_constraints_skips_nonmatching` | — |
| finds direct devDependency | 53 | ported | `extractors/npm.rs` | `dep_constraints_finds_dev_dep` | — |

---
