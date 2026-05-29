# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/index.spec.ts
**Total tests:** 33 | **Ported:** 0 | **Actionable:** 33 | **Status:** not-applicable

### `determineLockFileDirs()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 154 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| lockfile maintenance | 168 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|

### `writeExistingFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 193 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| writes .npmrc files | 206 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| only sources npmrc content from package config | 226 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| works only on relevant folders | 249 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| has no npm files | 262 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|

### `writeUpdatedPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 268 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| missing updated packages files | 276 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| prefers artifact content over package file content for the same path | 283 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|

### `updateYarnBinary()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update the Yarn binary | 320 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| should return .yarnrc.yml content if it has been overwritten | 334 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| should not update the Yarn binary if the old .yarnrc.yml doesn't exist | 348 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| should not update the Yarn binary if the new .yarnrc.yml doesn't exist | 361 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| should return existing .yarnrc.yml if the new one doesn't exist | 374 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| should support Yarn with corepack | 386 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|

### `getAdditionalFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 419 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| works for npm | 429 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| adds artifact notice on beforeFallback | 463 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| detects if lock file contents are unchanged(reuseExistingBranch=true) | 493 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| detects if lock file contents are unchanged(reuseExistingBranch=false) | 521 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| works for yarn | 549 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| works for pnpm | 570 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| no npm files | 603 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| no lockfiles updates | 611 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| skip lock file updating | 621 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| reuse existing up-to-date | 653 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| lockfile maintenance branch exists | 670 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| fails for npm | 690 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| fails for yarn | 703 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| fails for pnpm | 717 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|

### `getAdditionalFiles() › should fuzzy merge yarn npmRegistries`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should fuzzy merge the yarnrc Files | 756 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|
| should warn if there is an error writing the yarnrc.yml | 791 | not-applicable | — | — | mocking framework internals — vi.mock on npm/yarn/pnpm helpers; TypeScript npm post-update pipeline|

---
