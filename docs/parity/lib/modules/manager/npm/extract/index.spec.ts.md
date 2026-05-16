# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/extract/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/index.spec.ts
**Total tests:** 41 | **Ported:** 17 | **Actionable:** 17 | **Status:** ported

### `modules/manager/npm/extract/index › .extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if cannot parse | 38 | ported | `npm.rs` | `package_json_extract_returns_error_if_cannot_parse` | — |
| catches invalid names | 47 | ported | `npm.rs` | `package_json_invalid_dependency_names_are_skipped` | — |
| ignores vendorised package.json | 58 | ported | `npm.rs` | `package_json_vendorised_installed_package_is_ignored` | — |
| throws error if non-root renovate config | 67 | not-applicable | — | — | Requires package-file path wrapper validation; Rust package.json extractor is content-only |
| returns null if no deps | 77 | ported | `npm.rs` | `empty_package_json_returns_empty_list` | — |
| handles invalid | 86 | ported | `npm.rs` | `package_json_invalid_dependency_sections_return_empty` | — |
| returns an array of dependencies | 95 | ported | `npm.rs` | `package_json_fixture_extracts_dependency_array` | — |
| returns an array of dependencies with resolution comments | 122 | ported | `npm.rs` | `package_json_resolution_comments_are_invalid_names` | — |
| finds a lock file | 151 | not-applicable | — | — | Requires async sibling file reads and managerData lockfile wrapper; Rust package.json extractor is content-only |
| warns when multiple lock files found | 170 | not-applicable | — | — | Requires async sibling file reads, logging, and managerData lockfile wrapper; Rust package.json extractor is content-only |
| finds and filters .npmrc | 197 | not-applicable | — | — | Requires async .npmrc discovery/filtering wrapper; Rust package.json extractor is content-only |
| uses config.npmrc if no .npmrc is returned from search | 220 | not-applicable | — | — | Requires ExtractConfig npmrc merge wrapper; Rust package.json extractor has no config/npmrc API |
| uses config.npmrc if no .npmrc exists | 229 | not-applicable | — | — | Requires ExtractConfig npmrc merge wrapper; Rust package.json extractor has no config/npmrc API |
| uses config.npmrc if .npmrc does exist but npmrcMerge=false | 239 | not-applicable | — | — | Requires ExtractConfig npmrc merge wrapper and async file reads; Rust package.json extractor has no config/npmrc API |
| merges config.npmrc and repo .npmrc when npmrcMerge=true | 262 | not-applicable | — | — | Requires ExtractConfig npmrc merge wrapper and async file reads; Rust package.json extractor has no config/npmrc API |
| finds and filters .npmrc with variables | 285 | not-applicable | — | — | Requires async .npmrc discovery/filtering wrapper; Rust package.json extractor is content-only |
| reads registryUrls from .yarnrc.yml | 310 | not-applicable | — | — | Requires async .yarnrc.yml discovery integrated into package extraction; static Yarn registry parser is covered in yarnrc.spec.ts |
| reads registryUrls from .yarnrc | 338 | not-applicable | — | — | Requires async .yarnrc discovery integrated into package extraction; static Yarn registry parser is covered in yarnrc.spec.ts |
| resolves registry URLs using the package name if set | 365 | not-applicable | — | — | Requires async .yarnrc.yml discovery plus packageManager dependency integration; static registry resolution is covered in yarnrc.spec.ts |
| finds complex yarn workspaces | 398 | not-applicable | — | — | Requires workspace glob discovery and async filesystem reads; Rust package.json extractor is content-only |
| extracts engines | 412 | ported | `npm.rs` | `package_json_extracts_engines` | — |
| extracts volta | 503 | ported | `npm.rs` | `package_json_extracts_volta` | — |
| extracts volta yarn unspecified-version | 543 | ported | `npm.rs` | `package_json_extracts_volta_yarn_unspecified` | — |
| extracts volta yarn higher than 1 | 584 | ported | `npm.rs` | `package_json_extracts_volta_yarn_higher_than_one` | — |
| extracts non-npmjs | 626 | ported | `npm.rs` | `package_json_extracts_non_npmjs_github_dependencies` | — |
| does not set registryUrls for non-npmjs | 760 | not-applicable | — | — | Requires package-file registryUrls metadata; Rust package.json extractor returns dependencies only |
| extracts npm package alias | 815 | ported | `npm.rs` | `npm_aliases_are_extracted` | — |
| sets skipInstalls false if Yarn zero-install is used | 866 | not-applicable | — | — | Requires async lockfile/.yarnrc.yml discovery and install-strategy managerData wrapper; Rust package.json extractor is content-only |
| extracts packageManager | 894 | ported | `npm.rs` | `package_json_extracts_package_manager` | — |
| sets hasPackageManager to true when devEngines detected in package file | 923 | not-applicable | — | — | Requires package-file managerData; Rust package.json extractor returns dependencies only |
| extracts dependencies from overrides | 957 | ported | `npm.rs` | `extracts_npm_overrides` | — |
| extracts dependencies from pnpm.overrides | 1036 | ported | `npm.rs` | `extracts_pnpm_overrides` | — |
| extracts dependencies from pnpm.overrides, with version ranges in flat syntax | 1117 | ported | `npm.rs` | `extracts_pnpm_override_range_keys` | — |

### `modules/manager/npm/extract/index › .extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs | 1200 | not-applicable | — | — | Requires Renovate `extractAllPackageFiles()` async multi-file manager wrapper; Rust npm extraction is content-level |
| warns for invalid pnpm workspace yaml files | 1250 | not-applicable | — | — | Requires Renovate `extractAllPackageFiles()` async multi-file manager wrapper and logging |
| parses empty pnpm workspace yaml files | 1267 | not-applicable | — | — | Requires Renovate `extractAllPackageFiles()` async multi-file manager wrapper |
| extracts pnpm workspace yaml files | 1276 | not-applicable | — | — | Requires Renovate `extractAllPackageFiles()` async multi-file manager wrapper; static pnpm workspace helper is covered in pnpm.spec.ts |
| extracts yarnrc.yml and adds it as packageFile | 1306 | not-applicable | — | — | Requires Renovate `extractAllPackageFiles()` async multi-file manager wrapper; static Yarn catalog helper is covered in yarn.spec.ts |
| extracts yarnrc.yml and adds it as packageFile and packageManager to true | 1340 | not-applicable | — | — | Requires Renovate `extractAllPackageFiles()` async multi-file manager wrapper and packageManager cross-file detection |
| extracts yarnrc.yml and adds it as packageFile and packageManager to false if no deps | 1372 | not-applicable | — | — | Requires Renovate `extractAllPackageFiles()` async multi-file manager wrapper and packageManager cross-file detection |

### `modules/manager/npm/extract/index › .postExtract()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| runs | 1409 | not-applicable | — | — | Renovate postExtract hook is a no-op async manager hook; no Rust equivalent hook exists |

---

