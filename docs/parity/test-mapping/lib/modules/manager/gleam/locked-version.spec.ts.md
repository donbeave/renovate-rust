# `lib/modules/manager/gleam/locked-version.spec.ts`

[← `manager/gleam`](../../../../_by-module/manager/gleam.md) · [all modules](../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 19 | returns null for missing lock file | ported | [`crates/renovate-core/src/extractors/gleam.rs:425`](../../../../../../../crates/renovate-core/src/extractors/gleam.rs#L425) |
| 23 | returns null for invalid lock file | ported | [`crates/renovate-core/src/extractors/gleam.rs:431`](../../../../../../../crates/renovate-core/src/extractors/gleam.rs#L431) |
| 28 | returns empty map for lock file without packages | ported | [`crates/renovate-core/src/extractors/gleam.rs:437`](../../../../../../../crates/renovate-core/src/extractors/gleam.rs#L437) |
| 33 | returns a map of package versions | ported | [`crates/renovate-core/src/extractors/gleam.rs:444`](../../../../../../../crates/renovate-core/src/extractors/gleam.rs#L444) |
| 45 | parses lockfile string into an object | ported | [`crates/renovate-core/src/extractors/gleam.rs:452`](../../../../../../../crates/renovate-core/src/extractors/gleam.rs#L452) |
| 63 | can deal with invalid lockfiles | ported | [`crates/renovate-core/src/extractors/gleam.rs:465`](../../../../../../../crates/renovate-core/src/extractors/gleam.rs#L465) |

