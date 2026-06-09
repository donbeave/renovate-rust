# `lib/workers/repository/updates/branch-name.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**27/27 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | falls back to sharedvariablename if no groupname | ported | [`crates/renovate-core/src/branch.rs:1958`](../../../../../../../crates/renovate-core/src/branch.rs#L1958) |
| 19 | ignores grouping of replacement update | ported | [`crates/renovate-core/src/branch.rs:1969`](../../../../../../../crates/renovate-core/src/branch.rs#L1969) |
| 36 | applies grouping for lockfile maintenance update | ported | [`crates/renovate-core/src/branch.rs:1979`](../../../../../../../crates/renovate-core/src/branch.rs#L1979) |
| 52 | uses default branch name for lockfile maintenance without groupname | ported | [`crates/renovate-core/src/branch.rs:1990`](../../../../../../../crates/renovate-core/src/branch.rs#L1990) |
| 63 | separates lockfilemaintenance from non-lockfilemaintenance with same groupname | ported | [`crates/renovate-core/src/branch.rs:2000`](../../../../../../../crates/renovate-core/src/branch.rs#L2000) |
| 89 | uses groupname if no slug defined, ignores sharedvariablename | ported | [`crates/renovate-core/src/branch.rs:2011`](../../../../../../../crates/renovate-core/src/branch.rs#L2011) |
| 102 | compile groupname before slugging | ported | [`crates/renovate-core/src/branch.rs:2022`](../../../../../../../crates/renovate-core/src/branch.rs#L2022) |
| 115 | uses groupslug if defined | ported | [`crates/renovate-core/src/branch.rs:2033`](../../../../../../../crates/renovate-core/src/branch.rs#L2033) |
| 129 | separates major with groups | ported | [`crates/renovate-core/src/branch.rs:1375`](../../../../../../../crates/renovate-core/src/branch.rs#L1375) |
| 146 | separates minor with groups | ported | [`crates/renovate-core/src/branch.rs:2044`](../../../../../../../crates/renovate-core/src/branch.rs#L2044) |
| 163 | separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2056`](../../../../../../../crates/renovate-core/src/branch.rs#L2056) |
| 183 | uses single major with groups | ported | [`crates/renovate-core/src/branch.rs:1386`](../../../../../../../crates/renovate-core/src/branch.rs#L1386) |
| 200 | separates patch groups and uses update topic | ported | [`crates/renovate-core/src/branch.rs:2067`](../../../../../../../crates/renovate-core/src/branch.rs#L2067) |
| 218 | compiles multiple times | ported | [`crates/renovate-core/src/branch.rs:2079`](../../../../../../../crates/renovate-core/src/branch.rs#L2079) |
| 229 | separates patches when separateminorpatch=true | ported | [`crates/renovate-core/src/branch.rs:1284`](../../../../../../../crates/renovate-core/src/branch.rs#L1284) |
| 249 | does not separate patches when separateminorpatch=false | ported | [`crates/renovate-core/src/branch.rs:1304`](../../../../../../../crates/renovate-core/src/branch.rs#L1304) |
| 269 | realistic defaults | ported | [`crates/renovate-core/src/branch.rs:1431`](../../../../../../../crates/renovate-core/src/branch.rs#L1431) |
| 284 | realistic defaults with strict branch name enabled | ported | [`crates/renovate-core/src/branch.rs:1438`](../../../../../../../crates/renovate-core/src/branch.rs#L1438) |
| 300 | removes slashes from the non-suffix part | ported | [`crates/renovate-core/src/branch.rs:1448`](../../../../../../../crates/renovate-core/src/branch.rs#L1448) |
| 316 | hashedbranchlength hashing | ported | [`crates/renovate-core/src/branch.rs:1885`](../../../../../../../crates/renovate-core/src/branch.rs#L1885) |
| 332 | hashedbranchlength hashing with group name | ported | [`crates/renovate-core/src/branch.rs:1894`](../../../../../../../crates/renovate-core/src/branch.rs#L1894) |
| 350 | hashedbranchlength too short | ported | [`crates/renovate-core/src/branch.rs:1903`](../../../../../../../crates/renovate-core/src/branch.rs#L1903) |
| 368 | hashedbranchlength no topic | ported | [`crates/renovate-core/src/branch.rs:1909`](../../../../../../../crates/renovate-core/src/branch.rs#L1909) |
| 386 | hashedbranchlength separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:1915`](../../../../../../../crates/renovate-core/src/branch.rs#L1915) |
| 405 | enforces valid git branch name | ported | [`crates/renovate-core/src/branch.rs:1458`](../../../../../../../crates/renovate-core/src/branch.rs#L1458) |
| 491 | strict branch name enabled group | ported | [`crates/renovate-core/src/branch.rs:1530`](../../../../../../../crates/renovate-core/src/branch.rs#L1530) |
| 506 | strict branch name disabled | ported | [`crates/renovate-core/src/branch.rs:1541`](../../../../../../../crates/renovate-core/src/branch.rs#L1541) |

