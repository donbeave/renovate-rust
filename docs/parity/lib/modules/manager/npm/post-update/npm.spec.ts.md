# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/npm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/npm.spec.ts
**Total tests:** 35 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| generates lock files | 26 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| runs npm install twice | 54 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| performs lock file updates | 87 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| performs lock file updates retaining the package.json counterparts | 107 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| performs npm-shrinkwrap.json updates | 136 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| performs npm-shrinkwrap.json updates (no package-lock.json) | 163 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| performs full install | 186 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| deduplicates dependencies on installation with npm >= 7 | 204 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| deduplicates package-lock.json dependencies after installation with npm <= 6 | 236 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| deduplicates npm-shrinkwrap.json dependencies after installation with npm <= 6 | 271 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| runs twice if remediating | 311 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| catches errors | 328 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| finds npm globally | 344 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| uses docker npm | 369 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| performs lock file maintenance | 384 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| works for docker mode | 402 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| works for install mode | 442 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| does not install npm if no constraints specified | 468 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |

### `passes NODE_OPTIONS`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if nodeMaxMemory set on global config | 494 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| if nodeMaxMemory set on repo config | 538 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |

### `installs workspace only packages separately`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| workspace in sub-folder | 695 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| workspace in root folder | 727 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |

### `prevents injections`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| while performing lockfileUpdate (npm-workspaces) | 883 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| while performing lockfileUpdate (npm) | 931 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |

### `--before with minimumReleaseAge`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets --before from minimumReleaseAge | 980 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| skips --before on unparseable minimumReleaseAge | 1004 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| uses stricter npmrc before date when older than minimumReleaseAge | 1026 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| uses minimumReleaseAge date when stricter than npmrc before date | 1050 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| skips --before when minimumReleaseAge is absent even if npmrc has before | 1074 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| skips --before when .npmrc has min-release-age to avoid npm conflict | 1097 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| retries without --before on ETARGET with "with a date before" | 1120 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |
| does not retry on non-before ETARGET errors | 1166 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |

### `parseNpmrcCooldownDate › returns null`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| for: $content | 1211 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |

### `parseNpmrcCooldownDate › parses before= key`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1225 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |

### `parseNpmrcCooldownDate › parses min-release-age= key`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1238 | not-applicable | — | — | tests npm post-update script execution via Node.js exec; external tool invocation out of scope |

---

