# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/extract/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/index.spec.ts
**Total tests:** 41 | **Ported:** 17 | **Actionable:** 24 | **Status:** partial

### `modules/manager/npm/extract/index › .extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if cannot parse | 38 | ported | `npm.rs` | `package_json_extract_returns_error_if_cannot_parse` | — |
| catches invalid names | 47 | ported | `npm.rs` | `package_json_invalid_dependency_names_are_skipped` | — |
| ignores vendorised package.json | 58 | ported | `npm.rs` | `package_json_vendorised_installed_package_is_ignored` | — |
| throws error if non-root renovate config | 67 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| returns null if no deps | 77 | ported | `npm.rs` | `empty_package_json_returns_empty_list` | — |
| handles invalid | 86 | ported | `npm.rs` | `package_json_invalid_dependency_sections_return_empty` | — |
| returns an array of dependencies | 95 | ported | `npm.rs` | `package_json_fixture_extracts_dependency_array` | — |
| returns an array of dependencies with resolution comments | 122 | ported | `npm.rs` | `package_json_resolution_comments_are_invalid_names` | — |
| finds a lock file | 151 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| warns when multiple lock files found | 170 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| finds and filters .npmrc | 197 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| uses config.npmrc if no .npmrc is returned from search | 220 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| uses config.npmrc if no .npmrc exists | 229 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| uses config.npmrc if .npmrc does exist but npmrcMerge=false | 239 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| merges config.npmrc and repo .npmrc when npmrcMerge=true | 262 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| finds and filters .npmrc with variables | 285 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| reads registryUrls from .yarnrc.yml | 310 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| reads registryUrls from .yarnrc | 338 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| resolves registry URLs using the package name if set | 365 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| finds complex yarn workspaces | 398 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| extracts engines | 412 | ported | `npm.rs` | `package_json_extracts_engines` | — |
| extracts volta | 503 | ported | `npm.rs` | `package_json_extracts_volta` | — |
| extracts volta yarn unspecified-version | 543 | ported | `npm.rs` | `package_json_extracts_volta_yarn_unspecified` | — |
| extracts volta yarn higher than 1 | 584 | ported | `npm.rs` | `package_json_extracts_volta_yarn_higher_than_one` | — |
| extracts non-npmjs | 626 | ported | `npm.rs` | `package_json_extracts_non_npmjs_github_dependencies` | — |
| does not set registryUrls for non-npmjs | 760 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| extracts npm package alias | 815 | ported | `npm.rs` | `npm_aliases_are_extracted` | — |
| sets skipInstalls false if Yarn zero-install is used | 866 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| extracts packageManager | 894 | ported | `npm.rs` | `package_json_extracts_package_manager` | — |
| sets hasPackageManager to true when devEngines detected in package file | 923 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| extracts dependencies from overrides | 957 | ported | `npm.rs` | `extracts_npm_overrides` | — |
| extracts dependencies from pnpm.overrides | 1036 | ported | `npm.rs` | `extracts_pnpm_overrides` | — |
| extracts dependencies from pnpm.overrides, with version ranges in flat syntax | 1117 | ported | `npm.rs` | `extracts_pnpm_override_range_keys` | — |

### `modules/manager/npm/extract/index › .extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs | 1200 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| warns for invalid pnpm workspace yaml files | 1250 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| parses empty pnpm workspace yaml files | 1267 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| extracts pnpm workspace yaml files | 1276 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| extracts yarnrc.yml and adds it as packageFile | 1306 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| extracts yarnrc.yml and adds it as packageFile and packageManager to true | 1340 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|
| extracts yarnrc.yml and adds it as packageFile and packageManager to false if no deps | 1372 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|

### `modules/manager/npm/extract/index › .postExtract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs | 1409 | not-applicable | Mock framework internals — tests npm extract via vitest-mocked fs; Rust tests this at different layer | — | —|

---
