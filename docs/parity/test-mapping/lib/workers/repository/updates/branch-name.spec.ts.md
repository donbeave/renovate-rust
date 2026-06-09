# `lib/workers/repository/updates/branch-name.spec.ts`

[← `worker/repository`](../../../../_by-module/worker/repository.md) · [all modules](../../../../README.md)

**27/27 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 7 | falls back to sharedvariablename if no groupname | ported | [`crates/renovate-core/src/branch.rs:1920`](../../../../../../../crates/renovate-core/src/branch.rs#L1920) |
| 19 | ignores grouping of replacement update | ported | [`crates/renovate-core/src/branch.rs:1931`](../../../../../../../crates/renovate-core/src/branch.rs#L1931) |
| 36 | applies grouping for lockfile maintenance update | ported | [`crates/renovate-core/src/branch.rs:1941`](../../../../../../../crates/renovate-core/src/branch.rs#L1941) |
| 52 | uses default branch name for lockfile maintenance without groupname | ported | [`crates/renovate-core/src/branch.rs:1952`](../../../../../../../crates/renovate-core/src/branch.rs#L1952) |
| 63 | separates lockfilemaintenance from non-lockfilemaintenance with same groupname | ported | [`crates/renovate-core/src/branch.rs:1962`](../../../../../../../crates/renovate-core/src/branch.rs#L1962) |
| 89 | uses groupname if no slug defined, ignores sharedvariablename | ported | [`crates/renovate-core/src/branch.rs:1973`](../../../../../../../crates/renovate-core/src/branch.rs#L1973) |
| 102 | compile groupname before slugging | ported | [`crates/renovate-core/src/branch.rs:1984`](../../../../../../../crates/renovate-core/src/branch.rs#L1984) |
| 115 | uses groupslug if defined | ported | [`crates/renovate-core/src/branch.rs:1995`](../../../../../../../crates/renovate-core/src/branch.rs#L1995) |
| 129 | separates major with groups | ported | [`crates/renovate-core/src/branch.rs:1337`](../../../../../../../crates/renovate-core/src/branch.rs#L1337) |
| 146 | separates minor with groups | ported | [`crates/renovate-core/src/branch.rs:2006`](../../../../../../../crates/renovate-core/src/branch.rs#L2006) |
| 163 | separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:2018`](../../../../../../../crates/renovate-core/src/branch.rs#L2018) |
| 183 | uses single major with groups | ported | [`crates/renovate-core/src/branch.rs:1348`](../../../../../../../crates/renovate-core/src/branch.rs#L1348) |
| 200 | separates patch groups and uses update topic | ported | [`crates/renovate-core/src/branch.rs:2029`](../../../../../../../crates/renovate-core/src/branch.rs#L2029) |
| 218 | compiles multiple times | ported | [`crates/renovate-core/src/branch.rs:2041`](../../../../../../../crates/renovate-core/src/branch.rs#L2041) |
| 229 | separates patches when separateminorpatch=true | ported | [`crates/renovate-core/src/branch.rs:1246`](../../../../../../../crates/renovate-core/src/branch.rs#L1246) |
| 249 | does not separate patches when separateminorpatch=false | ported | [`crates/renovate-core/src/branch.rs:1266`](../../../../../../../crates/renovate-core/src/branch.rs#L1266) |
| 269 | realistic defaults | ported | [`crates/renovate-core/src/branch.rs:1393`](../../../../../../../crates/renovate-core/src/branch.rs#L1393) |
| 284 | realistic defaults with strict branch name enabled | ported | [`crates/renovate-core/src/branch.rs:1400`](../../../../../../../crates/renovate-core/src/branch.rs#L1400) |
| 300 | removes slashes from the non-suffix part | ported | [`crates/renovate-core/src/branch.rs:1410`](../../../../../../../crates/renovate-core/src/branch.rs#L1410) |
| 316 | hashedbranchlength hashing | ported | [`crates/renovate-core/src/branch.rs:1847`](../../../../../../../crates/renovate-core/src/branch.rs#L1847) |
| 332 | hashedbranchlength hashing with group name | ported | [`crates/renovate-core/src/branch.rs:1856`](../../../../../../../crates/renovate-core/src/branch.rs#L1856) |
| 350 | hashedbranchlength too short | ported | [`crates/renovate-core/src/branch.rs:1865`](../../../../../../../crates/renovate-core/src/branch.rs#L1865) |
| 368 | hashedbranchlength no topic | ported | [`crates/renovate-core/src/branch.rs:1871`](../../../../../../../crates/renovate-core/src/branch.rs#L1871) |
| 386 | hashedbranchlength separates minor when separatemultipleminor=true | ported | [`crates/renovate-core/src/branch.rs:1877`](../../../../../../../crates/renovate-core/src/branch.rs#L1877) |
| 405 | enforces valid git branch name | ported | [`crates/renovate-core/src/branch.rs:1420`](../../../../../../../crates/renovate-core/src/branch.rs#L1420) |
| 491 | strict branch name enabled group | ported | [`crates/renovate-core/src/branch.rs:1492`](../../../../../../../crates/renovate-core/src/branch.rs#L1492) |
| 506 | strict branch name disabled | ported | [`crates/renovate-core/src/branch.rs:1503`](../../../../../../../crates/renovate-core/src/branch.rs#L1503) |

