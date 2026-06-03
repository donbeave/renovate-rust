# `lib/workers/repository/updates/generate.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**0/58 ported** (58 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 31 | groups single upgrade by default | pending | — |
| 53 | groups single upgrade across multiple files | pending | — |
| 90 | does not group single upgrade when groupsingleupdates is false | pending | — |
| 110 | does not group single upgrade without groupname even with groupsingleupdates | pending | — |
| 128 | handles lockfilemaintenance | pending | — |
| 153 | sets minimumgroupsize based on upgrades | pending | — |
| 182 | handles lockfileupdate | pending | — |
| 223 | does not group same upgrades | pending | — |
| 251 | groups multiple upgrades same version | pending | — |
| 322 | groups major updates with different versions but same newvalue, no recreatewhen | pending | — |
| 356 | groups multiple digest updates immortally | pending | — |
| 386 | recreates grouped pin & pindigest | pending | — |
| 408 | does not recreate grouped pin & pindigest when closed if recreatewhen=never | pending | — |
| 432 | recreates grouped pin | pending | — |
| 459 | recreates grouped pindigest | pending | — |
| 485 | skips appending basebranch and updatetype to prtitle when prtitlestrict is true | pending | — |
| 538 | groups multiple upgrades different version | pending | — |
| 589 | groups multiple upgrades different version but same value | pending | — |
| 629 | groups multiple upgrades different value but same version | pending | — |
| 669 | groups multiple digest updates | pending | — |
| 705 | pins digest to table | pending | — |
| 723 | fixes different messages | pending | — |
| 760 | uses semantic commits | pending | — |
| 787 | calculates the highest priority semanticcommittype | pending | — |
| 835 | scopes monorepo commits | pending | — |
| 862 | scopes monorepo commits with nested package files using parent directory | pending | — |
| 892 | scopes monorepo commits with nested package files using base directory | pending | — |
| 921 | use prettyversion in pr title when there is a v | pending | — |
| 948 | use prettyversion in pr title there is no v | pending | — |
| 975 | use newmajor in pr title with v | pending | — |
| 1000 | default commitmessageextra pr title | pending | — |
| 1026 | adds commit message body | pending | — |
| 1044 | supports manual prtitle | pending | — |
| 1060 | handles @types specially | pending | — |
| 1125 | handles @types specially (reversed) | pending | — |
| 1186 | handles upgrades | pending | — |
| 1333 | combines prbodycolumns | pending | — |
| 1350 | sorts upgrades, without position first | pending | — |
| 1391 | passes through pendingchecks | pending | — |
| 1415 | filters pendingchecks | pending | — |
| 1438 | displays pending versions | pending | — |
| 1472 | merge excludecommitpaths if appears in upgrade | pending | — |
| 1505 | generates pretty version name properly | pending | — |
| 1529 | prevents issue with duplicating "v" character | pending | — |
| 1542 | apply semanticcommits and commitmessageprefix together | pending | — |
| 1562 | dedupes duplicate table rows | pending | — |
| 1625 | using commitmessageprefix without separator | pending | — |
| 1642 | merges additionalreviewers | pending | — |
| 1666 | merges deptypes | pending | — |
| 1691 | deptypes is available on each branch upgrade object | pending | — |
| 1721 | allows upgrades in commitmessage | pending | — |
| 1746 | allows upgrades in commitmessage (group) | pending | — |
| 1801 | sets skipartifactsupdate to false when no upgrades specify a value | pending | — |
| 1849 | sets skipartifactsupdate to true when all upgrades specify true | pending | — |
| 1900 | _(it.each / template — verify manually)_ | ? | — |
| 1964 | uses prettydeptype when already set | pending | — |
| 1978 | falls back to deptype when prettydeptype is not set | pending | — |
| 1991 | defaults prettydeptype to dependency when neither prettydeptype nor deptype is set | pending | — |

