# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/extract/post/locked-versions.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/post/locked-versions.spec.ts
**Total tests:** 21 | **Ported:** 0 | **Actionable:** 21 | **Status:** not-applicable

### `.getLockedVersions()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses yarn.lock with yarn v1.22.0 | 57 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| uses yarn.lock with yarn v2.1.0 | 94 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| uses yarn.lock with yarn v2.2.0 | 141 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| uses yarn.lock with yarn v3.0.0 | 188 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| uses yarn.lock but doesn't override extractedConstraints | 227 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| uses package-lock.json with npm v6.0.0 | 267 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| uses locked version corresponding to workspace | 298 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| does not set locked versions for engines, packageManager, and volta deps | 348 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| does nothing if managerData is not present | 457 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| uses package-lock.json with npm v7.0.0 | 485 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| augments v2 lock file constraint | 522 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| skips augmenting v2 lock file constraint | 559 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| appends <7 to npm extractedConstraints | 596 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| skips appending <7 to npm extractedConstraints | 641 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| uses pnpm-lock | 687 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| uses pnpm-lock for pnpm.catalog depType | 748 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| uses pnpm-lock in subfolder | 808 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| uses pnpm-lock with workspaces | 869 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| should log warning if unsupported lockfileVersion is found | 947 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|

### `lockfileVersion 3`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses package-lock.json with npm v9.0.0 | 978 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|
| uses package-lock.json with npm v7.0.0 | 1019 | not-applicable | — | — | mocking framework internals — vi.mocked on npm/pnpm/yarn lock file parsers; TypeScript lock file version lookup|

---
