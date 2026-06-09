# `lib/workers/repository/updates/branch-name.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**27/27 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | falls back to sharedvariablename if no groupname | ported | [`crates/renovate-core/src/branch.rs:2055`](../../../../../../../crates/renovate-core/src/branch.rs#L2055) |
| 19 | ignores grouping of replacement update | ported | [`crates/renovate-core/src/branch.rs:2066`](../../../../../../../crates/renovate-core/src/branch.rs#L2066) |
| 36 | applies grouping for lockfile maintenance update | ported | [`crates/renovate-core/src/branch.rs:2076`](../../../../../../../crates/renovate-core/src/branch.rs#L2076) |
| 52 | uses default branch name for lockfile maintenance without groupname | ported | [`crates/renovate-core/src/branch.rs:2087`](../../../../../../../crates/renovate-core/src/branch.rs#L2087) |
| 63 | separates lockfilemaintenance from non-lockfilemaintenance with same groupname | ported | [`crates/renovate-core/src/branch.rs:2097`](../../../../../../../crates/renovate-core/src/branch.rs#L2097) |
| 89 | uses groupname if no slug defined, ignores sharedvariablename | ported | [`crates/renovate-core/src/branch.rs:2108`](../../../../../../../crates/renovate-core/src/branch.rs#L2108) |
| 102 | compile groupname before slugging | ported | [`crates/renovate-core/src/branch.rs:2119`](../../../../../../../crates/renovate-core/src/branch.rs#L2119) |
| 115 | uses groupslug if defined | ported | [`crates/renovate-core/src/branch.rs:2130`](../../../../../../../crates/renovate-core/src/branch.rs#L2130) |
| 129 | separates major with groups | ported | [`crates/renovate-core/src/branch.rs:1472`](../../../../../../../crates/renovate-core/src/branch.rs#L1472) |
| 146 | separates minor with groups | ported | [`crates/renovate-core/src/branch.rs:2141`](../../../../../../../crates/renovate-core/src/branch.rs#L2141) |
| 163 | separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2153`](../../../../../../../crates/renovate-core/src/branch.rs#L2153) |
| 183 | uses single major with groups | ported | [`crates/renovate-core/src/branch.rs:1483`](../../../../../../../crates/renovate-core/src/branch.rs#L1483) |
| 200 | separates patch groups and uses update topic | ported | [`crates/renovate-core/src/branch.rs:2164`](../../../../../../../crates/renovate-core/src/branch.rs#L2164) |
| 218 | compiles multiple times | ported | [`crates/renovate-core/src/branch.rs:2176`](../../../../../../../crates/renovate-core/src/branch.rs#L2176) |
| 229 | separates patches when separateminorpatch=true | ported | [`crates/renovate-core/src/branch.rs:1381`](../../../../../../../crates/renovate-core/src/branch.rs#L1381) |
| 249 | does not separate patches when separateminorpatch=false | ported | [`crates/renovate-core/src/branch.rs:1401`](../../../../../../../crates/renovate-core/src/branch.rs#L1401) |
| 269 | realistic defaults | ported | [`crates/renovate-core/src/branch.rs:1528`](../../../../../../../crates/renovate-core/src/branch.rs#L1528) |
| 284 | realistic defaults with strict branch name enabled | ported | [`crates/renovate-core/src/branch.rs:1535`](../../../../../../../crates/renovate-core/src/branch.rs#L1535) |
| 300 | removes slashes from the non-suffix part | ported | [`crates/renovate-core/src/branch.rs:1545`](../../../../../../../crates/renovate-core/src/branch.rs#L1545) |
| 316 | hashedbranchlength hashing | ported | [`crates/renovate-core/src/branch.rs:1982`](../../../../../../../crates/renovate-core/src/branch.rs#L1982) |
| 332 | hashedbranchlength hashing with group name | ported | [`crates/renovate-core/src/branch.rs:1991`](../../../../../../../crates/renovate-core/src/branch.rs#L1991) |
| 350 | hashedbranchlength too short | ported | [`crates/renovate-core/src/branch.rs:2000`](../../../../../../../crates/renovate-core/src/branch.rs#L2000) |
| 368 | hashedbranchlength no topic | ported | [`crates/renovate-core/src/branch.rs:2006`](../../../../../../../crates/renovate-core/src/branch.rs#L2006) |
| 386 | hashedbranchlength separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2012`](../../../../../../../crates/renovate-core/src/branch.rs#L2012) |
| 405 | enforces valid git branch name | ported | [`crates/renovate-core/src/branch.rs:1555`](../../../../../../../crates/renovate-core/src/branch.rs#L1555) |
| 491 | strict branch name enabled group | ported | [`crates/renovate-core/src/branch.rs:1627`](../../../../../../../crates/renovate-core/src/branch.rs#L1627) |
| 506 | strict branch name disabled | ported | [`crates/renovate-core/src/branch.rs:1638`](../../../../../../../crates/renovate-core/src/branch.rs#L1638) |

