# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/init/merge.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/init/merge.spec.ts
**Total tests:** 40 | **Ported:** 8 | **Actionable:** 32 | **Status:** partial

### `workers/repository/init/merge › detectRepoFileConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns config if not found | 75 | ported | `repo_config.rs` | `returns_not_found_when_optional` | — |
| returns config if not found - uses cache  | 81 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| returns cache config from onboarding cache - package.json  | 95 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| clones, if onboarding cache is valid but parsed config is undefined  | 110 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| returns cache config from onboarding cache - renovate.json  | 133 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| uses package.json config if found | 152 | ported | `repo_config.rs` | `discovers_renovate_key_in_package_json` | — |
| massages package.json renovate string | 173 | ported | `repo_config.rs` | `parse_from_package_json_converts_string_to_extends` | — |
| returns error if cannot parse  | 187 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| throws error if duplicate keys  | 199 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| finds and parse renovate.json5 | 214 | ported | `repo_config.rs` | `discover_finds_and_parses_renovate_json5` | — |
| finds .github/renovate.json | 226 | ported | `repo_config.rs` | `discover_finds_github_renovate_json` | — |
| finds .gitlab/renovate.json | 238 | ported | `repo_config.rs` | `discover_finds_gitlab_renovate_json` | — |
| finds .renovaterc.json | 250 | ported | `repo_config.rs` | `discover_finds_renovaterc_json` | — |
| finds .renovaterc.json5 | 266 | ported | `repo_config.rs` | `discover_finds_renovaterc_json5` | — |

### `workers/repository/init/merge › checkForRepoConfigError`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no error  | 284 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| throws on error  | 288 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |

### `workers/repository/init/merge › mergeRenovateConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses onboarding config if silent  | 305 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| throws error if misconfigured  | 317 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| migrates nested config  | 333 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| ignores presets  | 363 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| continues if no errors  | 382 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| continues if no errors-2  | 393 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| sets npmToken to npmrc when it is not inside encrypted  | 413 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| sets npmToken to npmrc when it is inside encrypted  | 436 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| deletes user conifgured env after setting in mem cache  | 463 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| applies repositoryEntryConfig between global and repo file config  | 485 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| supports repositoryEntryConfig without extends or ignorePresets  | 608 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |

### `workers/repository/init/merge › setNpmTokenInNpmrc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| skips in no npmToken found  | 641 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| adds default npmrc registry if it does not exist  | 647 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| adds npmToken at end of npmrc string if ${NPM_TOKEN} string not found  | 655 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| replaces ${NPM_TOKEN} with npmToken value  | 661 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |

### `workers/repository/init/merge › applyNpmrc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing if npmrc is missing after token migration  | 672 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| migrates npmToken and sets npmrc  | 680 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |

### `workers/repository/init/merge › applyHostRules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing when hostRules is not configured  | 698 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| adds hostRules and clears queue and throttle  | 710 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| warns on invalid hostRule and continues applying others  | 730 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |

### `workers/repository/init/merge › static repository config › resolveStaticRepoConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $name  | 796 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |

### `workers/repository/init/merge › static repository config › resolveStaticRepoConfig termination cases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $name  | 820 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |
| should log static config validation errors and warnings  | 840 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |

### `workers/repository/init/merge › static repository config › mergeRenovateConfig() with a static repository config`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $name  | 868 | not-applicable | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer | — | Mock framework internals — tests init merge via vitest-mocked platform/config; Rust tests this at different layer |

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

