# `lib/modules/manager/npm/extract/npm.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns null if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3476`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3476) |
| 15 | extracts | ported | [`crates/renovate-core/src/extractors/npm.rs:3483`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3483) |
| 33 | extracts npm 7 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3515`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3515) |
| 51 | extracts npm 9 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3542`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3542) |
| 69 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:3574`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3574) |
| 75 | returns null on read error | ported | [`crates/renovate-core/src/extractors/npm.rs:3581`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3581) |

