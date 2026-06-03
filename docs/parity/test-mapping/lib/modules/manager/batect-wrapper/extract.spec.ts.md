# `lib/modules/manager/batect-wrapper/extract.spec.ts`

[← `manager/batect-wrapper`](../../../../_by-module/manager/batect-wrapper.md) · [all modules](../../../../README.md)

**4/4 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 9 | returns null for empty wrapper file | ported | `crates/renovate-core/src/extractors/batect_wrapper.rs:63` |
| 13 | returns null for file without version information | ported | `crates/renovate-core/src/extractors/batect_wrapper.rs:57` |
| 17 | extracts the current version from a valid wrapper script | ported | `crates/renovate-core/src/extractors/batect_wrapper.rs:50` |
| 31 | returns the first version from a wrapper script with multiple versions | ported | `crates/renovate-core/src/extractors/batect_wrapper.rs:69` |

