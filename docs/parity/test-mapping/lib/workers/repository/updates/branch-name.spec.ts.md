# `lib/workers/repository/updates/branch-name.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**27/27 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 7 | falls back to sharedvariablename if no groupname | ported | [`crates/renovate-core/src/branch.rs:1921`](../../../../../../../crates/renovate-core/src/branch.rs#L1921) |
| 19 | ignores grouping of replacement update | ported | [`crates/renovate-core/src/branch.rs:1932`](../../../../../../../crates/renovate-core/src/branch.rs#L1932) |
| 36 | applies grouping for lockfile maintenance update | ported | [`crates/renovate-core/src/branch.rs:1942`](../../../../../../../crates/renovate-core/src/branch.rs#L1942) |
| 52 | uses default branch name for lockfile maintenance without groupname | ported | [`crates/renovate-core/src/branch.rs:1953`](../../../../../../../crates/renovate-core/src/branch.rs#L1953) |
| 63 | separates lockfilemaintenance from non-lockfilemaintenance with same groupname | ported | [`crates/renovate-core/src/branch.rs:1963`](../../../../../../../crates/renovate-core/src/branch.rs#L1963) |
| 89 | uses groupname if no slug defined, ignores sharedvariablename | ported | [`crates/renovate-core/src/branch.rs:1974`](../../../../../../../crates/renovate-core/src/branch.rs#L1974) |
| 102 | compile groupname before slugging | ported | [`crates/renovate-core/src/branch.rs:1985`](../../../../../../../crates/renovate-core/src/branch.rs#L1985) |
| 115 | uses groupslug if defined | ported | [`crates/renovate-core/src/branch.rs:1996`](../../../../../../../crates/renovate-core/src/branch.rs#L1996) |
| 129 | separates major with groups | ported | [`crates/renovate-core/src/branch.rs:1338`](../../../../../../../crates/renovate-core/src/branch.rs#L1338) |
| 146 | separates minor with groups | ported | [`crates/renovate-core/src/branch.rs:2007`](../../../../../../../crates/renovate-core/src/branch.rs#L2007) |
| 163 | separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2019`](../../../../../../../crates/renovate-core/src/branch.rs#L2019) |
| 183 | uses single major with groups | ported | [`crates/renovate-core/src/branch.rs:1349`](../../../../../../../crates/renovate-core/src/branch.rs#L1349) |
| 200 | separates patch groups and uses update topic | ported | [`crates/renovate-core/src/branch.rs:2030`](../../../../../../../crates/renovate-core/src/branch.rs#L2030) |
| 218 | compiles multiple times | ported | [`crates/renovate-core/src/branch.rs:2042`](../../../../../../../crates/renovate-core/src/branch.rs#L2042) |
| 229 | separates patches when separateminorpatch=true | ported | [`crates/renovate-core/src/branch.rs:1247`](../../../../../../../crates/renovate-core/src/branch.rs#L1247) |
| 249 | does not separate patches when separateminorpatch=false | ported | [`crates/renovate-core/src/branch.rs:1267`](../../../../../../../crates/renovate-core/src/branch.rs#L1267) |
| 269 | realistic defaults | ported | [`crates/renovate-core/src/branch.rs:1394`](../../../../../../../crates/renovate-core/src/branch.rs#L1394) |
| 284 | realistic defaults with strict branch name enabled | ported | [`crates/renovate-core/src/branch.rs:1401`](../../../../../../../crates/renovate-core/src/branch.rs#L1401) |
| 300 | removes slashes from the non-suffix part | ported | [`crates/renovate-core/src/branch.rs:1411`](../../../../../../../crates/renovate-core/src/branch.rs#L1411) |
| 316 | hashedbranchlength hashing | ported | [`crates/renovate-core/src/branch.rs:1848`](../../../../../../../crates/renovate-core/src/branch.rs#L1848) |
| 332 | hashedbranchlength hashing with group name | ported | [`crates/renovate-core/src/branch.rs:1857`](../../../../../../../crates/renovate-core/src/branch.rs#L1857) |
| 350 | hashedbranchlength too short | ported | [`crates/renovate-core/src/branch.rs:1866`](../../../../../../../crates/renovate-core/src/branch.rs#L1866) |
| 368 | hashedbranchlength no topic | ported | [`crates/renovate-core/src/branch.rs:1872`](../../../../../../../crates/renovate-core/src/branch.rs#L1872) |
| 386 | hashedbranchlength separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:1878`](../../../../../../../crates/renovate-core/src/branch.rs#L1878) |
| 405 | enforces valid git branch name | ported | [`crates/renovate-core/src/branch.rs:1421`](../../../../../../../crates/renovate-core/src/branch.rs#L1421) |
| 491 | strict branch name enabled group | ported | [`crates/renovate-core/src/branch.rs:1493`](../../../../../../../crates/renovate-core/src/branch.rs#L1493) |
| 506 | strict branch name disabled | ported | [`crates/renovate-core/src/branch.rs:1504`](../../../../../../../crates/renovate-core/src/branch.rs#L1504) |

