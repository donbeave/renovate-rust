# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/index.spec.ts
**Total tests:** 33 | **Ported:** 0 | **Actionable:** 33 | **Status:** done

### `determineLockFileDirs()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 154 | not-applicable | — | — | Requires vi.mock(fs) + vi.mock(exec/env) + git/scm mock infrastructure |
| lockfile maintenance | 168 | not-applicable | — | — | Requires vi.mock(fs) + vi.mock(exec/env) + git/scm mock infrastructure |

### `writeExistingFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 193 | not-applicable | — | — | Requires vi.mock(fs) + vi.mock(exec/env) + git/scm mock infrastructure |
| writes .npmrc files | 206 | not-applicable | — | — | Requires vi.mock(fs) + vi.mock(exec/env) + git/scm mock infrastructure |
| only sources npmrc content from package config | 226 | not-applicable | — | — | Requires vi.mock(fs) + vi.mock(exec/env) + git/scm mock infrastructure |
| works only on relevant folders | 249 | not-applicable | — | — | Requires vi.mock(fs) + vi.mock(exec/env) + git/scm mock infrastructure |
| has no npm files | 262 | not-applicable | — | — | Requires vi.mock(fs) + vi.mock(exec/env) + git/scm mock infrastructure |

### `writeUpdatedPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 268 | not-applicable | — | — | Requires vi.mock(fs) + vi.mock(exec/env) + git/scm mock infrastructure |
| missing updated packages files | 276 | not-applicable | — | — | Requires vi.mock(fs) + vi.mock(exec/env) + git/scm mock infrastructure |
| prefers artifact content over package file content for the same path | 283 | not-applicable | — | — | Requires vi.mock(fs) + vi.mock(exec/env) + git/scm mock infrastructure |

### `updateYarnBinary()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update the Yarn binary | 320 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| should return .yarnrc.yml content if it has been overwritten | 334 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| should not update the Yarn binary if the old .yarnrc.yml doesn't exist | 348 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| should not update the Yarn binary if the new .yarnrc.yml doesn't exist | 361 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| should return existing .yarnrc.yml if the new one doesn't exist | 374 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| should support Yarn with corepack | 386 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |

### `getAdditionalFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 419 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |
| works for npm | 429 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |
| adds artifact notice on beforeFallback | 463 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |
| detects if lock file contents are unchanged(reuseExistingBranch=true) | 493 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |
| detects if lock file contents are unchanged(reuseExistingBranch=false) | 521 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |
| works for yarn | 549 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |
| works for pnpm | 570 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |
| no npm files | 603 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |
| no lockfiles updates | 611 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |
| skip lock file updating | 621 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |
| reuse existing up-to-date | 653 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |
| lockfile maintenance branch exists | 670 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |
| fails for npm | 690 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |
| fails for yarn | 703 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |
| fails for pnpm | 717 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |

### `getAdditionalFiles() › should fuzzy merge yarn npmRegistries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should fuzzy merge the yarnrc Files | 756 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |
| should warn if there is an error writing the yarnrc.yml | 791 | not-applicable | — | — | Requires vi.mock(fs) + mockExecAll + git/scm mock infrastructure |

---
