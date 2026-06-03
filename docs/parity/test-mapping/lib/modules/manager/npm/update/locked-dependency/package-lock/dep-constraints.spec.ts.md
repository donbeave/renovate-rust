# `lib/modules/manager/npm/update/locked-dependency/package-lock/dep-constraints.spec.ts`

[← `manager/npm`](../../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../../README.md)

**4/4 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | finds indirect dependency | ported | `crates/renovate-core/src/extractors/npm.rs:5349` |
| 29 | finds direct dependency | ported | `crates/renovate-core/src/extractors/npm.rs:5362` |
| 41 | skips non-matching direct dependency | ported | `crates/renovate-core/src/extractors/npm.rs:5374` |
| 53 | finds direct devdependency | ported | `crates/renovate-core/src/extractors/npm.rs:5384` |

