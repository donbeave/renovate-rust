# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/yarn.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/yarn.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 29 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| generates lock files using yarn v%s | 58 | pending | — | — | — |
| if nodeMaxMemory set on global config | 107 | pending | — | — | — |
| if nodeMaxMemory set on repo config | 148 | pending | — | — | — |
| only skips build if skipInstalls is false | 188 | pending | — | — | — |
| allows and ignore scripts | 211 | pending | — | — | — |
| sets http proxy | 238 | pending | — | — | — |
| does not use global cache if zero install is detected | 273 | pending | — | — | — |
| performs lock file updates using yarn v%s | 296 | pending | — | — | — |
| performs lock file updates and full install using yarn v%s | 335 | pending | — | — | — |
| performs lock file maintenance using yarn v%s | 359 | pending | — | — | — |
| performs lock file maintenance in subdirectory independent workspaces using yarn v%s | 395 | pending | — | — | — |
| performs yarn binary update using yarn v%s | 446 | pending | — | — | — |
| catches errors | 479 | pending | — | — | — |
| supports corepack | 489 | pending | — | — | — |
| supports packageManager url corepack | 535 | pending | — | — | — |
| supports corepack on grouping | 582 | pending | — | — | — |
| supports customizing corepack version via config constraints | 631 | pending | — | — | — |
| uses slim yarn instead of corepack | 690 | pending | — | — | — |
| uses devEngine.packageManager(object) instead of corepack | 729 | pending | — | — | — |
| uses devEngine.packageManager(array) instead of corepack | 768 | pending | — | — | — |
| patches local yarn | 807 | pending | — | — | — |
| patches local yarn (docker) | 853 | pending | — | — | — |

### `checkYarnrc()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns offline mirror and yarn path | 900 | pending | — | — | — |
| returns yarn path in subdir | 916 | pending | — | — | — |
| returns offline mirror | 930 | pending | — | — | — |
| returns no offline mirror and no absolute yarn path | 944 | pending | — | — | — |
| returns offline mirror and no yarn path for non-existant yarn-path binary | 959 | pending | — | — | — |
| removes pure-lockfile and frozen-lockfile from .yarnrc | 973 | pending | — | — | — |

### `fuzzyMatchAdditionalYarnrcYml()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return $expectedRegistry when parsing $additionalRegistry against local $existingRegistry | 987 | pending | — | — | — |

---

