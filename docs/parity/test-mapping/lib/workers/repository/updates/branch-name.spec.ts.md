# `lib/workers/repository/updates/branch-name.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**27/27 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | falls back to sharedvariablename if no groupname | ported | [`crates/renovate-core/src/branch.rs:2036`](../../../../../../../crates/renovate-core/src/branch.rs#L2036) |
| 19 | ignores grouping of replacement update | ported | [`crates/renovate-core/src/branch.rs:2047`](../../../../../../../crates/renovate-core/src/branch.rs#L2047) |
| 36 | applies grouping for lockfile maintenance update | ported | [`crates/renovate-core/src/branch.rs:2057`](../../../../../../../crates/renovate-core/src/branch.rs#L2057) |
| 52 | uses default branch name for lockfile maintenance without groupname | ported | [`crates/renovate-core/src/branch.rs:2068`](../../../../../../../crates/renovate-core/src/branch.rs#L2068) |
| 63 | separates lockfilemaintenance from non-lockfilemaintenance with same groupname | ported | [`crates/renovate-core/src/branch.rs:2078`](../../../../../../../crates/renovate-core/src/branch.rs#L2078) |
| 89 | uses groupname if no slug defined, ignores sharedvariablename | ported | [`crates/renovate-core/src/branch.rs:2089`](../../../../../../../crates/renovate-core/src/branch.rs#L2089) |
| 102 | compile groupname before slugging | ported | [`crates/renovate-core/src/branch.rs:2100`](../../../../../../../crates/renovate-core/src/branch.rs#L2100) |
| 115 | uses groupslug if defined | ported | [`crates/renovate-core/src/branch.rs:2111`](../../../../../../../crates/renovate-core/src/branch.rs#L2111) |
| 129 | separates major with groups | ported | [`crates/renovate-core/src/branch.rs:1453`](../../../../../../../crates/renovate-core/src/branch.rs#L1453) |
| 146 | separates minor with groups | ported | [`crates/renovate-core/src/branch.rs:2122`](../../../../../../../crates/renovate-core/src/branch.rs#L2122) |
| 163 | separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2134`](../../../../../../../crates/renovate-core/src/branch.rs#L2134) |
| 183 | uses single major with groups | ported | [`crates/renovate-core/src/branch.rs:1464`](../../../../../../../crates/renovate-core/src/branch.rs#L1464) |
| 200 | separates patch groups and uses update topic | ported | [`crates/renovate-core/src/branch.rs:2145`](../../../../../../../crates/renovate-core/src/branch.rs#L2145) |
| 218 | compiles multiple times | ported | [`crates/renovate-core/src/branch.rs:2157`](../../../../../../../crates/renovate-core/src/branch.rs#L2157) |
| 229 | separates patches when separateminorpatch=true | ported | [`crates/renovate-core/src/branch.rs:1362`](../../../../../../../crates/renovate-core/src/branch.rs#L1362) |
| 249 | does not separate patches when separateminorpatch=false | ported | [`crates/renovate-core/src/branch.rs:1382`](../../../../../../../crates/renovate-core/src/branch.rs#L1382) |
| 269 | realistic defaults | ported | [`crates/renovate-core/src/branch.rs:1509`](../../../../../../../crates/renovate-core/src/branch.rs#L1509) |
| 284 | realistic defaults with strict branch name enabled | ported | [`crates/renovate-core/src/branch.rs:1516`](../../../../../../../crates/renovate-core/src/branch.rs#L1516) |
| 300 | removes slashes from the non-suffix part | ported | [`crates/renovate-core/src/branch.rs:1526`](../../../../../../../crates/renovate-core/src/branch.rs#L1526) |
| 316 | hashedbranchlength hashing | ported | [`crates/renovate-core/src/branch.rs:1963`](../../../../../../../crates/renovate-core/src/branch.rs#L1963) |
| 332 | hashedbranchlength hashing with group name | ported | [`crates/renovate-core/src/branch.rs:1972`](../../../../../../../crates/renovate-core/src/branch.rs#L1972) |
| 350 | hashedbranchlength too short | ported | [`crates/renovate-core/src/branch.rs:1981`](../../../../../../../crates/renovate-core/src/branch.rs#L1981) |
| 368 | hashedbranchlength no topic | ported | [`crates/renovate-core/src/branch.rs:1987`](../../../../../../../crates/renovate-core/src/branch.rs#L1987) |
| 386 | hashedbranchlength separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:1993`](../../../../../../../crates/renovate-core/src/branch.rs#L1993) |
| 405 | enforces valid git branch name | ported | [`crates/renovate-core/src/branch.rs:1536`](../../../../../../../crates/renovate-core/src/branch.rs#L1536) |
| 491 | strict branch name enabled group | ported | [`crates/renovate-core/src/branch.rs:1608`](../../../../../../../crates/renovate-core/src/branch.rs#L1608) |
| 506 | strict branch name disabled | ported | [`crates/renovate-core/src/branch.rs:1619`](../../../../../../../crates/renovate-core/src/branch.rs#L1619) |

