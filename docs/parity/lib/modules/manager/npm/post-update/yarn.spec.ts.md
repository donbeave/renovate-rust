# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/yarn.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/yarn.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| generates lock files using yarn v%s | 58 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| if nodeMaxMemory set on global config | 107 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| if nodeMaxMemory set on repo config | 148 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| only skips build if skipInstalls is false | 188 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| allows and ignore scripts | 211 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| sets http proxy | 238 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| does not use global cache if zero install is detected | 273 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs lock file updates using yarn v%s | 296 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs lock file updates and full install using yarn v%s | 335 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs lock file maintenance using yarn v%s | 359 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs lock file maintenance in subdirectory independent workspaces using yarn v%s | 395 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs yarn binary update using yarn v%s | 446 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| catches errors | 479 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| supports corepack | 489 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| supports packageManager url corepack | 535 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| supports corepack on grouping | 582 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| supports customizing corepack version via config constraints | 631 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses slim yarn instead of corepack | 690 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses devEngine.packageManager(object) instead of corepack | 729 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses devEngine.packageManager(array) instead of corepack | 768 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| patches local yarn | 807 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| patches local yarn (docker) | 853 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `checkYarnrc()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns offline mirror and yarn path | 900 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns yarn path in subdir | 916 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns offline mirror | 930 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns no offline mirror and no absolute yarn path | 944 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns offline mirror and no yarn path for non-existant yarn-path binary | 959 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| removes pure-lockfile and frozen-lockfile from .yarnrc | 973 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `fuzzyMatchAdditionalYarnrcYml()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return $expectedRegistry when parsing $additionalRegistry against local $existingRegistry | 987 | not-applicable | Mock framework internals — tests npm post-update yarn via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

---
