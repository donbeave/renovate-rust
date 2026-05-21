# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/yarn.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/yarn.spec.ts
**Total tests:** 29 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| generates lock files using yarn v%s | 58 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| if nodeMaxMemory set on global config | 107 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| if nodeMaxMemory set on repo config | 148 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| only skips build if skipInstalls is false | 188 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| allows and ignore scripts | 211 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| sets http proxy | 238 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| does not use global cache if zero install is detected | 273 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| performs lock file updates using yarn v%s | 296 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| performs lock file updates and full install using yarn v%s | 335 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| performs lock file maintenance using yarn v%s | 359 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| performs lock file maintenance in subdirectory independent workspaces using yarn v%s | 395 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| performs yarn binary update using yarn v%s | 446 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| catches errors | 479 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| supports corepack | 489 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| supports packageManager url corepack | 535 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| supports corepack on grouping | 582 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| supports customizing corepack version via config constraints | 631 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| uses slim yarn instead of corepack | 690 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| uses devEngine.packageManager(object) instead of corepack | 729 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| uses devEngine.packageManager(array) instead of corepack | 768 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| patches local yarn | 807 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| patches local yarn (docker) | 853 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |

### `checkYarnrc()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns offline mirror and yarn path | 900 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| returns yarn path in subdir | 916 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| returns offline mirror | 930 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| returns no offline mirror and no absolute yarn path | 944 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| returns offline mirror and no yarn path for non-existant yarn-path binary | 959 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |
| removes pure-lockfile and frozen-lockfile from .yarnrc | 973 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |

### `fuzzyMatchAdditionalYarnrcYml()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return $expectedRegistry when parsing $additionalRegistry against local $existingRegistry | 987 | not-applicable | — | — | tests yarn post-update execution via Node.js exec; external tool invocation out of scope |

---

