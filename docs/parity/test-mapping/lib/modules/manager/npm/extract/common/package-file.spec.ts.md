# `lib/modules/manager/npm/extract/common/package-file.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 20 | returns true for a valid packagemanager with name@version(e.g. pnpm@8.15.4) | ported | [`crates/renovate-core/src/extractors/npm.rs:6452`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6452) |
| 31 | returns true for a valid range like npm@^9 | ported | [`crates/renovate-core/src/extractors/npm.rs:6458`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6458) |
| 38 | returns true for yarn classic pin yarn@1.22.19 | ported | [`crates/renovate-core/src/extractors/npm.rs:6464`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6464) |
| 45 | returns false when packagemanager does not contain '@' (e.g. 'npm') | ported | [`crates/renovate-core/src/extractors/npm.rs:6470`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6470) |
| 52 | returns false when packagemanager is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6476`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6476) |
| 57 | returns false when package.json is invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:6482`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6482) |
| 62 | returns false if packagemanager is an empty string | ported | [`crates/renovate-core/src/extractors/npm.rs:6488`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6488) |

