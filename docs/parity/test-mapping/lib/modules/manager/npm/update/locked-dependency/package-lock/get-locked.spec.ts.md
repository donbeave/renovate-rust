# `lib/modules/manager/npm/update/locked-dependency/package-lock/get-locked.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | handles error | ported | `crates/renovate-core/src/extractors/npm.rs:5405` |
| 17 | returns empty if failed to parse | ported | `crates/renovate-core/src/extractors/npm.rs:5417` |
| 21 | finds direct dependency | ported | `crates/renovate-core/src/extractors/npm.rs:5429` |
| 32 | finds indirect dependency | ported | `crates/renovate-core/src/extractors/npm.rs:5444` |
| 43 | finds any version | ported | `crates/renovate-core/src/extractors/npm.rs:5453` |
| 49 | finds bundled dependency | ported | `crates/renovate-core/src/extractors/npm.rs:5461` |

