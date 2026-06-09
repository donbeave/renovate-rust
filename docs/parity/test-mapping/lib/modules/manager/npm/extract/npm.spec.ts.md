# `lib/modules/manager/npm/extract/npm.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns null if failed to parse | ported | [`crates/renovate-core/src/extractors/npm.rs:3473`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3473) |
| 15 | extracts | ported | [`crates/renovate-core/src/extractors/npm.rs:3480`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3480) |
| 33 | extracts npm 7 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3512`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3512) |
| 51 | extracts npm 9 lockfile | ported | [`crates/renovate-core/src/extractors/npm.rs:3539`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3539) |
| 69 | returns null if no deps | ported | [`crates/renovate-core/src/extractors/npm.rs:3571`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3571) |
| 75 | returns null on read error | ported | [`crates/renovate-core/src/extractors/npm.rs:3578`](../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L3578) |

