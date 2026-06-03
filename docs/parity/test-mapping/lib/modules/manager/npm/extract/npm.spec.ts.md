# `lib/modules/manager/npm/extract/npm.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns null if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3431`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3431) |
| 15 | extracts | ported | [`crates/renovate-core/src/extractors/npm.rs:3438`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3438) |
| 33 | extracts npm 7 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3470`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3470) |
| 51 | extracts npm 9 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3497`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3497) |
| 69 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:3529`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3529) |
| 75 | returns null on read error | ported | [`crates/renovate-core/src/extractors/npm.rs:3536`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3536) |

