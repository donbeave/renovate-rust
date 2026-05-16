# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/pnpm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/pnpm.spec.ts
**Total tests:** 31 | **Ported:** 0 | **Actionable:** 31 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing when no upgrades | 48 | pending | — | — | — |
| generates lock files | 55 | pending | — | — | — |
| catches errors | 69 | pending | — | — | — |
| finds pnpm globally | 86 | pending | — | — | — |
| performs lock file updates | 100 | pending | — | — | — |
| performs lock file updates for workspace with packages | 120 | pending | — | — | — |
| performs lock file updates for workspace with packages using pnpm 10.x | 146 | pending | — | — | — |
| performs lock file updates for non workspace using pnpm 10.x | 181 | pending | — | — | — |
| performs lock file updates for workspace with empty package list | 210 | pending | — | — | — |
| performs lock file updates for workspace with config but no package list | 234 | pending | — | — | — |
| performs lock file updates and install when lock file updates mixed with regular updates | 261 | pending | — | — | — |
| performs lock file maintenance | 290 | pending | — | — | — |
| performs dedupe | 302 | pending | — | — | — |
| uses the new version if packageManager is updated | 324 | pending | — | — | — |
| uses constraint version if parent json has constraints | 341 | pending | — | — | — |
| uses packageManager version and puts it into constraint | 385 | pending | — | — | — |
| uses volta version and puts it into constraint | 429 | pending | — | — | — |
| uses skips pnpm v7 if lockfileVersion indicates <7 | 486 | pending | — | — | — |
| works for docker mode | 502 | pending | — | — | — |
| works for install mode | 539 | pending | — | — | — |
| allows pnpmfile even if ignoring scripts | 564 | pending | — | — | — |

### `passes NODE_OPTIONS`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if nodeMaxMemory set on global config | 591 | pending | — | — | — |
| if nodeMaxMemory set on repo config | 621 | pending | — | — | — |

### `getConstraintsFromLockFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no lock file | 650 | pending | — | — | — |
| returns null when error reading lock file | 656 | pending | — | — | — |
| returns null if no lockfileVersion | 662 | pending | — | — | — |
| returns null if lockfileVersion is not a number or numeric string | 668 | pending | — | — | — |
| returns default if lockfileVersion is 1 | 674 | pending | — | — | — |
| maps supported versions | 680 | pending | — | — | — |
| maps supported versions for v6 | 686 | pending | — | — | — |
| maps supported versions for v9 | 692 | pending | — | — | — |

---

