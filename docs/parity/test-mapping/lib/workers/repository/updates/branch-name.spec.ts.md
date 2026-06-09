# `lib/workers/repository/updates/branch-name.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**27/27 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | falls back to sharedvariablename if no groupname | ported | [`crates/renovate-core/src/branch.rs:2056`](../../../../../../../crates/renovate-core/src/branch.rs#L2056) |
| 19 | ignores grouping of replacement update | ported | [`crates/renovate-core/src/branch.rs:2067`](../../../../../../../crates/renovate-core/src/branch.rs#L2067) |
| 36 | applies grouping for lockfile maintenance update | ported | [`crates/renovate-core/src/branch.rs:2077`](../../../../../../../crates/renovate-core/src/branch.rs#L2077) |
| 52 | uses default branch name for lockfile maintenance without groupname | ported | [`crates/renovate-core/src/branch.rs:2088`](../../../../../../../crates/renovate-core/src/branch.rs#L2088) |
| 63 | separates lockfilemaintenance from non-lockfilemaintenance with same groupname | ported | [`crates/renovate-core/src/branch.rs:2098`](../../../../../../../crates/renovate-core/src/branch.rs#L2098) |
| 89 | uses groupname if no slug defined, ignores sharedvariablename | ported | [`crates/renovate-core/src/branch.rs:2109`](../../../../../../../crates/renovate-core/src/branch.rs#L2109) |
| 102 | compile groupname before slugging | ported | [`crates/renovate-core/src/branch.rs:2120`](../../../../../../../crates/renovate-core/src/branch.rs#L2120) |
| 115 | uses groupslug if defined | ported | [`crates/renovate-core/src/branch.rs:2131`](../../../../../../../crates/renovate-core/src/branch.rs#L2131) |
| 129 | separates major with groups | ported | [`crates/renovate-core/src/branch.rs:1473`](../../../../../../../crates/renovate-core/src/branch.rs#L1473) |
| 146 | separates minor with groups | ported | [`crates/renovate-core/src/branch.rs:2142`](../../../../../../../crates/renovate-core/src/branch.rs#L2142) |
| 163 | separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2154`](../../../../../../../crates/renovate-core/src/branch.rs#L2154) |
| 183 | uses single major with groups | ported | [`crates/renovate-core/src/branch.rs:1484`](../../../../../../../crates/renovate-core/src/branch.rs#L1484) |
| 200 | separates patch groups and uses update topic | ported | [`crates/renovate-core/src/branch.rs:2165`](../../../../../../../crates/renovate-core/src/branch.rs#L2165) |
| 218 | compiles multiple times | ported | [`crates/renovate-core/src/branch.rs:2177`](../../../../../../../crates/renovate-core/src/branch.rs#L2177) |
| 229 | separates patches when separateminorpatch=true | ported | [`crates/renovate-core/src/branch.rs:1382`](../../../../../../../crates/renovate-core/src/branch.rs#L1382) |
| 249 | does not separate patches when separateminorpatch=false | ported | [`crates/renovate-core/src/branch.rs:1402`](../../../../../../../crates/renovate-core/src/branch.rs#L1402) |
| 269 | realistic defaults | ported | [`crates/renovate-core/src/branch.rs:1529`](../../../../../../../crates/renovate-core/src/branch.rs#L1529) |
| 284 | realistic defaults with strict branch name enabled | ported | [`crates/renovate-core/src/branch.rs:1536`](../../../../../../../crates/renovate-core/src/branch.rs#L1536) |
| 300 | removes slashes from the non-suffix part | ported | [`crates/renovate-core/src/branch.rs:1546`](../../../../../../../crates/renovate-core/src/branch.rs#L1546) |
| 316 | hashedbranchlength hashing | ported | [`crates/renovate-core/src/branch.rs:1983`](../../../../../../../crates/renovate-core/src/branch.rs#L1983) |
| 332 | hashedbranchlength hashing with group name | ported | [`crates/renovate-core/src/branch.rs:1992`](../../../../../../../crates/renovate-core/src/branch.rs#L1992) |
| 350 | hashedbranchlength too short | ported | [`crates/renovate-core/src/branch.rs:2001`](../../../../../../../crates/renovate-core/src/branch.rs#L2001) |
| 368 | hashedbranchlength no topic | ported | [`crates/renovate-core/src/branch.rs:2007`](../../../../../../../crates/renovate-core/src/branch.rs#L2007) |
| 386 | hashedbranchlength separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2013`](../../../../../../../crates/renovate-core/src/branch.rs#L2013) |
| 405 | enforces valid git branch name | ported | [`crates/renovate-core/src/branch.rs:1556`](../../../../../../../crates/renovate-core/src/branch.rs#L1556) |
| 491 | strict branch name enabled group | ported | [`crates/renovate-core/src/branch.rs:1628`](../../../../../../../crates/renovate-core/src/branch.rs#L1628) |
| 506 | strict branch name disabled | ported | [`crates/renovate-core/src/branch.rs:1639`](../../../../../../../crates/renovate-core/src/branch.rs#L1639) |

