# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/init/merge.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/init/merge.spec.ts
**Total tests:** 40 | **Ported:** 8 | **Actionable:** 40 | **Status:** partial

### `workers/repository/init/merge › detectRepoFileConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns config if not found | 75 | ported | `repo_config.rs` | `returns_not_found_when_optional` | — |
| returns config if not found - uses cache | 81 | pending | — | — | —|
| returns cache config from onboarding cache - package.json | 95 | pending | — | — | —|
| clones, if onboarding cache is valid but parsed config is undefined | 110 | pending | — | — | —|
| returns cache config from onboarding cache - renovate.json | 133 | pending | — | — | —|
| uses package.json config if found | 152 | ported | `repo_config.rs` | `discovers_renovate_key_in_package_json` | — |
| massages package.json renovate string | 173 | ported | `repo_config.rs` | `parse_from_package_json_converts_string_to_extends` | — |
| returns error if cannot parse | 187 | pending | — | — | —|
| throws error if duplicate keys | 199 | pending | — | — | —|
| finds and parse renovate.json5 | 214 | ported | `repo_config.rs` | `discover_finds_and_parses_renovate_json5` | — |
| finds .github/renovate.json | 226 | ported | `repo_config.rs` | `discover_finds_github_renovate_json` | — |
| finds .gitlab/renovate.json | 238 | ported | `repo_config.rs` | `discover_finds_gitlab_renovate_json` | — |
| finds .renovaterc.json | 250 | ported | `repo_config.rs` | `discover_finds_renovaterc_json` | — |
| finds .renovaterc.json5 | 266 | ported | `repo_config.rs` | `discover_finds_renovaterc_json5` | — |

### `workers/repository/init/merge › checkForRepoConfigError`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no error | 284 | pending | — | — | —|
| throws on error | 288 | pending | — | — | —|

### `workers/repository/init/merge › mergeRenovateConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses onboarding config if silent | 305 | pending | — | — | —|
| throws error if misconfigured | 317 | pending | — | — | —|
| migrates nested config | 333 | pending | — | — | —|
| ignores presets | 363 | pending | — | — | —|
| continues if no errors | 382 | pending | — | — | —|
| continues if no errors-2 | 393 | pending | — | — | —|
| sets npmToken to npmrc when it is not inside encrypted | 413 | pending | — | — | —|
| sets npmToken to npmrc when it is inside encrypted | 436 | pending | — | — | —|
| deletes user conifgured env after setting in mem cache | 463 | pending | — | — | —|
| applies repositoryEntryConfig between global and repo file config | 485 | pending | — | — | —|
| supports repositoryEntryConfig without extends or ignorePresets | 608 | pending | — | — | —|

### `workers/repository/init/merge › setNpmTokenInNpmrc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips in no npmToken found | 641 | pending | — | — | —|
| adds default npmrc registry if it does not exist | 647 | pending | — | — | —|
| adds npmToken at end of npmrc string if ${NPM_TOKEN} string not found | 655 | pending | — | — | —|
| replaces ${NPM_TOKEN} with npmToken value | 661 | pending | — | — | —|

### `workers/repository/init/merge › applyNpmrc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing if npmrc is missing after token migration | 672 | pending | — | — | —|
| migrates npmToken and sets npmrc | 680 | pending | — | — | —|

### `workers/repository/init/merge › applyHostRules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing when hostRules is not configured | 698 | pending | — | — | —|
| adds hostRules and clears queue and throttle | 710 | pending | — | — | —|
| warns on invalid hostRule and continues applying others | 730 | pending | — | — | —|

### `workers/repository/init/merge › static repository config › resolveStaticRepoConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $name | 796 | pending | — | — | —|

### `workers/repository/init/merge › static repository config › resolveStaticRepoConfig termination cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $name | 820 | pending | — | — | —|
| should log static config validation errors and warnings | 840 | pending | — | — | —|

### `workers/repository/init/merge › static repository config › mergeRenovateConfig() with a static repository config`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $name | 868 | pending | — | — | —|

---

## Workers specs

| Renovate spec file | Renovate tests | Rust file | Rust tests | Status |
|--------------------|---------------|-----------|------------|--------|
<!-- workers/global/config/parse/cli.spec.ts converted to per-test format above -->
<!-- workers/global/config/parse/env.spec.ts converted to per-test format above -->
<!-- workers/global/config/parse/file.spec.ts converted to per-test format above -->
<!-- workers/repository/init/merge.spec.ts converted to per-test format above -->
<!-- workers/repository/init/apis.spec.ts converted to per-test format above -->
<!-- workers/repository/init/cache.spec.ts converted to per-test format above -->

---

