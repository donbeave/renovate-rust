# `lib/modules/manager/npm/extract/npm.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns null if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3475`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3475) |
| 15 | extracts | ported | [`crates/renovate-core/src/extractors/npm.rs:3482`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3482) |
| 33 | extracts npm 7 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3514`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3514) |
| 51 | extracts npm 9 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3541`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3541) |
| 69 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:3573`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3573) |
| 75 | returns null on read error | ported | [`crates/renovate-core/src/extractors/npm.rs:3580`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3580) |

