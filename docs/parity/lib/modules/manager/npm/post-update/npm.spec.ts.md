# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/npm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/npm.spec.ts
**Total tests:** 35 | **Ported:** 0 | **Actionable:** 35 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| generates lock files | 26 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| runs npm install twice | 54 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs lock file updates | 87 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs lock file updates retaining the package.json counterparts | 107 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs npm-shrinkwrap.json updates | 136 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs npm-shrinkwrap.json updates (no package-lock.json) | 163 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs full install | 186 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| deduplicates dependencies on installation with npm >= 7 | 204 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| deduplicates package-lock.json dependencies after installation with npm <= 6 | 236 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| deduplicates npm-shrinkwrap.json dependencies after installation with npm <= 6 | 271 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| runs twice if remediating | 311 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| catches errors | 328 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| finds npm globally | 344 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| uses docker npm | 369 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| performs lock file maintenance | 384 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| works for docker mode | 402 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| works for install mode | 442 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| does not install npm if no constraints specified | 468 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |

### `passes NODE_OPTIONS`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if nodeMaxMemory set on global config | 494 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| if nodeMaxMemory set on repo config | 538 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |

### `installs workspace only packages separately`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| workspace in sub-folder | 695 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| workspace in root folder | 727 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |

### `prevents injections`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| while performing lockfileUpdate (npm-workspaces) | 883 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| while performing lockfileUpdate (npm) | 931 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |

### `--before with minimumReleaseAge`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets --before from minimumReleaseAge | 980 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| skips --before on unparseable minimumReleaseAge | 1004 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| uses stricter npmrc before date when older than minimumReleaseAge | 1026 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| uses minimumReleaseAge date when stricter than npmrc before date | 1050 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| skips --before when minimumReleaseAge is absent even if npmrc has before | 1074 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| skips --before when .npmrc has min-release-age to avoid npm conflict | 1097 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| retries without --before on ETARGET with "with a date before" | 1120 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |
| does not retry on non-before ETARGET errors | 1166 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) + vi.mock(exec/env) mock infrastructure |

### `parseNpmrcCooldownDate › returns null`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| for: $content | 1211 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) mock infrastructure |

### `parseNpmrcCooldownDate › parses before= key`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1225 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) mock infrastructure |

### `parseNpmrcCooldownDate › parses min-release-age= key`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1238 | not-applicable | — | — | Requires mockExecAll + vi.mock(fs) mock infrastructure |

---
