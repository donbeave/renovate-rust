# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/npm.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/npm.spec.ts
**Total tests:** 35 | **Ported:** 0 | **Actionable:** 35 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| generates lock files | 26 | pending | — | — | —|
| runs npm install twice | 54 | pending | — | — | —|
| performs lock file updates | 87 | pending | — | — | —|
| performs lock file updates retaining the package.json counterparts | 107 | pending | — | — | —|
| performs npm-shrinkwrap.json updates | 136 | pending | — | — | —|
| performs npm-shrinkwrap.json updates (no package-lock.json) | 163 | pending | — | — | —|
| performs full install | 186 | pending | — | — | —|
| deduplicates dependencies on installation with npm >= 7 | 204 | pending | — | — | —|
| deduplicates package-lock.json dependencies after installation with npm <= 6 | 236 | pending | — | — | —|
| deduplicates npm-shrinkwrap.json dependencies after installation with npm <= 6 | 271 | pending | — | — | —|
| runs twice if remediating | 311 | pending | — | — | —|
| catches errors | 328 | pending | — | — | —|
| finds npm globally | 344 | pending | — | — | —|
| uses docker npm | 369 | pending | — | — | —|
| performs lock file maintenance | 384 | pending | — | — | —|
| works for docker mode | 402 | pending | — | — | —|
| works for install mode | 442 | pending | — | — | —|
| does not install npm if no constraints specified | 468 | pending | — | — | —|

### `passes NODE_OPTIONS`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if nodeMaxMemory set on global config | 494 | pending | — | — | —|
| if nodeMaxMemory set on repo config | 538 | pending | — | — | —|

### `installs workspace only packages separately`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| workspace in sub-folder | 695 | pending | — | — | —|
| workspace in root folder | 727 | pending | — | — | —|

### `prevents injections`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| while performing lockfileUpdate (npm-workspaces) | 883 | pending | — | — | —|
| while performing lockfileUpdate (npm) | 931 | pending | — | — | —|

### `--before with minimumReleaseAge`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets --before from minimumReleaseAge | 980 | pending | — | — | —|
| skips --before on unparseable minimumReleaseAge | 1004 | pending | — | — | —|
| uses stricter npmrc before date when older than minimumReleaseAge | 1026 | pending | — | — | —|
| uses minimumReleaseAge date when stricter than npmrc before date | 1050 | pending | — | — | —|
| skips --before when minimumReleaseAge is absent even if npmrc has before | 1074 | pending | — | — | —|
| skips --before when .npmrc has min-release-age to avoid npm conflict | 1097 | pending | — | — | —|
| retries without --before on ETARGET with "with a date before" | 1120 | pending | — | — | —|
| does not retry on non-before ETARGET errors | 1166 | pending | — | — | —|

### `parseNpmrcCooldownDate › returns null`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| for: $content | 1211 | pending | — | — | —|

### `parseNpmrcCooldownDate › parses before= key`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1225 | pending | — | — | —|

### `parseNpmrcCooldownDate › parses min-release-age= key`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 1238 | pending | — | — | —|

---
