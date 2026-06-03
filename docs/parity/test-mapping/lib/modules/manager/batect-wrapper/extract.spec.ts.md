# `lib/modules/manager/batect-wrapper/extract.spec.ts`

[← `manager/batect-wrapper`](../../../../_by-module/manager/batect-wrapper.md) · [all modules](../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 9 | returns null for empty wrapper file | ported | [`crates/renovate-core/src/extractors/batect_wrapper.rs:63`](../../../../../../../crates/renovate-core/src/extractors/batect_wrapper.rs#L63) |
| 13 | returns null for file without version information | ported | [`crates/renovate-core/src/extractors/batect_wrapper.rs:57`](../../../../../../../crates/renovate-core/src/extractors/batect_wrapper.rs#L57) |
| 17 | extracts the current version from a valid wrapper script | ported | [`crates/renovate-core/src/extractors/batect_wrapper.rs:50`](../../../../../../../crates/renovate-core/src/extractors/batect_wrapper.rs#L50) |
| 31 | returns the first version from a wrapper script with multiple versions | ported | [`crates/renovate-core/src/extractors/batect_wrapper.rs:69`](../../../../../../../crates/renovate-core/src/extractors/batect_wrapper.rs#L69) |

