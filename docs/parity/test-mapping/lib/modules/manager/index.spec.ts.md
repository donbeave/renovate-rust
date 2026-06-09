# `lib/modules/manager/index.spec.ts`

[← `manager/_common`](../../../_by-module/manager/_common.md) · [all modules](../../../README.md)

**17/22 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | has valid supporteddatasources for ${m} | ported | [`crates/renovate-core/src/managers.rs:2598`](../../../../../../crates/renovate-core/src/managers.rs#L2598) |
| 31 | has lockfilenames for ${name} | ported | [`crates/renovate-core/src/managers.rs:2616`](../../../../../../crates/renovate-core/src/managers.rs#L2616) |
| 38 | gets something | ported | [`crates/renovate-core/src/managers.rs:2413`](../../../../../../crates/renovate-core/src/managers.rs#L2413) |
| 45 | gets | ported | [`crates/renovate-core/src/managers.rs:2369`](../../../../../../crates/renovate-core/src/managers.rs#L2369) |
| 51 | works | ported | [`crates/renovate-core/src/managers.rs:2379`](../../../../../../crates/renovate-core/src/managers.rs#L2379) |
| 60 | validates | ported | [`crates/renovate-core/src/managers.rs:2628`](../../../../../../crates/renovate-core/src/managers.rs#L2628) |
| 108 | iterates through managers | ported | [`crates/renovate-core/src/managers.rs:2429`](../../../../../../crates/renovate-core/src/managers.rs#L2429) |
| 114 | returns null | ported | [`crates/renovate-core/src/managers.rs:2648`](../../../../../../crates/renovate-core/src/managers.rs#L2648) |
| 127 | returns non-null | ported | [`crates/renovate-core/src/managers.rs:2663`](../../../../../../crates/renovate-core/src/managers.rs#L2663) |
| 144 | returns null | ported | [`crates/renovate-core/src/managers.rs:2648`](../../../../../../crates/renovate-core/src/managers.rs#L2648) |
| 157 | handles custom managers | ported | [`crates/renovate-core/src/managers.rs:2673`](../../../../../../crates/renovate-core/src/managers.rs#L2673) |
| 168 | returns non-null | ported | [`crates/renovate-core/src/managers.rs:2663`](../../../../../../crates/renovate-core/src/managers.rs#L2663) |
| 186 | returns null | ported | [`crates/renovate-core/src/managers.rs:2648`](../../../../../../crates/renovate-core/src/managers.rs#L2648) |
| 196 | returns non-null | ported | [`crates/renovate-core/src/managers.rs:2663`](../../../../../../crates/renovate-core/src/managers.rs#L2663) |
| 219 | returns update-lockfile for in-range-only | ported | [`crates/renovate-core/src/util.rs:9630`](../../../../../../crates/renovate-core/src/util.rs#L9630) |
| 232 | returns update-lockfile for in-range-only if it is proposed my manager | ported | [`crates/renovate-core/src/util.rs:9631`](../../../../../../crates/renovate-core/src/util.rs#L9631) |
| 252 | returns true | ported | [`crates/renovate-core/src/managers.rs:2414`](../../../../../../crates/renovate-core/src/managers.rs#L2414) |
| 258 | returns false | ported | [`crates/renovate-core/src/managers.rs:2415`](../../../../../../crates/renovate-core/src/managers.rs#L2415) |
| 265 | when no manager found, returns undefined | ported | [`crates/renovate-core/src/managers.rs:2396`](../../../../../../crates/renovate-core/src/managers.rs#L2396) |
| 271 | when manager found, but no prettydeptype found, returns undefined | ported | [`crates/renovate-core/src/managers.rs:2397`](../../../../../../crates/renovate-core/src/managers.rs#L2397) |
| 275 | when manager found, but no prettydeptype found, returns undefined | ported | [`crates/renovate-core/src/managers.rs:2397`](../../../../../../crates/renovate-core/src/managers.rs#L2397) |
| 279 | when manager found, and a prettydeptype found in knowndeptypes, returns the defined prettydeptype | ported | [`crates/renovate-core/src/managers.rs:2398`](../../../../../../crates/renovate-core/src/managers.rs#L2398) |

