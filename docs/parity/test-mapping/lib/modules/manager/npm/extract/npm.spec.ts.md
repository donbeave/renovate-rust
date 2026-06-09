# `lib/modules/manager/npm/extract/npm.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns null if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3474`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3474) |
| 15 | extracts | ported | [`crates/renovate-core/src/extractors/npm.rs:3481`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3481) |
| 33 | extracts npm 7 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3513`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3513) |
| 51 | extracts npm 9 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3540`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3540) |
| 69 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:3572`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3572) |
| 75 | returns null on read error | ported | [`crates/renovate-core/src/extractors/npm.rs:3579`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3579) |

