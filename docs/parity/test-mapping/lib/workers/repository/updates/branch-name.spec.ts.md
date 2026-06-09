# `lib/workers/repository/updates/branch-name.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**27/27 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | falls back to sharedvariablename if no groupname | ported | [`crates/renovate-core/src/branch.rs:2069`](../../../../../../../crates/renovate-core/src/branch.rs#L2069) |
| 19 | ignores grouping of replacement update | ported | [`crates/renovate-core/src/branch.rs:2080`](../../../../../../../crates/renovate-core/src/branch.rs#L2080) |
| 36 | applies grouping for lockfile maintenance update | ported | [`crates/renovate-core/src/branch.rs:2090`](../../../../../../../crates/renovate-core/src/branch.rs#L2090) |
| 52 | uses default branch name for lockfile maintenance without groupname | ported | [`crates/renovate-core/src/branch.rs:2101`](../../../../../../../crates/renovate-core/src/branch.rs#L2101) |
| 63 | separates lockfilemaintenance from non-lockfilemaintenance with same groupname | ported | [`crates/renovate-core/src/branch.rs:2111`](../../../../../../../crates/renovate-core/src/branch.rs#L2111) |
| 89 | uses groupname if no slug defined, ignores sharedvariablename | ported | [`crates/renovate-core/src/branch.rs:2122`](../../../../../../../crates/renovate-core/src/branch.rs#L2122) |
| 102 | compile groupname before slugging | ported | [`crates/renovate-core/src/branch.rs:2133`](../../../../../../../crates/renovate-core/src/branch.rs#L2133) |
| 115 | uses groupslug if defined | ported | [`crates/renovate-core/src/branch.rs:2144`](../../../../../../../crates/renovate-core/src/branch.rs#L2144) |
| 129 | separates major with groups | ported | [`crates/renovate-core/src/branch.rs:1486`](../../../../../../../crates/renovate-core/src/branch.rs#L1486) |
| 146 | separates minor with groups | ported | [`crates/renovate-core/src/branch.rs:2155`](../../../../../../../crates/renovate-core/src/branch.rs#L2155) |
| 163 | separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2167`](../../../../../../../crates/renovate-core/src/branch.rs#L2167) |
| 183 | uses single major with groups | ported | [`crates/renovate-core/src/branch.rs:1497`](../../../../../../../crates/renovate-core/src/branch.rs#L1497) |
| 200 | separates patch groups and uses update topic | ported | [`crates/renovate-core/src/branch.rs:2178`](../../../../../../../crates/renovate-core/src/branch.rs#L2178) |
| 218 | compiles multiple times | ported | [`crates/renovate-core/src/branch.rs:2190`](../../../../../../../crates/renovate-core/src/branch.rs#L2190) |
| 229 | separates patches when separateminorpatch=true | ported | [`crates/renovate-core/src/branch.rs:1395`](../../../../../../../crates/renovate-core/src/branch.rs#L1395) |
| 249 | does not separate patches when separateminorpatch=false | ported | [`crates/renovate-core/src/branch.rs:1415`](../../../../../../../crates/renovate-core/src/branch.rs#L1415) |
| 269 | realistic defaults | ported | [`crates/renovate-core/src/branch.rs:1542`](../../../../../../../crates/renovate-core/src/branch.rs#L1542) |
| 284 | realistic defaults with strict branch name enabled | ported | [`crates/renovate-core/src/branch.rs:1549`](../../../../../../../crates/renovate-core/src/branch.rs#L1549) |
| 300 | removes slashes from the non-suffix part | ported | [`crates/renovate-core/src/branch.rs:1559`](../../../../../../../crates/renovate-core/src/branch.rs#L1559) |
| 316 | hashedbranchlength hashing | ported | [`crates/renovate-core/src/branch.rs:1996`](../../../../../../../crates/renovate-core/src/branch.rs#L1996) |
| 332 | hashedbranchlength hashing with group name | ported | [`crates/renovate-core/src/branch.rs:2005`](../../../../../../../crates/renovate-core/src/branch.rs#L2005) |
| 350 | hashedbranchlength too short | ported | [`crates/renovate-core/src/branch.rs:2014`](../../../../../../../crates/renovate-core/src/branch.rs#L2014) |
| 368 | hashedbranchlength no topic | ported | [`crates/renovate-core/src/branch.rs:2020`](../../../../../../../crates/renovate-core/src/branch.rs#L2020) |
| 386 | hashedbranchlength separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2026`](../../../../../../../crates/renovate-core/src/branch.rs#L2026) |
| 405 | enforces valid git branch name | ported | [`crates/renovate-core/src/branch.rs:1569`](../../../../../../../crates/renovate-core/src/branch.rs#L1569) |
| 491 | strict branch name enabled group | ported | [`crates/renovate-core/src/branch.rs:1641`](../../../../../../../crates/renovate-core/src/branch.rs#L1641) |
| 506 | strict branch name disabled | ported | [`crates/renovate-core/src/branch.rs:1652`](../../../../../../../crates/renovate-core/src/branch.rs#L1652) |

