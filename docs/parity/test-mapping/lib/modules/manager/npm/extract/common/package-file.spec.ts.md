# `lib/modules/manager/npm/extract/common/package-file.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**7/7 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 20 | returns true for a valid packagemanager with name@version(e.g. pnpm@8.15.4) | ported | [`crates/renovate-core/src/extractors/npm.rs:6410`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6410) |
| 31 | returns true for a valid range like npm@^9 | ported | [`crates/renovate-core/src/extractors/npm.rs:6416`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6416) |
| 38 | returns true for yarn classic pin yarn@1.22.19 | ported | [`crates/renovate-core/src/extractors/npm.rs:6422`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6422) |
| 45 | returns false when packagemanager does not contain '@' (e.g. 'npm') | ported | [`crates/renovate-core/src/extractors/npm.rs:6428`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6428) |
| 52 | returns false when packagemanager is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6434`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6434) |
| 57 | returns false when package.json is invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:6440`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6440) |
| 62 | returns false if packagemanager is an empty string | ported | [`crates/renovate-core/src/extractors/npm.rs:6446`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6446) |

