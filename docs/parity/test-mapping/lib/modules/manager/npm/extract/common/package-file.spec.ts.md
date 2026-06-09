# `lib/modules/manager/npm/extract/common/package-file.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 20 | returns true for a valid packagemanager with name@version(e.g. pnpm@8.15.4) | ported | [`crates/renovate-core/src/extractors/npm.rs:6457`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6457) |
| 31 | returns true for a valid range like npm@^9 | ported | [`crates/renovate-core/src/extractors/npm.rs:6463`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6463) |
| 38 | returns true for yarn classic pin yarn@1.22.19 | ported | [`crates/renovate-core/src/extractors/npm.rs:6469`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6469) |
| 45 | returns false when packagemanager does not contain '@' (e.g. 'npm') | ported | [`crates/renovate-core/src/extractors/npm.rs:6475`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6475) |
| 52 | returns false when packagemanager is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6481`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6481) |
| 57 | returns false when package.json is invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:6487`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6487) |
| 62 | returns false if packagemanager is an empty string | ported | [`crates/renovate-core/src/extractors/npm.rs:6493`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6493) |

