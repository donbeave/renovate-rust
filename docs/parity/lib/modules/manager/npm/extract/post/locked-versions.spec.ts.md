# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/post/locked-versions.spec.ts
**Total tests:** 21 | **Ported:** 0 | **Actionable:** 21 | **Status:** done

### `.getLockedVersions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses yarn.lock with yarn v1.22.0 | 57 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers; all tests use mock infrastructure |
| uses yarn.lock with yarn v2.1.0 | 94 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| uses yarn.lock with yarn v2.2.0 | 141 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| uses yarn.lock with yarn v3.0.0 | 188 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| uses yarn.lock but doesn't override extractedConstraints | 227 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| uses package-lock.json with npm v6.0.0 | 267 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| uses locked version corresponding to workspace | 298 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| does not set locked versions for engines, packageManager, and volta deps | 348 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| does nothing if managerData is not present | 457 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| uses package-lock.json with npm v7.0.0 | 485 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| augments v2 lock file constraint | 522 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| skips augmenting v2 lock file constraint | 559 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| appends <7 to npm extractedConstraints | 596 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| skips appending <7 to npm extractedConstraints | 641 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| uses pnpm-lock | 687 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| uses pnpm-lock for pnpm.catalog depType | 748 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| uses pnpm-lock in subfolder | 808 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| uses pnpm-lock with workspaces | 869 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| should log warning if unsupported lockfileVersion is found | 947 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers + logger spy |

### `lockfileVersion 3`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses package-lock.json with npm v9.0.0 | 978 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |
| uses package-lock.json with npm v7.0.0 | 1019 | not-applicable | — | — | Requires vi.mocked on npm/pnpm/yarn helpers |

---
