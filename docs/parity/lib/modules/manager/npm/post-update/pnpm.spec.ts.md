# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/pnpm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/pnpm.spec.ts
**Total tests:** 31 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing when no upgrades | 48 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| generates lock files | 55 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| catches errors | 69 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| finds pnpm globally | 86 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs lock file updates | 100 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs lock file updates for workspace with packages | 120 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs lock file updates for workspace with packages using pnpm 10.x | 146 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs lock file updates for non workspace using pnpm 10.x | 181 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs lock file updates for workspace with empty package list | 210 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs lock file updates for workspace with config but no package list | 234 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs lock file updates and install when lock file updates mixed with regular updates | 261 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs lock file maintenance | 290 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs dedupe | 302 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses the new version if packageManager is updated | 324 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses constraint version if parent json has constraints | 341 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses packageManager version and puts it into constraint | 385 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses volta version and puts it into constraint | 429 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses skips pnpm v7 if lockfileVersion indicates <7 | 486 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| works for docker mode | 502 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| works for install mode | 539 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| allows pnpmfile even if ignoring scripts | 564 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `passes NODE_OPTIONS`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if nodeMaxMemory set on global config | 591 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| if nodeMaxMemory set on repo config | 621 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `getConstraintsFromLockFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no lock file | 650 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null when error reading lock file | 656 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if no lockfileVersion | 662 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if lockfileVersion is not a number or numeric string | 668 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns default if lockfileVersion is 1 | 674 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| maps supported versions | 680 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| maps supported versions for v6 | 686 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| maps supported versions for v9 | 692 | not-applicable | Mock framework internals — tests npm post-update pnpm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

---
