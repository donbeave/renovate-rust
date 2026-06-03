# `lib/modules/manager/ant/extract.spec.ts`

[← `manager/ant`](../../../../_by-module/manager/ant.md) · [all modules](../../../../README.md)

**49/49 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 13 | extracts inline version dependencies from build.xml | ported | `crates/renovate-core/src/extractors/ant.rs:863` |
| 37 | extracts multiple dependencies | ported | `crates/renovate-core/src/extractors/ant.rs:1188` |
| 72 | defaults deptype to compile when no scope is set | ported | `crates/renovate-core/src/extractors/ant.rs:1173` |
| 94 | returns null for invalid xml | ported | `crates/renovate-core/src/extractors/ant.rs:1369` |
| 98 | returns null for build.xml with no dependencies | ported | `crates/renovate-core/src/extractors/ant.rs:1512` |
| 108 | ignores dependency nodes without version | ported | `crates/renovate-core/src/extractors/ant.rs:1519` |
| 123 | extracts dependencies with single-quoted attributes | ported | `crates/renovate-core/src/extractors/ant.rs:1537` |
| 139 | returns null for unreadable build.xml | ported | `crates/renovate-core/src/extractors/ant.rs:1547` |
| 147 | does not revisit the same file | ported | `crates/renovate-core/src/extractors/ant.rs:1554` |
| 171 | resolves inline property references | ported | `crates/renovate-core/src/extractors/ant.rs:909` |
| 197 | resolves properties from external .properties files | ported | `crates/renovate-core/src/extractors/ant.rs:930` |
| 232 | implements first-definition-wins for inline properties | ported | `crates/renovate-core/src/extractors/ant.rs:956` |
| 258 | inline properties take precedence over file properties | ported | `crates/renovate-core/src/extractors/ant.rs:976` |
| 292 | skips dependencies with unresolvable property references | ported | `crates/renovate-core/src/extractors/ant.rs:895` |
| 316 | detects circular property references | ported | `crates/renovate-core/src/extractors/ant.rs:1001` |
| 342 | resolves chained property references | ported | `crates/renovate-core/src/extractors/ant.rs:1020` |
| 372 | groups multiple dependencies sharing the same property | ported | `crates/renovate-core/src/extractors/ant.rs:1042` |
| 404 | handles properties file in subdirectory | ported | `crates/renovate-core/src/extractors/ant.rs:1062` |
| 438 | handles unreadable properties file gracefully | ported | `crates/renovate-core/src/extractors/ant.rs:1092` |
| 468 | returns deps with mixed inline and property versions | ported | `crates/renovate-core/src/extractors/ant.rs:1112` |
| 499 | ignores dependency without version during property resolution | ported | `crates/renovate-core/src/extractors/ant.rs:1136` |
| 526 | skips partial placeholder in version string | ported | `crates/renovate-core/src/extractors/ant.rs:1157` |
| 553 | handles unparseable xml returned by readlocalfile | ported | `crates/renovate-core/src/extractors/ant.rs:1375` |
| 561 | handles absolute path in property file reference | ported | `crates/renovate-core/src/extractors/ant.rs:1381` |
| 595 | skips duplicate property file references | ported | `crates/renovate-core/src/extractors/ant.rs:1407` |
| 632 | follows import file references | ported | `crates/renovate-core/src/extractors/ant.rs:1429` |
| 666 | skips missing import files | ported | `crates/renovate-core/src/extractors/ant.rs:1450` |
| 696 | does not loop on self-importing files | ported | `crates/renovate-core/src/extractors/ant.rs:1468` |
| 726 | shares properties across imported files | ported | `crates/renovate-core/src/extractors/ant.rs:1486` |
| 764 | extracts dependency from 3-part coords attribute | ported | `crates/renovate-core/src/extractors/ant.rs:880` |
| 795 | extracts scope from 4-part coords attribute | ported | `crates/renovate-core/src/extractors/ant.rs:1572` |
| 825 | ignores coords with fewer than 3 parts | ported | `crates/renovate-core/src/extractors/ant.rs:1609` |
| 844 | ignores coords with empty groupid | ported | `crates/renovate-core/src/extractors/ant.rs:1621` |
| 863 | resolves property references in coords version | ported | `crates/renovate-core/src/extractors/ant.rs:1588` |
| 894 | marks coords dependency with unresolvable property | ported | `crates/renovate-core/src/extractors/ant.rs:1633` |
| 923 | treats last part as version when it is not a known scope | ported | `crates/renovate-core/src/extractors/ant.rs:1648` |
| 953 | collects registry urls from remoterepository elements | ported | `crates/renovate-core/src/extractors/ant.rs:1208` |
| 983 | passes registry urls to coords-style dependencies | ported | `crates/renovate-core/src/extractors/ant.rs:1223` |
| 1013 | collects registry urls from settingsfile attribute | ported | `crates/renovate-core/src/extractors/ant.rs:1242` |
| 1051 | merges registries from settingsfile and remoterepository | ported | `crates/renovate-core/src/extractors/ant.rs:1271` |
| 1093 | handles absolute settingsfile path | ported | `crates/renovate-core/src/extractors/ant.rs:1303` |
| 1131 | logs debug when settingsfile cannot be read | ported | `crates/renovate-core/src/extractors/ant.rs:1331` |
| 1159 | does not pass registries to dependencies outside the block | ported | `crates/renovate-core/src/extractors/ant.rs:1347` |
| 1195 | skips property file references with unresolved placeholders in path | ported | `crates/renovate-core/src/extractors/ant.rs:1855` |
| 1227 | skips property file references that resolve outside the repository | ported | `crates/renovate-core/src/extractors/ant.rs:1871` |
| 1262 | skips import file references that resolve outside the repository | ported | `crates/renovate-core/src/extractors/ant.rs:1886` |
| 1297 | skips settingsfile references that resolve outside the repository | ported | `crates/renovate-core/src/extractors/ant.rs:1901` |
| 1331 | skips import file references with unresolved placeholders in path | ported | `crates/renovate-core/src/extractors/ant.rs:1915` |
| 1363 | handles chain referencing undefined property | ported | `crates/renovate-core/src/extractors/ant.rs:1667` |

