# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/post/locked-versions.spec.ts
**Total tests:** 21 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `.getLockedVersions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses yarn.lock with yarn v1.22.0 | 57 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| uses yarn.lock with yarn v2.1.0 | 94 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| uses yarn.lock with yarn v2.2.0 | 141 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| uses yarn.lock with yarn v3.0.0 | 188 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| uses yarn.lock but doesn't override extractedConstraints | 227 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| uses package-lock.json with npm v6.0.0 | 267 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| uses locked version corresponding to workspace | 298 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| does not set locked versions for engines, packageManager, and volta deps | 348 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| does nothing if managerData is not present | 457 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| uses package-lock.json with npm v7.0.0 | 485 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| augments v2 lock file constraint | 522 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| skips augmenting v2 lock file constraint | 559 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| appends <7 to npm extractedConstraints | 596 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| skips appending <7 to npm extractedConstraints | 641 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| uses pnpm-lock | 687 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| uses pnpm-lock for pnpm.catalog depType | 748 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| uses pnpm-lock in subfolder | 808 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| uses pnpm-lock with workspaces | 869 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| should log warning if unsupported lockfileVersion is found | 947 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |

### `lockfileVersion 3`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses package-lock.json with npm v9.0.0 | 978 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |
| uses package-lock.json with npm v7.0.0 | 1019 | not-applicable | — | — | tests locked version extraction requiring npm/pnpm/yarn lockfile parsing infrastructure |

---

