# `lib/modules/manager/npm/extract/npm.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns null if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3481`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3481) |
| 15 | extracts | ported | [`crates/renovate-core/src/extractors/npm.rs:3488`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3488) |
| 33 | extracts npm 7 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3520`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3520) |
| 51 | extracts npm 9 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3547`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3547) |
| 69 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:3579`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3579) |
| 75 | returns null on read error | ported | [`crates/renovate-core/src/extractors/npm.rs:3586`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3586) |

