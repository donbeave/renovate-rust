# `lib/workers/repository/updates/branch-name.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**27/27 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | falls back to sharedvariablename if no groupname | ported | [`crates/renovate-core/src/branch.rs:1939`](../../../../../../../crates/renovate-core/src/branch.rs#L1939) |
| 19 | ignores grouping of replacement update | ported | [`crates/renovate-core/src/branch.rs:1950`](../../../../../../../crates/renovate-core/src/branch.rs#L1950) |
| 36 | applies grouping for lockfile maintenance update | ported | [`crates/renovate-core/src/branch.rs:1960`](../../../../../../../crates/renovate-core/src/branch.rs#L1960) |
| 52 | uses default branch name for lockfile maintenance without groupname | ported | [`crates/renovate-core/src/branch.rs:1971`](../../../../../../../crates/renovate-core/src/branch.rs#L1971) |
| 63 | separates lockfilemaintenance from non-lockfilemaintenance with same groupname | ported | [`crates/renovate-core/src/branch.rs:1981`](../../../../../../../crates/renovate-core/src/branch.rs#L1981) |
| 89 | uses groupname if no slug defined, ignores sharedvariablename | ported | [`crates/renovate-core/src/branch.rs:1992`](../../../../../../../crates/renovate-core/src/branch.rs#L1992) |
| 102 | compile groupname before slugging | ported | [`crates/renovate-core/src/branch.rs:2003`](../../../../../../../crates/renovate-core/src/branch.rs#L2003) |
| 115 | uses groupslug if defined | ported | [`crates/renovate-core/src/branch.rs:2014`](../../../../../../../crates/renovate-core/src/branch.rs#L2014) |
| 129 | separates major with groups | ported | [`crates/renovate-core/src/branch.rs:1356`](../../../../../../../crates/renovate-core/src/branch.rs#L1356) |
| 146 | separates minor with groups | ported | [`crates/renovate-core/src/branch.rs:2025`](../../../../../../../crates/renovate-core/src/branch.rs#L2025) |
| 163 | separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2037`](../../../../../../../crates/renovate-core/src/branch.rs#L2037) |
| 183 | uses single major with groups | ported | [`crates/renovate-core/src/branch.rs:1367`](../../../../../../../crates/renovate-core/src/branch.rs#L1367) |
| 200 | separates patch groups and uses update topic | ported | [`crates/renovate-core/src/branch.rs:2048`](../../../../../../../crates/renovate-core/src/branch.rs#L2048) |
| 218 | compiles multiple times | ported | [`crates/renovate-core/src/branch.rs:2060`](../../../../../../../crates/renovate-core/src/branch.rs#L2060) |
| 229 | separates patches when separateminorpatch=true | ported | [`crates/renovate-core/src/branch.rs:1265`](../../../../../../../crates/renovate-core/src/branch.rs#L1265) |
| 249 | does not separate patches when separateminorpatch=false | ported | [`crates/renovate-core/src/branch.rs:1285`](../../../../../../../crates/renovate-core/src/branch.rs#L1285) |
| 269 | realistic defaults | ported | [`crates/renovate-core/src/branch.rs:1412`](../../../../../../../crates/renovate-core/src/branch.rs#L1412) |
| 284 | realistic defaults with strict branch name enabled | ported | [`crates/renovate-core/src/branch.rs:1419`](../../../../../../../crates/renovate-core/src/branch.rs#L1419) |
| 300 | removes slashes from the non-suffix part | ported | [`crates/renovate-core/src/branch.rs:1429`](../../../../../../../crates/renovate-core/src/branch.rs#L1429) |
| 316 | hashedbranchlength hashing | ported | [`crates/renovate-core/src/branch.rs:1866`](../../../../../../../crates/renovate-core/src/branch.rs#L1866) |
| 332 | hashedbranchlength hashing with group name | ported | [`crates/renovate-core/src/branch.rs:1875`](../../../../../../../crates/renovate-core/src/branch.rs#L1875) |
| 350 | hashedbranchlength too short | ported | [`crates/renovate-core/src/branch.rs:1884`](../../../../../../../crates/renovate-core/src/branch.rs#L1884) |
| 368 | hashedbranchlength no topic | ported | [`crates/renovate-core/src/branch.rs:1890`](../../../../../../../crates/renovate-core/src/branch.rs#L1890) |
| 386 | hashedbranchlength separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:1896`](../../../../../../../crates/renovate-core/src/branch.rs#L1896) |
| 405 | enforces valid git branch name | ported | [`crates/renovate-core/src/branch.rs:1439`](../../../../../../../crates/renovate-core/src/branch.rs#L1439) |
| 491 | strict branch name enabled group | ported | [`crates/renovate-core/src/branch.rs:1511`](../../../../../../../crates/renovate-core/src/branch.rs#L1511) |
| 506 | strict branch name disabled | ported | [`crates/renovate-core/src/branch.rs:1522`](../../../../../../../crates/renovate-core/src/branch.rs#L1522) |

