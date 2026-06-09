# `lib/modules/manager/index.spec.ts`

[← `manager/_common`](../../../_by-module/manager/_common.md) · [all modules](../../../README.md)

**9/22 in-scope tests ported** (13 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | has valid supporteddatasources for ${m} | pending | — |
| 31 | has lockfilenames for ${name} | pending | — |
| 38 | gets something | ported | [`crates/renovate-core/src/managers.rs:2411`](../../../../../../crates/renovate-core/src/managers.rs#L2411) |
| 45 | gets | ported | [`crates/renovate-core/src/managers.rs:2367`](../../../../../../crates/renovate-core/src/managers.rs#L2367) |
| 51 | works | ported | [`crates/renovate-core/src/managers.rs:2377`](../../../../../../crates/renovate-core/src/managers.rs#L2377) |
| 60 | validates | pending | — |
| 108 | iterates through managers | ported | [`crates/renovate-core/src/managers.rs:2427`](../../../../../../crates/renovate-core/src/managers.rs#L2427) |
| 114 | returns null | pending | — |
| 127 | returns non-null | pending | — |
| 144 | returns null | pending | — |
| 157 | handles custom managers | pending | — |
| 168 | returns non-null | pending | — |
| 186 | returns null | pending | — |
| 196 | returns non-null | pending | — |
| 219 | returns update-lockfile for in-range-only | pending | — |
| 232 | returns update-lockfile for in-range-only if it is proposed my manager | pending | — |
| 252 | returns true | ported | [`crates/renovate-core/src/managers.rs:2412`](../../../../../../crates/renovate-core/src/managers.rs#L2412) |
| 258 | returns false | ported | [`crates/renovate-core/src/managers.rs:2413`](../../../../../../crates/renovate-core/src/managers.rs#L2413) |
| 265 | when no manager found, returns undefined | ported | [`crates/renovate-core/src/managers.rs:2394`](../../../../../../crates/renovate-core/src/managers.rs#L2394) |
| 271 | when manager found, but no prettydeptype found, returns undefined | ported | [`crates/renovate-core/src/managers.rs:2395`](../../../../../../crates/renovate-core/src/managers.rs#L2395) |
| 275 | when manager found, but no prettydeptype found, returns undefined | ported | [`crates/renovate-core/src/managers.rs:2395`](../../../../../../crates/renovate-core/src/managers.rs#L2395) |
| 279 | when manager found, and a prettydeptype found in knowndeptypes, returns the defined prettydeptype | ported | [`crates/renovate-core/src/managers.rs:2396`](../../../../../../crates/renovate-core/src/managers.rs#L2396) |

