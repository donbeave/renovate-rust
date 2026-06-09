# `lib/modules/manager/index.spec.ts`

[← `manager/_common`](../../../_by-module/manager/_common.md) · [all modules](../../../README.md)

**9/22 in-scope tests ported** (13 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | has valid supporteddatasources for ${m} | pending | — |
| 31 | has lockfilenames for ${name} | pending | — |
| 38 | gets something | ported | [`crates/renovate-core/src/managers.rs:2406`](../../../../../../crates/renovate-core/src/managers.rs#L2406) |
| 45 | gets | ported | [`crates/renovate-core/src/managers.rs:2362`](../../../../../../crates/renovate-core/src/managers.rs#L2362) |
| 51 | works | ported | [`crates/renovate-core/src/managers.rs:2372`](../../../../../../crates/renovate-core/src/managers.rs#L2372) |
| 60 | validates | pending | — |
| 108 | iterates through managers | ported | [`crates/renovate-core/src/managers.rs:2422`](../../../../../../crates/renovate-core/src/managers.rs#L2422) |
| 114 | returns null | pending | — |
| 127 | returns non-null | pending | — |
| 144 | returns null | pending | — |
| 157 | handles custom managers | pending | — |
| 168 | returns non-null | pending | — |
| 186 | returns null | pending | — |
| 196 | returns non-null | pending | — |
| 219 | returns update-lockfile for in-range-only | pending | — |
| 232 | returns update-lockfile for in-range-only if it is proposed my manager | pending | — |
| 252 | returns true | ported | [`crates/renovate-core/src/managers.rs:2407`](../../../../../../crates/renovate-core/src/managers.rs#L2407) |
| 258 | returns false | ported | [`crates/renovate-core/src/managers.rs:2408`](../../../../../../crates/renovate-core/src/managers.rs#L2408) |
| 265 | when no manager found, returns undefined | ported | [`crates/renovate-core/src/managers.rs:2389`](../../../../../../crates/renovate-core/src/managers.rs#L2389) |
| 271 | when manager found, but no prettydeptype found, returns undefined | ported | [`crates/renovate-core/src/managers.rs:2390`](../../../../../../crates/renovate-core/src/managers.rs#L2390) |
| 275 | when manager found, but no prettydeptype found, returns undefined | ported | [`crates/renovate-core/src/managers.rs:2390`](../../../../../../crates/renovate-core/src/managers.rs#L2390) |
| 279 | when manager found, and a prettydeptype found in knowndeptypes, returns the defined prettydeptype | ported | [`crates/renovate-core/src/managers.rs:2391`](../../../../../../crates/renovate-core/src/managers.rs#L2391) |

