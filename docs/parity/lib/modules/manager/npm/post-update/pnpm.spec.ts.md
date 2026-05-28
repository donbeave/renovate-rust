# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/pnpm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/pnpm.spec.ts
**Total tests:** 31 | **Ported:** 0 | **Actionable:** 31 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing when no upgrades | 48 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| generates lock files | 55 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| catches errors | 69 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| finds pnpm globally | 86 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs lock file updates | 100 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs lock file updates for workspace with packages | 120 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs lock file updates for workspace with packages using pnpm 10.x | 146 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs lock file updates for non workspace using pnpm 10.x | 181 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs lock file updates for workspace with empty package list | 210 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs lock file updates for workspace with config but no package list | 234 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs lock file updates and install when lock file updates mixed with regular updates | 261 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs lock file maintenance | 290 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs dedupe | 302 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| uses the new version if packageManager is updated | 324 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| uses constraint version if parent json has constraints | 341 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| uses packageManager version and puts it into constraint | 385 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| uses volta version and puts it into constraint | 429 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| uses skips pnpm v7 if lockfileVersion indicates <7 | 486 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| works for docker mode | 502 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| works for install mode | 539 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| allows pnpmfile even if ignoring scripts | 564 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |

### `passes NODE_OPTIONS`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if nodeMaxMemory set on global config | 591 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| if nodeMaxMemory set on repo config | 621 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |

### `getConstraintsFromLockFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no lock file | 650 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| returns null when error reading lock file | 656 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| returns null if no lockfileVersion | 662 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| returns null if lockfileVersion is not a number or numeric string | 668 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| returns default if lockfileVersion is 1 | 674 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| maps supported versions | 680 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| maps supported versions for v6 | 686 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |
| maps supported versions for v9 | 692 | not-applicable | — | — | Requires vi.mock(fs) mock infrastructure |

---
