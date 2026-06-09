# `lib/modules/manager/npm/extract/npm.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns null if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3458`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3458) |
| 15 | extracts | ported | [`crates/renovate-core/src/extractors/npm.rs:3465`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3465) |
| 33 | extracts npm 7 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3497`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3497) |
| 51 | extracts npm 9 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3524`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3524) |
| 69 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:3556`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3556) |
| 75 | returns null on read error | ported | [`crates/renovate-core/src/extractors/npm.rs:3563`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3563) |

