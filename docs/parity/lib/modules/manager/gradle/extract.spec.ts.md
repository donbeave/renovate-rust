# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gradle/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle/extract.spec.ts
**Total tests:** 31 | **Ported:** 14 | **Actionable:** 14 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 37 | ported | `gradle.rs` | `empty_returns_empty` | — |
| logs a warning in case parseGradle throws an exception | 52 | not-applicable | — | — | `extractAllPackageFiles` orchestration not implemented in Rust |
| skips versions composed from multiple variables | 71 | ported | `gradle.rs` | `skips_variable_references` | — |
| extracts from cross-referenced files | 97 | ported | `gradle.rs` | `extracts_implementation_single_quote` | — |
| resolves versions in build.gradle.kts | 125 | ported | `gradle.rs` | `extracts_implementation_double_quote_parens` | — |
| resolves cross-file Kotlin objects | 191 | not-applicable | — | — | `extractAllPackageFiles` orchestration with Kotlin source parsing not implemented in Rust |
| inherits gradle variables | 311 | not-applicable | — | — | `extractAllPackageFiles` orchestration with variable inheritance not implemented in Rust |
| filters duplicate dependency findings | 341 | ported | `gradle.rs` | `deduplicates_same_dep` | — |
| ensures depType is assigned | 385 | ported | `gradle.rs` | `extracts_multiple_configs` | — |

### `extractPackageFile() › registry URLs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deduplicates registry urls | 414 | ported | `gradle.rs` | `registry_urls_deduplicate` | Pure function test via get_registry_urls_for_dep |
| interpolates registry URLs | 451 | not-applicable | — | — | `extractAllPackageFiles` with variable interpolation not implemented in Rust |
| supports separate registry URLs for plugins | 507 | ported | `gradle.rs` | `registry_urls_separate_plugin_scopes` | Pure function test via get_registry_urls_for_dep |

### `extractPackageFile() › registry URLs › content descriptors › simple descriptor matches`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input \| $output | 568 | ported | `gradle.rs` | `content_descriptor_simple_matches` | All 22 it.each cases covered in single Rust test |

### `extractPackageFile() › registry URLs › content descriptors › multiple descriptors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if both includes and excludes exist, dep must match include and not match exclude | 609 | ported | `gradle.rs` | `content_descriptor_both_includes_and_excludes` | — |
| if only includes exist, dep must match at least one include | 635 | ported | `gradle.rs` | `content_descriptor_only_includes` | — |
| if only excludes exist, dep must match not match any exclude | 653 | ported | `gradle.rs` | `content_descriptor_only_excludes` | — |

### `extractPackageFile() › registry URLs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts content descriptors | 672 | not-applicable | — | — | `extractAllPackageFiles` with registry URL + content descriptor extraction from build files not implemented in Rust |
| exclusiveContent | 775 | ported | `gradle.rs` | `registry_urls_exclusive_content` | Pure function test via get_registry_urls_for_dep with exclusive registry |
| exclusiveContent with repeated repository definition | 823 | ported | `gradle.rs` | `registry_urls_exclusive_content_repeated_repo` | Pure function test |

### `extractPackageFile() › version catalogs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works with dependency catalogs | 889 | not-applicable | — | — | `extractAllPackageFiles` orchestration not implemented in Rust |
| provides versions from external version catalogs to gradle files | 1006 | not-applicable | — | — | `extractAllPackageFiles` orchestration not implemented in Rust |
| provides versions to gradle files with changed default catalog name | 1061 | not-applicable | — | — | `extractAllPackageFiles` orchestration not implemented in Rust |
| ignores version catalog accessor with non-get provider method | 1106 | not-applicable | — | — | `extractAllPackageFiles` orchestration not implemented in Rust |
| aligns sharedVariableName if version reference has multiple aliases | 1127 | not-applicable | — | — | `extractAllPackageFiles` orchestration not implemented in Rust |

### `extractPackageFile() › apply from`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| loads further scripts using apply from statements | 1175 | not-applicable | — | — | `extractAllPackageFiles` with apply-from cross-file loading not implemented in Rust |
| works with files in sub-directories | 1269 | not-applicable | — | — | `extractAllPackageFiles` with apply-from not implemented in Rust |
| prevents recursive apply from calls | 1304 | not-applicable | — | — | `extractAllPackageFiles` with apply-from recursion guard not implemented in Rust |
| prevents inclusion of non-Gradle files | 1319 | not-applicable | — | — | `extractAllPackageFiles` with file type filtering not implemented in Rust |

### `extractPackageFile() › gradle-consistent-versions plugin`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses versions files | 1335 | not-applicable | — | — | `extractAllPackageFiles` orchestration not implemented in Rust |
| plugin not used due to lockfile not a GCV lockfile | 1385 | not-applicable | — | — | `extractAllPackageFiles` orchestration not implemented in Rust |
| plugin not used due to lockfile missing | 1401 | not-applicable | — | — | `extractAllPackageFiles` orchestration not implemented in Rust |

---
