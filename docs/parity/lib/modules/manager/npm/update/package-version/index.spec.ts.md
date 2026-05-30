# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/update/package-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/update/package-version/index.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `.bumpPackageVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| mirrors | 11 | ported | `npm.rs` | `npm_bump_mirrors_dependency_version` | — |
| aborts mirror | 21 | ported | `npm.rs` | `npm_bump_aborts_mirror_when_dep_not_found` | — |
| increments | 30 | ported | `npm.rs` | `npm_bump_increments_patch` | — |
| no ops | 40 | ported | `npm.rs` | `npm_bump_no_op_when_bumped_version_matches_content` | — |
| updates | 49 | ported | `npm.rs` | `npm_bump_updates_minor` | — |
| returns content if bumping errors | 59 | ported | `npm.rs` | `npm_bump_returns_content_on_invalid_bump_type` | — |

---

