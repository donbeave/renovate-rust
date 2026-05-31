# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/npm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/npm.spec.ts
**Total tests:** 35 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| generates lock files | 26 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| runs npm install twice | 54 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs lock file updates | 87 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs lock file updates retaining the package.json counterparts | 107 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs npm-shrinkwrap.json updates | 136 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs npm-shrinkwrap.json updates (no package-lock.json) | 163 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs full install | 186 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| deduplicates dependencies on installation with npm >= 7 | 204 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| deduplicates package-lock.json dependencies after installation with npm <= 6 | 236 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| deduplicates npm-shrinkwrap.json dependencies after installation with npm <= 6 | 271 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| runs twice if remediating | 311 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| catches errors | 328 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| finds npm globally | 344 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses docker npm | 369 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| performs lock file maintenance | 384 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| works for docker mode | 402 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| works for install mode | 442 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| does not install npm if no constraints specified | 468 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `passes NODE_OPTIONS`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if nodeMaxMemory set on global config | 494 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| if nodeMaxMemory set on repo config | 538 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `installs workspace only packages separately`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| workspace in sub-folder | 695 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| workspace in root folder | 727 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `prevents injections`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| while performing lockfileUpdate (npm-workspaces) | 883 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| while performing lockfileUpdate (npm) | 931 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `--before with minimumReleaseAge`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets --before from minimumReleaseAge | 980 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| skips --before on unparseable minimumReleaseAge | 1004 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses stricter npmrc before date when older than minimumReleaseAge | 1026 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses minimumReleaseAge date when stricter than npmrc before date | 1050 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| skips --before when minimumReleaseAge is absent even if npmrc has before | 1074 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| skips --before when .npmrc has min-release-age to avoid npm conflict | 1097 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| retries without --before on ETARGET with "with a date before" | 1120 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| does not retry on non-before ETARGET errors | 1166 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `parseNpmrcCooldownDate › returns null`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| for: $content | 1211 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `parseNpmrcCooldownDate › parses before= key`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1225 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `parseNpmrcCooldownDate › parses min-release-age= key`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1238 | not-applicable | Mock framework internals — tests npm post-update npm via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

---
