# `lib/workers/repository/updates/branch-name.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**27/27 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | falls back to sharedvariablename if no groupname | ported | [`crates/renovate-core/src/branch.rs:2059`](../../../../../../../crates/renovate-core/src/branch.rs#L2059) |
| 19 | ignores grouping of replacement update | ported | [`crates/renovate-core/src/branch.rs:2070`](../../../../../../../crates/renovate-core/src/branch.rs#L2070) |
| 36 | applies grouping for lockfile maintenance update | ported | [`crates/renovate-core/src/branch.rs:2080`](../../../../../../../crates/renovate-core/src/branch.rs#L2080) |
| 52 | uses default branch name for lockfile maintenance without groupname | ported | [`crates/renovate-core/src/branch.rs:2091`](../../../../../../../crates/renovate-core/src/branch.rs#L2091) |
| 63 | separates lockfilemaintenance from non-lockfilemaintenance with same groupname | ported | [`crates/renovate-core/src/branch.rs:2101`](../../../../../../../crates/renovate-core/src/branch.rs#L2101) |
| 89 | uses groupname if no slug defined, ignores sharedvariablename | ported | [`crates/renovate-core/src/branch.rs:2112`](../../../../../../../crates/renovate-core/src/branch.rs#L2112) |
| 102 | compile groupname before slugging | ported | [`crates/renovate-core/src/branch.rs:2123`](../../../../../../../crates/renovate-core/src/branch.rs#L2123) |
| 115 | uses groupslug if defined | ported | [`crates/renovate-core/src/branch.rs:2134`](../../../../../../../crates/renovate-core/src/branch.rs#L2134) |
| 129 | separates major with groups | ported | [`crates/renovate-core/src/branch.rs:1476`](../../../../../../../crates/renovate-core/src/branch.rs#L1476) |
| 146 | separates minor with groups | ported | [`crates/renovate-core/src/branch.rs:2145`](../../../../../../../crates/renovate-core/src/branch.rs#L2145) |
| 163 | separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2157`](../../../../../../../crates/renovate-core/src/branch.rs#L2157) |
| 183 | uses single major with groups | ported | [`crates/renovate-core/src/branch.rs:1487`](../../../../../../../crates/renovate-core/src/branch.rs#L1487) |
| 200 | separates patch groups and uses update topic | ported | [`crates/renovate-core/src/branch.rs:2168`](../../../../../../../crates/renovate-core/src/branch.rs#L2168) |
| 218 | compiles multiple times | ported | [`crates/renovate-core/src/branch.rs:2180`](../../../../../../../crates/renovate-core/src/branch.rs#L2180) |
| 229 | separates patches when separateminorpatch=true | ported | [`crates/renovate-core/src/branch.rs:1385`](../../../../../../../crates/renovate-core/src/branch.rs#L1385) |
| 249 | does not separate patches when separateminorpatch=false | ported | [`crates/renovate-core/src/branch.rs:1405`](../../../../../../../crates/renovate-core/src/branch.rs#L1405) |
| 269 | realistic defaults | ported | [`crates/renovate-core/src/branch.rs:1532`](../../../../../../../crates/renovate-core/src/branch.rs#L1532) |
| 284 | realistic defaults with strict branch name enabled | ported | [`crates/renovate-core/src/branch.rs:1539`](../../../../../../../crates/renovate-core/src/branch.rs#L1539) |
| 300 | removes slashes from the non-suffix part | ported | [`crates/renovate-core/src/branch.rs:1549`](../../../../../../../crates/renovate-core/src/branch.rs#L1549) |
| 316 | hashedbranchlength hashing | ported | [`crates/renovate-core/src/branch.rs:1986`](../../../../../../../crates/renovate-core/src/branch.rs#L1986) |
| 332 | hashedbranchlength hashing with group name | ported | [`crates/renovate-core/src/branch.rs:1995`](../../../../../../../crates/renovate-core/src/branch.rs#L1995) |
| 350 | hashedbranchlength too short | ported | [`crates/renovate-core/src/branch.rs:2004`](../../../../../../../crates/renovate-core/src/branch.rs#L2004) |
| 368 | hashedbranchlength no topic | ported | [`crates/renovate-core/src/branch.rs:2010`](../../../../../../../crates/renovate-core/src/branch.rs#L2010) |
| 386 | hashedbranchlength separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2016`](../../../../../../../crates/renovate-core/src/branch.rs#L2016) |
| 405 | enforces valid git branch name | ported | [`crates/renovate-core/src/branch.rs:1559`](../../../../../../../crates/renovate-core/src/branch.rs#L1559) |
| 491 | strict branch name enabled group | ported | [`crates/renovate-core/src/branch.rs:1631`](../../../../../../../crates/renovate-core/src/branch.rs#L1631) |
| 506 | strict branch name disabled | ported | [`crates/renovate-core/src/branch.rs:1642`](../../../../../../../crates/renovate-core/src/branch.rs#L1642) |

