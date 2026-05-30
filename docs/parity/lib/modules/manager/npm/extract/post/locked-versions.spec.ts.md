# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/post/locked-versions.spec.ts
**Total tests:** 21 | **Ported:** 0 | **Actionable:** 21 | **Status:** pending

### `.getLockedVersions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses yarn.lock with yarn v1.22.0 | 57 | pending | — | — | — |
| uses yarn.lock with yarn v2.1.0 | 94 | pending | — | — | — |
| uses yarn.lock with yarn v2.2.0 | 141 | pending | — | — | — |
| uses yarn.lock with yarn v3.0.0 | 188 | pending | — | — | — |
| uses yarn.lock but doesn't override extractedConstraints | 227 | pending | — | — | — |
| uses package-lock.json with npm v6.0.0 | 267 | pending | — | — | — |
| uses locked version corresponding to workspace | 298 | pending | — | — | — |
| does not set locked versions for engines, packageManager, and volta deps | 348 | pending | — | — | — |
| does nothing if managerData is not present | 457 | pending | — | — | — |
| uses package-lock.json with npm v7.0.0 | 485 | pending | — | — | — |
| augments v2 lock file constraint | 522 | pending | — | — | — |
| skips augmenting v2 lock file constraint | 559 | pending | — | — | — |
| appends <7 to npm extractedConstraints | 596 | pending | — | — | — |
| skips appending <7 to npm extractedConstraints | 641 | pending | — | — | — |
| uses pnpm-lock | 687 | pending | — | — | — |
| uses pnpm-lock for pnpm.catalog depType | 748 | pending | — | — | — |
| uses pnpm-lock in subfolder | 808 | pending | — | — | — |
| uses pnpm-lock with workspaces | 869 | pending | — | — | — |
| should log warning if unsupported lockfileVersion is found | 947 | pending | — | — | — |

### `lockfileVersion 3`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses package-lock.json with npm v9.0.0 | 978 | pending | — | — | — |
| uses package-lock.json with npm v7.0.0 | 1019 | pending | — | — | — |

---
