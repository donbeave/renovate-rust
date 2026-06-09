# `lib/workers/repository/updates/branch-name.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**27/27 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | falls back to sharedvariablename if no groupname | ported | [`crates/renovate-core/src/branch.rs:1956`](../../../../../../../crates/renovate-core/src/branch.rs#L1956) |
| 19 | ignores grouping of replacement update | ported | [`crates/renovate-core/src/branch.rs:1967`](../../../../../../../crates/renovate-core/src/branch.rs#L1967) |
| 36 | applies grouping for lockfile maintenance update | ported | [`crates/renovate-core/src/branch.rs:1977`](../../../../../../../crates/renovate-core/src/branch.rs#L1977) |
| 52 | uses default branch name for lockfile maintenance without groupname | ported | [`crates/renovate-core/src/branch.rs:1988`](../../../../../../../crates/renovate-core/src/branch.rs#L1988) |
| 63 | separates lockfilemaintenance from non-lockfilemaintenance with same groupname | ported | [`crates/renovate-core/src/branch.rs:1998`](../../../../../../../crates/renovate-core/src/branch.rs#L1998) |
| 89 | uses groupname if no slug defined, ignores sharedvariablename | ported | [`crates/renovate-core/src/branch.rs:2009`](../../../../../../../crates/renovate-core/src/branch.rs#L2009) |
| 102 | compile groupname before slugging | ported | [`crates/renovate-core/src/branch.rs:2020`](../../../../../../../crates/renovate-core/src/branch.rs#L2020) |
| 115 | uses groupslug if defined | ported | [`crates/renovate-core/src/branch.rs:2031`](../../../../../../../crates/renovate-core/src/branch.rs#L2031) |
| 129 | separates major with groups | ported | [`crates/renovate-core/src/branch.rs:1373`](../../../../../../../crates/renovate-core/src/branch.rs#L1373) |
| 146 | separates minor with groups | ported | [`crates/renovate-core/src/branch.rs:2042`](../../../../../../../crates/renovate-core/src/branch.rs#L2042) |
| 163 | separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2054`](../../../../../../../crates/renovate-core/src/branch.rs#L2054) |
| 183 | uses single major with groups | ported | [`crates/renovate-core/src/branch.rs:1384`](../../../../../../../crates/renovate-core/src/branch.rs#L1384) |
| 200 | separates patch groups and uses update topic | ported | [`crates/renovate-core/src/branch.rs:2065`](../../../../../../../crates/renovate-core/src/branch.rs#L2065) |
| 218 | compiles multiple times | ported | [`crates/renovate-core/src/branch.rs:2077`](../../../../../../../crates/renovate-core/src/branch.rs#L2077) |
| 229 | separates patches when separateminorpatch=true | ported | [`crates/renovate-core/src/branch.rs:1282`](../../../../../../../crates/renovate-core/src/branch.rs#L1282) |
| 249 | does not separate patches when separateminorpatch=false | ported | [`crates/renovate-core/src/branch.rs:1302`](../../../../../../../crates/renovate-core/src/branch.rs#L1302) |
| 269 | realistic defaults | ported | [`crates/renovate-core/src/branch.rs:1429`](../../../../../../../crates/renovate-core/src/branch.rs#L1429) |
| 284 | realistic defaults with strict branch name enabled | ported | [`crates/renovate-core/src/branch.rs:1436`](../../../../../../../crates/renovate-core/src/branch.rs#L1436) |
| 300 | removes slashes from the non-suffix part | ported | [`crates/renovate-core/src/branch.rs:1446`](../../../../../../../crates/renovate-core/src/branch.rs#L1446) |
| 316 | hashedbranchlength hashing | ported | [`crates/renovate-core/src/branch.rs:1883`](../../../../../../../crates/renovate-core/src/branch.rs#L1883) |
| 332 | hashedbranchlength hashing with group name | ported | [`crates/renovate-core/src/branch.rs:1892`](../../../../../../../crates/renovate-core/src/branch.rs#L1892) |
| 350 | hashedbranchlength too short | ported | [`crates/renovate-core/src/branch.rs:1901`](../../../../../../../crates/renovate-core/src/branch.rs#L1901) |
| 368 | hashedbranchlength no topic | ported | [`crates/renovate-core/src/branch.rs:1907`](../../../../../../../crates/renovate-core/src/branch.rs#L1907) |
| 386 | hashedbranchlength separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:1913`](../../../../../../../crates/renovate-core/src/branch.rs#L1913) |
| 405 | enforces valid git branch name | ported | [`crates/renovate-core/src/branch.rs:1456`](../../../../../../../crates/renovate-core/src/branch.rs#L1456) |
| 491 | strict branch name enabled group | ported | [`crates/renovate-core/src/branch.rs:1528`](../../../../../../../crates/renovate-core/src/branch.rs#L1528) |
| 506 | strict branch name disabled | ported | [`crates/renovate-core/src/branch.rs:1539`](../../../../../../../crates/renovate-core/src/branch.rs#L1539) |

