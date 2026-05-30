# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gradle/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle/extract.spec.ts
**Total tests:** 31 | **Ported:** 6 | **Actionable:** 31 | **Status:** partial

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 37 | ported | `gradle.rs` | `empty_returns_empty` | — |
| logs a warning in case parseGradle throws an exception | 52 | pending | — | — | —|
| skips versions composed from multiple variables | 71 | ported | `gradle.rs` | `skips_variable_references` | — |
| extracts from cross-referenced files | 97 | ported | `gradle.rs` | `extracts_implementation_single_quote` | — |
| resolves versions in build.gradle.kts | 125 | ported | `gradle.rs` | `extracts_implementation_double_quote_parens` | — |
| resolves cross-file Kotlin objects | 191 | pending | — | — | —|
| inherits gradle variables | 311 | pending | — | — | —|
| filters duplicate dependency findings | 341 | ported | `gradle.rs` | `deduplicates_same_dep` | — |
| ensures depType is assigned | 385 | ported | `gradle.rs` | `extracts_multiple_configs` | — |

### `extractPackageFile() › registry URLs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deduplicates registry urls | 414 | pending | — | — | —|
| interpolates registry URLs | 451 | pending | — | — | —|
| supports separate registry URLs for plugins | 507 | pending | — | — | —|

### `extractPackageFile() › registry URLs › content descriptors › simple descriptor matches`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input \| $output | 568 | pending | — | — | —|

### `extractPackageFile() › registry URLs › content descriptors › multiple descriptors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if both includes and excludes exist, dep must match include and not match exclude | 609 | pending | — | — | —|
| if only includes exist, dep must match at least one include | 635 | pending | — | — | —|
| if only excludes exist, dep must match not match any exclude | 653 | pending | — | — | —|

### `extractPackageFile() › registry URLs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts content descriptors | 672 | pending | — | — | —|
| exclusiveContent | 775 | pending | — | — | —|
| exclusiveContent with repeated repository definition | 823 | pending | — | — | —|

### `extractPackageFile() › version catalogs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works with dependency catalogs | 889 | pending | — | — | —|
| provides versions from external version catalogs to gradle files | 1006 | pending | — | — | —|
| provides versions to gradle files with changed default catalog name | 1061 | pending | — | — | —|
| ignores version catalog accessor with non-get provider method | 1106 | pending | — | — | —|
| aligns sharedVariableName if version reference has multiple aliases | 1127 | pending | — | — | —|

### `extractPackageFile() › apply from`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| loads further scripts using apply from statements | 1175 | pending | — | — | —|
| works with files in sub-directories | 1269 | pending | — | — | —|
| prevents recursive apply from calls | 1304 | pending | — | — | —|
| prevents inclusion of non-Gradle files | 1319 | pending | — | — | —|

### `extractPackageFile() › gradle-consistent-versions plugin`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses versions files | 1335 | pending | — | — | —|
| plugin not used due to lockfile not a GCV lockfile | 1385 | pending | — | — | —|
| plugin not used due to lockfile missing | 1401 | pending | — | — | —|

---
