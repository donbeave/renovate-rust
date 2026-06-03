# `lib/modules/manager/gradle/extract.spec.ts`

[← `manager/gradle`](../../../../_by-module/manager/gradle.md) · [all modules](../../../../README.md)

**14/31 ported** (17 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 37 | returns null | ported | [`crates/renovate-core/src/extractors/gradle.rs:2106`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2106) |
| 52 | logs a warning in case parsegradle throws an exception | pending | — |
| 71 | skips versions composed from multiple variables | ported | [`crates/renovate-core/src/extractors/gradle.rs:1692`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L1692) |
| 97 | extracts from cross-referenced files | ported | [`crates/renovate-core/src/extractors/gradle.rs:1647`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L1647) |
| 125 | resolves versions in build.gradle.kts | ported | [`crates/renovate-core/src/extractors/gradle.rs:1658`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L1658) |
| 191 | resolves cross-file kotlin objects | pending | — |
| 311 | inherits gradle variables | pending | — |
| 341 | filters duplicate dependency findings | ported | [`crates/renovate-core/src/extractors/gradle.rs:1732`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L1732) |
| 385 | ensures deptype is assigned | ported | [`crates/renovate-core/src/extractors/gradle.rs:1667`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L1667) |
| 414 | deduplicates registry urls | ported | [`crates/renovate-core/src/extractors/gradle.rs:3224`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L3224) |
| 451 | interpolates registry urls | pending | — |
| 507 | supports separate registry urls for plugins | ported | [`crates/renovate-core/src/extractors/gradle.rs:3283`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L3283) |
| 568 | _(it.each / template — verify manually)_ | ? | — |
| 609 | if both includes and excludes exist, dep must match include and not match exclude | ported | [`crates/renovate-core/src/extractors/gradle.rs:3080`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L3080) |
| 635 | if only includes exist, dep must match at least one include | ported | [`crates/renovate-core/src/extractors/gradle.rs:3124`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L3124) |
| 653 | if only excludes exist, dep must match not match any exclude | ported | [`crates/renovate-core/src/extractors/gradle.rs:3174`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L3174) |
| 672 | extracts content descriptors | pending | — |
| 775 | exclusivecontent | ported | [`crates/renovate-core/src/extractors/gradle.rs:3330`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L3330) |
| 823 | exclusivecontent with repeated repository definition | ported | [`crates/renovate-core/src/extractors/gradle.rs:3369`](../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L3369) |
| 889 | works with dependency catalogs | pending | — |
| 1006 | provides versions from external version catalogs to gradle files | pending | — |
| 1061 | provides versions to gradle files with changed default catalog name | pending | — |
| 1106 | ignores version catalog accessor with non-get provider method | pending | — |
| 1127 | aligns sharedvariablename if version reference has multiple aliases | pending | — |
| 1175 | loads further scripts using apply from statements | pending | — |
| 1269 | works with files in sub-directories | pending | — |
| 1304 | prevents recursive apply from calls | pending | — |
| 1319 | prevents inclusion of non-gradle files | pending | — |
| 1335 | parses versions files | pending | — |
| 1385 | plugin not used due to lockfile not a gcv lockfile | pending | — |
| 1401 | plugin not used due to lockfile missing | pending | — |

