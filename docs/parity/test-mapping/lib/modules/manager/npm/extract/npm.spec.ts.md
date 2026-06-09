# `lib/modules/manager/npm/extract/npm.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns null if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3485`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3485) |
| 15 | extracts | ported | [`crates/renovate-core/src/extractors/npm.rs:3492`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3492) |
| 33 | extracts npm 7 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3524`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3524) |
| 51 | extracts npm 9 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3551`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3551) |
| 69 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:3583`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3583) |
| 75 | returns null on read error | ported | [`crates/renovate-core/src/extractors/npm.rs:3590`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3590) |

