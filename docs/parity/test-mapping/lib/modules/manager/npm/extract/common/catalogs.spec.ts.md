# `lib/modules/manager/npm/extract/common/catalogs.spec.ts`

[← `manager/npm`](../../../../../../_by-module/manager/npm.md) · [all modules](../../../../../../README.md)

**4/4 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 5 | returns correct dependencies for pnpm | ported | `crates/renovate-core/src/extractors/npm.rs:4793` |
| 39 | returns correct dependencies for yarn | ported | `crates/renovate-core/src/extractors/npm.rs:4806` |
| 73 | handles empty catalogs list | ported | `crates/renovate-core/src/extractors/npm.rs:4814` |
| 80 | handles catalog with no dependencies | ported | `crates/renovate-core/src/extractors/npm.rs:4821` |

