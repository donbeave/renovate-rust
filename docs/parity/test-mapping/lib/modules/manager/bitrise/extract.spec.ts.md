# `lib/modules/manager/bitrise/extract.spec.ts`

[← `manager/bitrise`](../../../../_by-module/manager/bitrise.md) · [all modules](../../../../README.md)

**6/6 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 7 | returns null on an empty file | ported | `crates/renovate-core/src/extractors/bitrise.rs:405` |
| 11 | returns a valid file | ported | `crates/renovate-core/src/extractors/bitrise.rs:328` |
| 34 | returns a valid file with custom default_step_lib_source | ported | `crates/renovate-core/src/extractors/bitrise.rs:342` |
| 75 | extracts git and path prefixes | ported | `crates/renovate-core/src/extractors/bitrise.rs:359` |
| 114 | handles workflows without steps | ported | `crates/renovate-core/src/extractors/bitrise.rs:433` |
| 142 | extracts bitrise library reference | ported | `crates/renovate-core/src/extractors/bitrise.rs:387` |

