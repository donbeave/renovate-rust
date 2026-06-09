# `lib/workers/repository/updates/branch-name.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**27/27 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | falls back to sharedvariablename if no groupname | ported | [`crates/renovate-core/src/branch.rs:1942`](../../../../../../../crates/renovate-core/src/branch.rs#L1942) |
| 19 | ignores grouping of replacement update | ported | [`crates/renovate-core/src/branch.rs:1953`](../../../../../../../crates/renovate-core/src/branch.rs#L1953) |
| 36 | applies grouping for lockfile maintenance update | ported | [`crates/renovate-core/src/branch.rs:1963`](../../../../../../../crates/renovate-core/src/branch.rs#L1963) |
| 52 | uses default branch name for lockfile maintenance without groupname | ported | [`crates/renovate-core/src/branch.rs:1974`](../../../../../../../crates/renovate-core/src/branch.rs#L1974) |
| 63 | separates lockfilemaintenance from non-lockfilemaintenance with same groupname | ported | [`crates/renovate-core/src/branch.rs:1984`](../../../../../../../crates/renovate-core/src/branch.rs#L1984) |
| 89 | uses groupname if no slug defined, ignores sharedvariablename | ported | [`crates/renovate-core/src/branch.rs:1995`](../../../../../../../crates/renovate-core/src/branch.rs#L1995) |
| 102 | compile groupname before slugging | ported | [`crates/renovate-core/src/branch.rs:2006`](../../../../../../../crates/renovate-core/src/branch.rs#L2006) |
| 115 | uses groupslug if defined | ported | [`crates/renovate-core/src/branch.rs:2017`](../../../../../../../crates/renovate-core/src/branch.rs#L2017) |
| 129 | separates major with groups | ported | [`crates/renovate-core/src/branch.rs:1359`](../../../../../../../crates/renovate-core/src/branch.rs#L1359) |
| 146 | separates minor with groups | ported | [`crates/renovate-core/src/branch.rs:2028`](../../../../../../../crates/renovate-core/src/branch.rs#L2028) |
| 163 | separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2040`](../../../../../../../crates/renovate-core/src/branch.rs#L2040) |
| 183 | uses single major with groups | ported | [`crates/renovate-core/src/branch.rs:1370`](../../../../../../../crates/renovate-core/src/branch.rs#L1370) |
| 200 | separates patch groups and uses update topic | ported | [`crates/renovate-core/src/branch.rs:2051`](../../../../../../../crates/renovate-core/src/branch.rs#L2051) |
| 218 | compiles multiple times | ported | [`crates/renovate-core/src/branch.rs:2063`](../../../../../../../crates/renovate-core/src/branch.rs#L2063) |
| 229 | separates patches when separateminorpatch=true | ported | [`crates/renovate-core/src/branch.rs:1268`](../../../../../../../crates/renovate-core/src/branch.rs#L1268) |
| 249 | does not separate patches when separateminorpatch=false | ported | [`crates/renovate-core/src/branch.rs:1288`](../../../../../../../crates/renovate-core/src/branch.rs#L1288) |
| 269 | realistic defaults | ported | [`crates/renovate-core/src/branch.rs:1415`](../../../../../../../crates/renovate-core/src/branch.rs#L1415) |
| 284 | realistic defaults with strict branch name enabled | ported | [`crates/renovate-core/src/branch.rs:1422`](../../../../../../../crates/renovate-core/src/branch.rs#L1422) |
| 300 | removes slashes from the non-suffix part | ported | [`crates/renovate-core/src/branch.rs:1432`](../../../../../../../crates/renovate-core/src/branch.rs#L1432) |
| 316 | hashedbranchlength hashing | ported | [`crates/renovate-core/src/branch.rs:1869`](../../../../../../../crates/renovate-core/src/branch.rs#L1869) |
| 332 | hashedbranchlength hashing with group name | ported | [`crates/renovate-core/src/branch.rs:1878`](../../../../../../../crates/renovate-core/src/branch.rs#L1878) |
| 350 | hashedbranchlength too short | ported | [`crates/renovate-core/src/branch.rs:1887`](../../../../../../../crates/renovate-core/src/branch.rs#L1887) |
| 368 | hashedbranchlength no topic | ported | [`crates/renovate-core/src/branch.rs:1893`](../../../../../../../crates/renovate-core/src/branch.rs#L1893) |
| 386 | hashedbranchlength separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:1899`](../../../../../../../crates/renovate-core/src/branch.rs#L1899) |
| 405 | enforces valid git branch name | ported | [`crates/renovate-core/src/branch.rs:1442`](../../../../../../../crates/renovate-core/src/branch.rs#L1442) |
| 491 | strict branch name enabled group | ported | [`crates/renovate-core/src/branch.rs:1514`](../../../../../../../crates/renovate-core/src/branch.rs#L1514) |
| 506 | strict branch name disabled | ported | [`crates/renovate-core/src/branch.rs:1525`](../../../../../../../crates/renovate-core/src/branch.rs#L1525) |

