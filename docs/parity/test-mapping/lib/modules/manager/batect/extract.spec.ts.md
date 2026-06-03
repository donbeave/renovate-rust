# `lib/modules/manager/batect/extract.spec.ts`

[← `manager/batect`](../../../../_by-module/manager/batect.md) · [all modules](../../../../README.md)

**4/4 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 41 | returns empty array for empty configuration file | ported | [`crates/renovate-core/src/extractors/batect.rs:281`](../../../../../../../crates/renovate-core/src/extractors/batect.rs#L281) |
| 49 | returns empty array for non-object configuration file | ported | [`crates/renovate-core/src/extractors/batect.rs:287`](../../../../../../../crates/renovate-core/src/extractors/batect.rs#L287) |
| 57 | returns an a package file with no dependencies for configuration file without containers or includes | ported | [`crates/renovate-core/src/extractors/batect.rs:293`](../../../../../../../crates/renovate-core/src/extractors/batect.rs#L293) |
| 70 | extracts all available images and bundles from a valid batect configuration file, including dependencies in included files | ported | [`crates/renovate-core/src/extractors/batect.rs:255`](../../../../../../../crates/renovate-core/src/extractors/batect.rs#L255) |

