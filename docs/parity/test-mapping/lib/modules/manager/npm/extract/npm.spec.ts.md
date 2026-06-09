# `lib/modules/manager/npm/extract/npm.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns null if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3477`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3477) |
| 15 | extracts | ported | [`crates/renovate-core/src/extractors/npm.rs:3484`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3484) |
| 33 | extracts npm 7 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3516`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3516) |
| 51 | extracts npm 9 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3543`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3543) |
| 69 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:3575`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3575) |
| 75 | returns null on read error | ported | [`crates/renovate-core/src/extractors/npm.rs:3582`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3582) |

