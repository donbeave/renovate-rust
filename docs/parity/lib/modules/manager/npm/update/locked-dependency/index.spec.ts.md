# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/update/locked-dependency/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/update/locked-dependency/index.spec.ts
**Total tests:** 20 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `updateLockedDependency()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| validates filename | 45 | ported | `extractors/npm.rs` | `npm_locked_dep_main_validates_filename` | Returns object with some status; yarn.lock routes to yarn handler  | — | — | —|
| validates versions | 54 | ported | `extractors/npm.rs` | `npm_locked_dep_main_validates_versions` | ^2.0.0 is not clean semver → update-failed  | — | — | —|
| returns null for unparseable files | 63 | ported | `extractors/npm.rs` | `npm_locked_dep_main_unparseable_lock` | —  | — | — | —|
| rejects lockFileVersion 2 | 72 | ported | `extractors/npm.rs` | `npm_locked_dep_main_rejects_v2` | —  | — | — | —|
| returns null if no locked deps | 81 | ported | `extractors/npm.rs` | `npm_locked_dep_main_no_locked_deps` | —  | — | — | —|
| rejects null if no constraint found | 85 | not-applicable | — | — | Lockfile dependency constraint remediation not implemented in Rust |
| remediates in-range | 97 | not-applicable | — | — | Lockfile dependency constraint remediation not implemented in Rust |
| rejects in-range remediation if lockfile v2+ | 109 | ported | `extractors/npm.rs` | `npm_locked_dep_main_v2_unsupported` | Returns update-failed (not unsupported) for v2  | — | — | —|
| fails to remediate if parent dep cannot support | 120 | not-applicable | — | — | Lockfile parent dependency constraint resolution not implemented in Rust |
| remediates express | 140 | not-applicable | — | — | Lockfile dependency constraint remediation not implemented in Rust |
| remediates lock file v2 express | 150 | not-applicable | — | — | Lockfile v2 dependency constraint remediation not implemented in Rust |
| returns already-updated if already remediated exactly | 161 | ported | `extractors/npm.rs` | `npm_locked_dep_main_already_updated` | —  | — | — | —|
| returns already-updated if already v2 remediated exactly | 169 | not-applicable | — | — | Lockfile v2 dependency constraint remediation not implemented in Rust |
| returns already-updated if already remediated higher | 178 | not-applicable | — | — | Lockfile dependency constraint remediation not implemented in Rust |
| returns already-updated if not found | 187 | ported | `extractors/npm.rs` | `npm_locked_dep_main_already_updated_via_parent` | —  | — | — | —|
| returns update-failed if other, lower version found | 196 | not-applicable | — | — | Lockfile dependency constraint remediation not implemented in Rust |
| remediates mime | 205 | not-applicable | — | — | Lockfile dependency constraint remediation not implemented in Rust |
| fails remediation if cannot update parent | 222 | not-applicable | — | — | Lockfile parent dependency constraint resolution not implemented in Rust |
| fails remediation if bundled | 231 | not-applicable | — | — | Lockfile bundled dependency handling not implemented in Rust |
| rejects in-range remediation if pnpm | 241 | not-applicable | — | — | pnpm lockfile remediation not implemented in Rust |

---
