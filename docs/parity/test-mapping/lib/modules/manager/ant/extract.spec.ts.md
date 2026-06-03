# `lib/modules/manager/ant/extract.spec.ts`

[← `manager/ant`](../../../../_by-module/manager/ant.md) · [all modules](../../../../README.md)

**49/49 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 13 | extracts inline version dependencies from build.xml | ported | [`crates/renovate-core/src/extractors/ant.rs:863`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L863) |
| 37 | extracts multiple dependencies | ported | [`crates/renovate-core/src/extractors/ant.rs:1188`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1188) |
| 72 | defaults deptype to compile when no scope is set | ported | [`crates/renovate-core/src/extractors/ant.rs:1173`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1173) |
| 94 | returns null for invalid xml | ported | [`crates/renovate-core/src/extractors/ant.rs:1369`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1369) |
| 98 | returns null for build.xml with no dependencies | ported | [`crates/renovate-core/src/extractors/ant.rs:1512`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1512) |
| 108 | ignores dependency nodes without version | ported | [`crates/renovate-core/src/extractors/ant.rs:1519`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1519) |
| 123 | extracts dependencies with single-quoted attributes | ported | [`crates/renovate-core/src/extractors/ant.rs:1537`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1537) |
| 139 | returns null for unreadable build.xml | ported | [`crates/renovate-core/src/extractors/ant.rs:1547`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1547) |
| 147 | does not revisit the same file | ported | [`crates/renovate-core/src/extractors/ant.rs:1554`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1554) |
| 171 | resolves inline property references | ported | [`crates/renovate-core/src/extractors/ant.rs:909`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L909) |
| 197 | resolves properties from external .properties files | ported | [`crates/renovate-core/src/extractors/ant.rs:930`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L930) |
| 232 | implements first-definition-wins for inline properties | ported | [`crates/renovate-core/src/extractors/ant.rs:956`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L956) |
| 258 | inline properties take precedence over file properties | ported | [`crates/renovate-core/src/extractors/ant.rs:976`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L976) |
| 292 | skips dependencies with unresolvable property references | ported | [`crates/renovate-core/src/extractors/ant.rs:895`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L895) |
| 316 | detects circular property references | ported | [`crates/renovate-core/src/extractors/ant.rs:1001`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1001) |
| 342 | resolves chained property references | ported | [`crates/renovate-core/src/extractors/ant.rs:1020`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1020) |
| 372 | groups multiple dependencies sharing the same property | ported | [`crates/renovate-core/src/extractors/ant.rs:1042`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1042) |
| 404 | handles properties file in subdirectory | ported | [`crates/renovate-core/src/extractors/ant.rs:1062`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1062) |
| 438 | handles unreadable properties file gracefully | ported | [`crates/renovate-core/src/extractors/ant.rs:1092`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1092) |
| 468 | returns deps with mixed inline and property versions | ported | [`crates/renovate-core/src/extractors/ant.rs:1112`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1112) |
| 499 | ignores dependency without version during property resolution | ported | [`crates/renovate-core/src/extractors/ant.rs:1136`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1136) |
| 526 | skips partial placeholder in version string | ported | [`crates/renovate-core/src/extractors/ant.rs:1157`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1157) |
| 553 | handles unparseable xml returned by readlocalfile | ported | [`crates/renovate-core/src/extractors/ant.rs:1375`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1375) |
| 561 | handles absolute path in property file reference | ported | [`crates/renovate-core/src/extractors/ant.rs:1381`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1381) |
| 595 | skips duplicate property file references | ported | [`crates/renovate-core/src/extractors/ant.rs:1407`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1407) |
| 632 | follows import file references | ported | [`crates/renovate-core/src/extractors/ant.rs:1429`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1429) |
| 666 | skips missing import files | ported | [`crates/renovate-core/src/extractors/ant.rs:1450`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1450) |
| 696 | does not loop on self-importing files | ported | [`crates/renovate-core/src/extractors/ant.rs:1468`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1468) |
| 726 | shares properties across imported files | ported | [`crates/renovate-core/src/extractors/ant.rs:1486`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1486) |
| 764 | extracts dependency from 3-part coords attribute | ported | [`crates/renovate-core/src/extractors/ant.rs:880`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L880) |
| 795 | extracts scope from 4-part coords attribute | ported | [`crates/renovate-core/src/extractors/ant.rs:1572`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1572) |
| 825 | ignores coords with fewer than 3 parts | ported | [`crates/renovate-core/src/extractors/ant.rs:1609`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1609) |
| 844 | ignores coords with empty groupid | ported | [`crates/renovate-core/src/extractors/ant.rs:1621`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1621) |
| 863 | resolves property references in coords version | ported | [`crates/renovate-core/src/extractors/ant.rs:1588`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1588) |
| 894 | marks coords dependency with unresolvable property | ported | [`crates/renovate-core/src/extractors/ant.rs:1633`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1633) |
| 923 | treats last part as version when it is not a known scope | ported | [`crates/renovate-core/src/extractors/ant.rs:1648`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1648) |
| 953 | collects registry urls from remoterepository elements | ported | [`crates/renovate-core/src/extractors/ant.rs:1208`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1208) |
| 983 | passes registry urls to coords-style dependencies | ported | [`crates/renovate-core/src/extractors/ant.rs:1223`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1223) |
| 1013 | collects registry urls from settingsfile attribute | ported | [`crates/renovate-core/src/extractors/ant.rs:1242`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1242) |
| 1051 | merges registries from settingsfile and remoterepository | ported | [`crates/renovate-core/src/extractors/ant.rs:1271`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1271) |
| 1093 | handles absolute settingsfile path | ported | [`crates/renovate-core/src/extractors/ant.rs:1303`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1303) |
| 1131 | logs debug when settingsfile cannot be read | ported | [`crates/renovate-core/src/extractors/ant.rs:1331`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1331) |
| 1159 | does not pass registries to dependencies outside the block | ported | [`crates/renovate-core/src/extractors/ant.rs:1347`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1347) |
| 1195 | skips property file references with unresolved placeholders in path | ported | [`crates/renovate-core/src/extractors/ant.rs:1855`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1855) |
| 1227 | skips property file references that resolve outside the repository | ported | [`crates/renovate-core/src/extractors/ant.rs:1871`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1871) |
| 1262 | skips import file references that resolve outside the repository | ported | [`crates/renovate-core/src/extractors/ant.rs:1886`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1886) |
| 1297 | skips settingsfile references that resolve outside the repository | ported | [`crates/renovate-core/src/extractors/ant.rs:1901`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1901) |
| 1331 | skips import file references with unresolved placeholders in path | ported | [`crates/renovate-core/src/extractors/ant.rs:1915`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1915) |
| 1363 | handles chain referencing undefined property | ported | [`crates/renovate-core/src/extractors/ant.rs:1667`](../../../../../../../crates/renovate-core/src/extractors/ant.rs#L1667) |

