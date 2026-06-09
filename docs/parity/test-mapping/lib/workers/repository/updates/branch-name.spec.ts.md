# `lib/workers/repository/updates/branch-name.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**27/27 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | falls back to sharedvariablename if no groupname | ported | [`crates/renovate-core/src/branch.rs:2060`](../../../../../../../crates/renovate-core/src/branch.rs#L2060) |
| 19 | ignores grouping of replacement update | ported | [`crates/renovate-core/src/branch.rs:2071`](../../../../../../../crates/renovate-core/src/branch.rs#L2071) |
| 36 | applies grouping for lockfile maintenance update | ported | [`crates/renovate-core/src/branch.rs:2081`](../../../../../../../crates/renovate-core/src/branch.rs#L2081) |
| 52 | uses default branch name for lockfile maintenance without groupname | ported | [`crates/renovate-core/src/branch.rs:2092`](../../../../../../../crates/renovate-core/src/branch.rs#L2092) |
| 63 | separates lockfilemaintenance from non-lockfilemaintenance with same groupname | ported | [`crates/renovate-core/src/branch.rs:2102`](../../../../../../../crates/renovate-core/src/branch.rs#L2102) |
| 89 | uses groupname if no slug defined, ignores sharedvariablename | ported | [`crates/renovate-core/src/branch.rs:2113`](../../../../../../../crates/renovate-core/src/branch.rs#L2113) |
| 102 | compile groupname before slugging | ported | [`crates/renovate-core/src/branch.rs:2124`](../../../../../../../crates/renovate-core/src/branch.rs#L2124) |
| 115 | uses groupslug if defined | ported | [`crates/renovate-core/src/branch.rs:2135`](../../../../../../../crates/renovate-core/src/branch.rs#L2135) |
| 129 | separates major with groups | ported | [`crates/renovate-core/src/branch.rs:1477`](../../../../../../../crates/renovate-core/src/branch.rs#L1477) |
| 146 | separates minor with groups | ported | [`crates/renovate-core/src/branch.rs:2146`](../../../../../../../crates/renovate-core/src/branch.rs#L2146) |
| 163 | separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2158`](../../../../../../../crates/renovate-core/src/branch.rs#L2158) |
| 183 | uses single major with groups | ported | [`crates/renovate-core/src/branch.rs:1488`](../../../../../../../crates/renovate-core/src/branch.rs#L1488) |
| 200 | separates patch groups and uses update topic | ported | [`crates/renovate-core/src/branch.rs:2169`](../../../../../../../crates/renovate-core/src/branch.rs#L2169) |
| 218 | compiles multiple times | ported | [`crates/renovate-core/src/branch.rs:2181`](../../../../../../../crates/renovate-core/src/branch.rs#L2181) |
| 229 | separates patches when separateminorpatch=true | ported | [`crates/renovate-core/src/branch.rs:1386`](../../../../../../../crates/renovate-core/src/branch.rs#L1386) |
| 249 | does not separate patches when separateminorpatch=false | ported | [`crates/renovate-core/src/branch.rs:1406`](../../../../../../../crates/renovate-core/src/branch.rs#L1406) |
| 269 | realistic defaults | ported | [`crates/renovate-core/src/branch.rs:1533`](../../../../../../../crates/renovate-core/src/branch.rs#L1533) |
| 284 | realistic defaults with strict branch name enabled | ported | [`crates/renovate-core/src/branch.rs:1540`](../../../../../../../crates/renovate-core/src/branch.rs#L1540) |
| 300 | removes slashes from the non-suffix part | ported | [`crates/renovate-core/src/branch.rs:1550`](../../../../../../../crates/renovate-core/src/branch.rs#L1550) |
| 316 | hashedbranchlength hashing | ported | [`crates/renovate-core/src/branch.rs:1987`](../../../../../../../crates/renovate-core/src/branch.rs#L1987) |
| 332 | hashedbranchlength hashing with group name | ported | [`crates/renovate-core/src/branch.rs:1996`](../../../../../../../crates/renovate-core/src/branch.rs#L1996) |
| 350 | hashedbranchlength too short | ported | [`crates/renovate-core/src/branch.rs:2005`](../../../../../../../crates/renovate-core/src/branch.rs#L2005) |
| 368 | hashedbranchlength no topic | ported | [`crates/renovate-core/src/branch.rs:2011`](../../../../../../../crates/renovate-core/src/branch.rs#L2011) |
| 386 | hashedbranchlength separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2017`](../../../../../../../crates/renovate-core/src/branch.rs#L2017) |
| 405 | enforces valid git branch name | ported | [`crates/renovate-core/src/branch.rs:1560`](../../../../../../../crates/renovate-core/src/branch.rs#L1560) |
| 491 | strict branch name enabled group | ported | [`crates/renovate-core/src/branch.rs:1632`](../../../../../../../crates/renovate-core/src/branch.rs#L1632) |
| 506 | strict branch name disabled | ported | [`crates/renovate-core/src/branch.rs:1643`](../../../../../../../crates/renovate-core/src/branch.rs#L1643) |

