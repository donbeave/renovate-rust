# `lib/modules/manager/npm/extract/common/package-file.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 20 | returns true for a valid packagemanager with name@version(e.g. pnpm@8.15.4) | ported | [`crates/renovate-core/src/extractors/npm.rs:6459`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6459) |
| 31 | returns true for a valid range like npm@^9 | ported | [`crates/renovate-core/src/extractors/npm.rs:6465`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6465) |
| 38 | returns true for yarn classic pin yarn@1.22.19 | ported | [`crates/renovate-core/src/extractors/npm.rs:6471`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6471) |
| 45 | returns false when packagemanager does not contain '@' (e.g. 'npm') | ported | [`crates/renovate-core/src/extractors/npm.rs:6477`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6477) |
| 52 | returns false when packagemanager is missing | ported | [`crates/renovate-core/src/extractors/npm.rs:6483`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6483) |
| 57 | returns false when package.json is invalid | ported | [`crates/renovate-core/src/extractors/npm.rs:6489`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6489) |
| 62 | returns false if packagemanager is an empty string | ported | [`crates/renovate-core/src/extractors/npm.rs:6495`](../../../../../../../../../crates/renovate-core/src/extractors/npm.rs#L6495) |

