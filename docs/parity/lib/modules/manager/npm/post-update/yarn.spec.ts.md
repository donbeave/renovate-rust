# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/yarn.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/yarn.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 29 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| generates lock files using yarn v%s | 58 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| if nodeMaxMemory set on global config | 107 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| if nodeMaxMemory set on repo config | 148 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| only skips build if skipInstalls is false | 188 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| allows and ignore scripts | 211 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| sets http proxy | 238 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| does not use global cache if zero install is detected | 273 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs lock file updates using yarn v%s | 296 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs lock file updates and full install using yarn v%s | 335 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs lock file maintenance using yarn v%s | 359 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs lock file maintenance in subdirectory independent workspaces using yarn v%s | 395 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs yarn binary update using yarn v%s | 446 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| catches errors | 479 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| supports corepack | 489 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| supports packageManager url corepack | 535 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| supports corepack on grouping | 582 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| supports customizing corepack version via config constraints | 631 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| uses slim yarn instead of corepack | 690 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| uses devEngine.packageManager(object) instead of corepack | 729 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| uses devEngine.packageManager(array) instead of corepack | 768 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| patches local yarn | 807 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| patches local yarn (docker) | 853 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |

### `checkYarnrc()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns offline mirror and yarn path | 900 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| returns yarn path in subdir | 916 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| returns offline mirror | 930 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| returns no offline mirror and no absolute yarn path | 944 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| returns offline mirror and no yarn path for non-existant yarn-path binary | 959 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| removes pure-lockfile and frozen-lockfile from .yarnrc | 973 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |

### `fuzzyMatchAdditionalYarnrcYml()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return $expectedRegistry when parsing $additionalRegistry against local $existingRegistry | 987 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |

---
