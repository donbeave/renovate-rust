# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/index.spec.ts
**Total tests:** 33 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `determineLockFileDirs()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 154 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| lockfile maintenance | 168 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |

### `writeExistingFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 193 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| writes .npmrc files | 206 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| only sources npmrc content from package config | 226 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| works only on relevant folders | 249 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| has no npm files | 262 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |

### `writeUpdatedPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 268 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| missing updated packages files | 276 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| prefers artifact content over package file content for the same path | 283 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |

### `updateYarnBinary()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update the Yarn binary | 320 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| should return .yarnrc.yml content if it has been overwritten | 334 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| should not update the Yarn binary if the old .yarnrc.yml doesn't exist | 348 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| should not update the Yarn binary if the new .yarnrc.yml doesn't exist | 361 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| should return existing .yarnrc.yml if the new one doesn't exist | 374 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| should support Yarn with corepack | 386 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |

### `getAdditionalFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 419 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| works for npm | 429 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| adds artifact notice on beforeFallback | 463 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| detects if lock file contents are unchanged(reuseExistingBranch=true) | 493 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| detects if lock file contents are unchanged(reuseExistingBranch=false) | 521 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| works for yarn | 549 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| works for pnpm | 570 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| no npm files | 603 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| no lockfiles updates | 611 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| skip lock file updating | 621 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| reuse existing up-to-date | 653 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| lockfile maintenance branch exists | 670 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| fails for npm | 690 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| fails for yarn | 703 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| fails for pnpm | 717 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |

### `getAdditionalFiles() › should fuzzy merge yarn npmRegistries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should fuzzy merge the yarnrc Files | 756 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |
| should warn if there is an error writing the yarnrc.yml | 791 | not-applicable | — | — | tests npm post-update orchestration via Node.js exec; external tool invocation out of scope |

---

