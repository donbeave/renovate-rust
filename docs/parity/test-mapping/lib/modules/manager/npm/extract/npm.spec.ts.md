# `lib/modules/manager/npm/extract/npm.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns null if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3479`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3479) |
| 15 | extracts | ported | [`crates/renovate-core/src/extractors/npm.rs:3486`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3486) |
| 33 | extracts npm 7 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3518`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3518) |
| 51 | extracts npm 9 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3545`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3545) |
| 69 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:3577`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3577) |
| 75 | returns null on read error | ported | [`crates/renovate-core/src/extractors/npm.rs:3584`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3584) |

