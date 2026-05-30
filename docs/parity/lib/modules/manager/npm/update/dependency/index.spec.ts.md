# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/update/dependency/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/update/dependency/index.spec.ts
**Total tests:** 24 | **Ported:** 24 | **Actionable:** 24 | **Status:** ported

### `.updateDependency(fileContent, depType, depName, newValue)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| replaces a dependency value | 13 | ported | `extractors/npm.rs` | `npm_update_dep_replaces_value` | — |
| replaces a github dependency value | 28 | ported | `extractors/npm.rs` | `npm_update_dep_github_value` | — |
| replaces a npm package alias | 52 | ported | `extractors/npm.rs` | `npm_update_dep_npm_alias` | — |
| replaces a github short hash | 77 | ported | `extractors/npm.rs` | `npm_update_dep_short_hash` | — |
| replaces a github fully specified version | 101 | ported | `extractors/npm.rs` | `npm_update_dep_git_tag` | — |
| updates resolutions too | 123 | ported | `extractors/npm.rs` | `npm_update_dep_updates_resolutions` | — |
| updates glob resolutions | 138 | ported | `extractors/npm.rs` | `npm_update_dep_glob_resolutions` | — |
| updates glob resolutions without dep | 153 | ported | `extractors/npm.rs` | `npm_update_dep_glob_resolutions_no_dep` | — |
| replaces only the first instance of a value | 170 | ported | `extractors/npm.rs` | `npm_update_dep_first_instance` | — |
| replaces only the second instance of a value | 185 | ported | `extractors/npm.rs` | `npm_update_dep_second_instance` | — |
| handles the case where the desired version is already supported | 200 | ported | `extractors/npm.rs` | `npm_update_dep_already_at_version` | — |
| returns null if throws error | 214 | ported | `extractors/npm.rs` | `npm_update_dep_returns_null_on_error` | — |
| updates packageManager | 228 | ported | `extractors/npm.rs` | `npm_update_dep_package_manager` | — |
| returns null if empty file | 243 | ported | `extractors/npm.rs` | `npm_update_dep_null_on_empty` | — |
| replaces package | 257 | ported | `extractors/npm.rs` | `npm_update_dep_replaces_package` | — |
| supports alias-based replacement | 273 | ported | `extractors/npm.rs` | `npm_update_dep_alias_replacement` | — |
| replaces glob package resolutions | 291 | ported | `extractors/npm.rs` | `npm_update_dep_glob_package_resolution` | — |
| pins also the version in patch with npm protocol in resolutions | 307 | ported | `extractors/npm.rs` | `npm_update_dep_patch_npm_protocol` | — |
| replaces also the version in patch with range in resolutions | 322 | ported | `extractors/npm.rs` | `npm_update_dep_patch_range` | — |
| handles override dependency | 337 | ported | `extractors/npm.rs` | `npm_update_dep_override` | — |
| handles override dependency object | 361 | ported | `extractors/npm.rs` | `npm_update_dep_override_object` | — |
| handles override dependency object where lastParent === depName | 390 | ported | `extractors/npm.rs` | `npm_update_dep_override_self_parent` | — |
| handles pnpm.override dependency | 419 | ported | `extractors/npm.rs` | `npm_update_dep_pnpm_override` | — |
| handles yarn.catalogs dependencies | 446 | ported | `extractors/npm.rs` | `npm_update_dep_yarn_catalogs` | — |

---
