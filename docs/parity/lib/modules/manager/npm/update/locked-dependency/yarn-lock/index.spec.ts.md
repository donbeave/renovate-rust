# Renovate Test Detail

[Back to test map](../../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/update/locked-dependency/yarn-lock/index.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `updateLockedDependency()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if cannot parse lock file | 17 | ported | `extractors/npm.rs` | `yarn_locked_dep_fails_invalid_content` | — |
| returns if yarn lock 2 | 22 | ported | `extractors/npm.rs` | `yarn_locked_dep_unsupported_yarn2` | — |
| fails if cannot find dep | 30 | ported | `extractors/npm.rs` | `yarn_locked_dep_fails_not_found` | — |
| returns already-updated | 38 | ported | `extractors/npm.rs` | `yarn_locked_dep_already_updated` | — |
| fails if cannot update dep in-range | 46 | ported | `extractors/npm.rs` | `yarn_locked_dep_fails_out_of_range` | — |
| succeeds if can update within range | 54 | ported | `extractors/npm.rs` | `yarn_locked_dep_succeeds_in_range` | — |

---
