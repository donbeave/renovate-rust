# `lib/workers/repository/updates/branch-name.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**27/27 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | falls back to sharedvariablename if no groupname | ported | [`crates/renovate-core/src/branch.rs:2035`](../../../../../../../crates/renovate-core/src/branch.rs#L2035) |
| 19 | ignores grouping of replacement update | ported | [`crates/renovate-core/src/branch.rs:2046`](../../../../../../../crates/renovate-core/src/branch.rs#L2046) |
| 36 | applies grouping for lockfile maintenance update | ported | [`crates/renovate-core/src/branch.rs:2056`](../../../../../../../crates/renovate-core/src/branch.rs#L2056) |
| 52 | uses default branch name for lockfile maintenance without groupname | ported | [`crates/renovate-core/src/branch.rs:2067`](../../../../../../../crates/renovate-core/src/branch.rs#L2067) |
| 63 | separates lockfilemaintenance from non-lockfilemaintenance with same groupname | ported | [`crates/renovate-core/src/branch.rs:2077`](../../../../../../../crates/renovate-core/src/branch.rs#L2077) |
| 89 | uses groupname if no slug defined, ignores sharedvariablename | ported | [`crates/renovate-core/src/branch.rs:2088`](../../../../../../../crates/renovate-core/src/branch.rs#L2088) |
| 102 | compile groupname before slugging | ported | [`crates/renovate-core/src/branch.rs:2099`](../../../../../../../crates/renovate-core/src/branch.rs#L2099) |
| 115 | uses groupslug if defined | ported | [`crates/renovate-core/src/branch.rs:2110`](../../../../../../../crates/renovate-core/src/branch.rs#L2110) |
| 129 | separates major with groups | ported | [`crates/renovate-core/src/branch.rs:1452`](../../../../../../../crates/renovate-core/src/branch.rs#L1452) |
| 146 | separates minor with groups | ported | [`crates/renovate-core/src/branch.rs:2121`](../../../../../../../crates/renovate-core/src/branch.rs#L2121) |
| 163 | separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2133`](../../../../../../../crates/renovate-core/src/branch.rs#L2133) |
| 183 | uses single major with groups | ported | [`crates/renovate-core/src/branch.rs:1463`](../../../../../../../crates/renovate-core/src/branch.rs#L1463) |
| 200 | separates patch groups and uses update topic | ported | [`crates/renovate-core/src/branch.rs:2144`](../../../../../../../crates/renovate-core/src/branch.rs#L2144) |
| 218 | compiles multiple times | ported | [`crates/renovate-core/src/branch.rs:2156`](../../../../../../../crates/renovate-core/src/branch.rs#L2156) |
| 229 | separates patches when separateminorpatch=true | ported | [`crates/renovate-core/src/branch.rs:1361`](../../../../../../../crates/renovate-core/src/branch.rs#L1361) |
| 249 | does not separate patches when separateminorpatch=false | ported | [`crates/renovate-core/src/branch.rs:1381`](../../../../../../../crates/renovate-core/src/branch.rs#L1381) |
| 269 | realistic defaults | ported | [`crates/renovate-core/src/branch.rs:1508`](../../../../../../../crates/renovate-core/src/branch.rs#L1508) |
| 284 | realistic defaults with strict branch name enabled | ported | [`crates/renovate-core/src/branch.rs:1515`](../../../../../../../crates/renovate-core/src/branch.rs#L1515) |
| 300 | removes slashes from the non-suffix part | ported | [`crates/renovate-core/src/branch.rs:1525`](../../../../../../../crates/renovate-core/src/branch.rs#L1525) |
| 316 | hashedbranchlength hashing | ported | [`crates/renovate-core/src/branch.rs:1962`](../../../../../../../crates/renovate-core/src/branch.rs#L1962) |
| 332 | hashedbranchlength hashing with group name | ported | [`crates/renovate-core/src/branch.rs:1971`](../../../../../../../crates/renovate-core/src/branch.rs#L1971) |
| 350 | hashedbranchlength too short | ported | [`crates/renovate-core/src/branch.rs:1980`](../../../../../../../crates/renovate-core/src/branch.rs#L1980) |
| 368 | hashedbranchlength no topic | ported | [`crates/renovate-core/src/branch.rs:1986`](../../../../../../../crates/renovate-core/src/branch.rs#L1986) |
| 386 | hashedbranchlength separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:1992`](../../../../../../../crates/renovate-core/src/branch.rs#L1992) |
| 405 | enforces valid git branch name | ported | [`crates/renovate-core/src/branch.rs:1535`](../../../../../../../crates/renovate-core/src/branch.rs#L1535) |
| 491 | strict branch name enabled group | ported | [`crates/renovate-core/src/branch.rs:1607`](../../../../../../../crates/renovate-core/src/branch.rs#L1607) |
| 506 | strict branch name disabled | ported | [`crates/renovate-core/src/branch.rs:1618`](../../../../../../../crates/renovate-core/src/branch.rs#L1618) |

