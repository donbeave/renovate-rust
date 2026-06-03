# `lib/modules/manager/mix/extract.spec.ts`

[← `manager/mix`](../../../../_by-module/manager/mix.md) · [all modules](../../../../README.md)

**3/3 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | returns empty for invalid dependency file | ported | `crates/renovate-core/src/extractors/mix.rs:427` |
| 16 | extracts all dependencies when no lockfile | ported | `crates/renovate-core/src/extractors/mix.rs:249` |
| 139 | extracts all dependencies and adds the locked version if lockfile present | ported | `crates/renovate-core/src/extractors/mix.rs:388` |

