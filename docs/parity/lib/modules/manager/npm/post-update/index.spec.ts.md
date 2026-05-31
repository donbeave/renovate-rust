# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/index.spec.ts
**Total tests:** 33 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `determineLockFileDirs()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 154 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| lockfile maintenance | 168 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |

### `writeExistingFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 193 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| writes .npmrc files | 206 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| only sources npmrc content from package config | 226 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| works only on relevant folders | 249 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| has no npm files | 262 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |

### `writeUpdatedPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 268 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| missing updated packages files | 276 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| prefers artifact content over package file content for the same path | 283 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |

### `updateYarnBinary()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update the Yarn binary | 320 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| should return .yarnrc.yml content if it has been overwritten | 334 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| should not update the Yarn binary if the old .yarnrc.yml doesn't exist | 348 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| should not update the Yarn binary if the new .yarnrc.yml doesn't exist | 361 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| should return existing .yarnrc.yml if the new one doesn't exist | 374 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| should support Yarn with corepack | 386 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |

### `getAdditionalFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 419 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| works for npm | 429 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| adds artifact notice on beforeFallback | 463 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| detects if lock file contents are unchanged(reuseExistingBranch=true) | 493 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| detects if lock file contents are unchanged(reuseExistingBranch=false) | 521 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| works for yarn | 549 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| works for pnpm | 570 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| no npm files | 603 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| no lockfiles updates | 611 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| skip lock file updating | 621 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| reuse existing up-to-date | 653 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| lockfile maintenance branch exists | 670 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| fails for npm | 690 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| fails for yarn | 703 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| fails for pnpm | 717 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |

### `getAdditionalFiles() › should fuzzy merge yarn npmRegistries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should fuzzy merge the yarnrc Files | 756 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |
| should warn if there is an error writing the yarnrc.yml | 791 | not-applicable | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests npm post-update via vitest-mocked fs/exec; Rust tests this at different layer |

---
