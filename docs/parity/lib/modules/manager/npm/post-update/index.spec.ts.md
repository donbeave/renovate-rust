# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/index.spec.ts
**Total tests:** 33 | **Ported:** 0 | **Actionable:** 33 | **Status:** pending

### `determineLockFileDirs()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 154 | pending | — | — | — |
| lockfile maintenance | 168 | pending | — | — | — |

### `writeExistingFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 193 | pending | — | — | — |
| writes .npmrc files | 206 | pending | — | — | — |
| only sources npmrc content from package config | 226 | pending | — | — | — |
| works only on relevant folders | 249 | pending | — | — | — |
| has no npm files | 262 | pending | — | — | — |

### `writeUpdatedPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 268 | pending | — | — | — |
| missing updated packages files | 276 | pending | — | — | — |
| prefers artifact content over package file content for the same path | 283 | pending | — | — | — |

### `updateYarnBinary()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update the Yarn binary | 320 | pending | — | — | — |
| should return .yarnrc.yml content if it has been overwritten | 334 | pending | — | — | — |
| should not update the Yarn binary if the old .yarnrc.yml doesn't exist | 348 | pending | — | — | — |
| should not update the Yarn binary if the new .yarnrc.yml doesn't exist | 361 | pending | — | — | — |
| should return existing .yarnrc.yml if the new one doesn't exist | 374 | pending | — | — | — |
| should support Yarn with corepack | 386 | pending | — | — | — |

### `getAdditionalFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 419 | pending | — | — | — |
| works for npm | 429 | pending | — | — | — |
| adds artifact notice on beforeFallback | 463 | pending | — | — | — |
| detects if lock file contents are unchanged(reuseExistingBranch=true) | 493 | pending | — | — | — |
| detects if lock file contents are unchanged(reuseExistingBranch=false) | 521 | pending | — | — | — |
| works for yarn | 549 | pending | — | — | — |
| works for pnpm | 570 | pending | — | — | — |
| no npm files | 603 | pending | — | — | — |
| no lockfiles updates | 611 | pending | — | — | — |
| skip lock file updating | 621 | pending | — | — | — |
| reuse existing up-to-date | 653 | pending | — | — | — |
| lockfile maintenance branch exists | 670 | pending | — | — | — |
| fails for npm | 690 | pending | — | — | — |
| fails for yarn | 703 | pending | — | — | — |
| fails for pnpm | 717 | pending | — | — | — |

### `getAdditionalFiles() › should fuzzy merge yarn npmRegistries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should fuzzy merge the yarnrc Files | 756 | pending | — | — | — |
| should warn if there is an error writing the yarnrc.yml | 791 | pending | — | — | — |

---

