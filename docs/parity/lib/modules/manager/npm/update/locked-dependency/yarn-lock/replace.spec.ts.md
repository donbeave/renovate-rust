# Renovate Test Detail

[Back to test map](../../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/update/locked-dependency/yarn-lock/replace.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `replaceConstraintVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns same if Yarn 2+ | 11 | ported | `npm.rs` | `yarn_replace_returns_same_for_yarn2` | — |
| replaces without dependencies | 21 | ported | `npm.rs` | `yarn_replace_without_dependencies` | — |
| replaces with dependencies | 46 | ported | `npm.rs` | `yarn_replace_with_dependencies` | — |
| replaces constraint too | 71 | ported | `npm.rs` | `yarn_replace_constraint_too` | — |
| handles escaped constraints | 99 | ported | `npm.rs` | `yarn_replace_handles_escaped_constraints` | — |
| handles quoted | 124 | ported | `npm.rs` | `yarn_replace_handles_quoted` | — |

---

