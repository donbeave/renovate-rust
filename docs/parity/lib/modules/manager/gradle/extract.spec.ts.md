# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gradle/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle/extract.spec.ts
**Total tests:** 31 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 37 | ported | `gradle.rs` | `empty_returns_empty` | — |
| logs a warning in case parseGradle throws an exception | 52 | not-applicable | — | — | Tests warning log output; no Rust equivalent |
| skips versions composed from multiple variables | 71 | ported | `gradle.rs` | `skips_variable_references` | — |
| extracts from cross-referenced files | 97 | not-applicable | — | — | Requires cross-file extraction; single-file extractor only |
| resolves versions in build.gradle.kts | 125 | not-applicable | — | — | Requires cross-file Kotlin object resolution |
| resolves cross-file Kotlin objects | 191 | not-applicable | — | — | Requires cross-file analysis |
| inherits gradle variables | 311 | not-applicable | — | — | Requires multi-file variable inheritance |
| filters duplicate dependency findings | 341 | ported | `gradle.rs` | `deduplicates_same_dep` | — |
| ensures depType is assigned | 385 | not-applicable | — | — | Requires multi-file extraction with filesystem mock |

### `extractPackageFile() › registry URLs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deduplicates registry urls | 414 | not-applicable | — | — | Requires multi-file repository registry resolution |
| interpolates registry URLs | 451 | not-applicable | — | — | Requires multi-file repository registry resolution |
| supports separate registry URLs for plugins | 507 | not-applicable | — | — | Requires multi-file repository registry resolution |

### `extractPackageFile() › registry URLs › content descriptors › simple descriptor matches`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input \| $output | 568 | not-applicable | — | — | Tests TypeScript-internal matchesContentDescriptor utility |

### `extractPackageFile() › registry URLs › content descriptors › multiple descriptors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| if both includes and excludes exist, dep must match include and not match exclude | 609 | not-applicable | — | — | Tests configuration filtering logic |
| if only includes exist, dep must match at least one include | 635 | not-applicable | — | — | Tests configuration filtering logic |
| if only excludes exist, dep must match not match any exclude | 653 | not-applicable | — | — | Tests configuration filtering logic |

### `extractPackageFile() › registry URLs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts content descriptors | 672 | not-applicable | — | — | Requires multi-file repository registry resolution |
| exclusiveContent | 775 | not-applicable | — | — | Requires multi-file repository registry resolution |
| exclusiveContent with repeated repository definition | 823 | not-applicable | — | — | Requires multi-file repository registry resolution |

### `extractPackageFile() › version catalogs`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works with dependency catalogs | 889 | not-applicable | — | — | Requires cross-file version catalog resolution |
| provides versions from external version catalogs to gradle files | 1006 | not-applicable | — | — | Requires cross-file version catalog resolution |
| provides versions to gradle files with changed default catalog name | 1061 | not-applicable | — | — | Requires cross-file version catalog resolution |
| ignores version catalog accessor with non-get provider method | 1106 | not-applicable | — | — | Requires multi-file extractAllPackageFiles with cross-file version catalog resolution |
| aligns sharedVariableName if version reference has multiple aliases | 1127 | not-applicable | — | — | Requires multi-file extractAllPackageFiles with cross-file version catalog resolution |

### `extractPackageFile() › apply from`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| loads further scripts using apply from statements | 1175 | not-applicable | — | — | Requires multi-file extractAllPackageFiles; apply from resolution crosses file boundaries |
| works with files in sub-directories | 1269 | not-applicable | — | — | Requires multi-file extractAllPackageFiles; subdirectory traversal |
| prevents recursive apply from calls | 1304 | not-applicable | — | — | Requires multi-file extractAllPackageFiles; circular dependency detection |
| prevents inclusion of non-Gradle files | 1319 | not-applicable | — | — | Requires multi-file extractAllPackageFiles |

### `extractPackageFile() › gradle-consistent-versions plugin`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses versions files | 1335 | not-applicable | — | — | Requires multi-file: versions.props + versions.lock |
| plugin not used due to lockfile not a GCV lockfile | 1385 | not-applicable | — | — | Requires multi-file lockfile detection |
| plugin not used due to lockfile missing | 1401 | not-applicable | — | — | Requires multi-file lockfile detection |

---

