# `lib/workers/repository/updates/branch-name.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**27/27 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | falls back to sharedvariablename if no groupname | ported | [`crates/renovate-core/src/branch.rs:2064`](../../../../../../../crates/renovate-core/src/branch.rs#L2064) |
| 19 | ignores grouping of replacement update | ported | [`crates/renovate-core/src/branch.rs:2075`](../../../../../../../crates/renovate-core/src/branch.rs#L2075) |
| 36 | applies grouping for lockfile maintenance update | ported | [`crates/renovate-core/src/branch.rs:2085`](../../../../../../../crates/renovate-core/src/branch.rs#L2085) |
| 52 | uses default branch name for lockfile maintenance without groupname | ported | [`crates/renovate-core/src/branch.rs:2096`](../../../../../../../crates/renovate-core/src/branch.rs#L2096) |
| 63 | separates lockfilemaintenance from non-lockfilemaintenance with same groupname | ported | [`crates/renovate-core/src/branch.rs:2106`](../../../../../../../crates/renovate-core/src/branch.rs#L2106) |
| 89 | uses groupname if no slug defined, ignores sharedvariablename | ported | [`crates/renovate-core/src/branch.rs:2117`](../../../../../../../crates/renovate-core/src/branch.rs#L2117) |
| 102 | compile groupname before slugging | ported | [`crates/renovate-core/src/branch.rs:2128`](../../../../../../../crates/renovate-core/src/branch.rs#L2128) |
| 115 | uses groupslug if defined | ported | [`crates/renovate-core/src/branch.rs:2139`](../../../../../../../crates/renovate-core/src/branch.rs#L2139) |
| 129 | separates major with groups | ported | [`crates/renovate-core/src/branch.rs:1481`](../../../../../../../crates/renovate-core/src/branch.rs#L1481) |
| 146 | separates minor with groups | ported | [`crates/renovate-core/src/branch.rs:2150`](../../../../../../../crates/renovate-core/src/branch.rs#L2150) |
| 163 | separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2162`](../../../../../../../crates/renovate-core/src/branch.rs#L2162) |
| 183 | uses single major with groups | ported | [`crates/renovate-core/src/branch.rs:1492`](../../../../../../../crates/renovate-core/src/branch.rs#L1492) |
| 200 | separates patch groups and uses update topic | ported | [`crates/renovate-core/src/branch.rs:2173`](../../../../../../../crates/renovate-core/src/branch.rs#L2173) |
| 218 | compiles multiple times | ported | [`crates/renovate-core/src/branch.rs:2185`](../../../../../../../crates/renovate-core/src/branch.rs#L2185) |
| 229 | separates patches when separateminorpatch=true | ported | [`crates/renovate-core/src/branch.rs:1390`](../../../../../../../crates/renovate-core/src/branch.rs#L1390) |
| 249 | does not separate patches when separateminorpatch=false | ported | [`crates/renovate-core/src/branch.rs:1410`](../../../../../../../crates/renovate-core/src/branch.rs#L1410) |
| 269 | realistic defaults | ported | [`crates/renovate-core/src/branch.rs:1537`](../../../../../../../crates/renovate-core/src/branch.rs#L1537) |
| 284 | realistic defaults with strict branch name enabled | ported | [`crates/renovate-core/src/branch.rs:1544`](../../../../../../../crates/renovate-core/src/branch.rs#L1544) |
| 300 | removes slashes from the non-suffix part | ported | [`crates/renovate-core/src/branch.rs:1554`](../../../../../../../crates/renovate-core/src/branch.rs#L1554) |
| 316 | hashedbranchlength hashing | ported | [`crates/renovate-core/src/branch.rs:1991`](../../../../../../../crates/renovate-core/src/branch.rs#L1991) |
| 332 | hashedbranchlength hashing with group name | ported | [`crates/renovate-core/src/branch.rs:2000`](../../../../../../../crates/renovate-core/src/branch.rs#L2000) |
| 350 | hashedbranchlength too short | ported | [`crates/renovate-core/src/branch.rs:2009`](../../../../../../../crates/renovate-core/src/branch.rs#L2009) |
| 368 | hashedbranchlength no topic | ported | [`crates/renovate-core/src/branch.rs:2015`](../../../../../../../crates/renovate-core/src/branch.rs#L2015) |
| 386 | hashedbranchlength separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2021`](../../../../../../../crates/renovate-core/src/branch.rs#L2021) |
| 405 | enforces valid git branch name | ported | [`crates/renovate-core/src/branch.rs:1564`](../../../../../../../crates/renovate-core/src/branch.rs#L1564) |
| 491 | strict branch name enabled group | ported | [`crates/renovate-core/src/branch.rs:1636`](../../../../../../../crates/renovate-core/src/branch.rs#L1636) |
| 506 | strict branch name disabled | ported | [`crates/renovate-core/src/branch.rs:1647`](../../../../../../../crates/renovate-core/src/branch.rs#L1647) |

