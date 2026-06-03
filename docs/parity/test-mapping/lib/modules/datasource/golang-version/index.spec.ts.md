# `lib/modules/datasource/golang-version/index.spec.ts`

[← `datasource/golang-version`](../../../../_by-module/datasource/golang-version.md) · [all modules](../../../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 19 | parses real data | ported | [`crates/renovate-core/src/datasources/golang_version.rs:228`](../../../../../../../crates/renovate-core/src/datasources/golang_version.rs#L228) |
| 36 | supports custom registry url | ported | [`crates/renovate-core/src/datasources/golang_version.rs:342`](../../../../../../../crates/renovate-core/src/datasources/golang_version.rs#L342) |
| 56 | throws externalhosterror for invalid release with no versions | ported | [`crates/renovate-core/src/datasources/golang_version.rs:252`](../../../../../../../crates/renovate-core/src/datasources/golang_version.rs#L252) |
| 69 | throws externalhosterror for invalid release with wrong termination | ported | [`crates/renovate-core/src/datasources/golang_version.rs:259`](../../../../../../../crates/renovate-core/src/datasources/golang_version.rs#L259) |
| 82 | throws externalhosterror for empty result | ported | [`crates/renovate-core/src/datasources/golang_version.rs:361`](../../../../../../../crates/renovate-core/src/datasources/golang_version.rs#L361) |
| 92 | throws externalhosterror for zero releases extracted | ported | [`crates/renovate-core/src/datasources/golang_version.rs:271`](../../../../../../../crates/renovate-core/src/datasources/golang_version.rs#L271) |
| 102 | throws externalhosterror for invalid release semver | ported | [`crates/renovate-core/src/datasources/golang_version.rs:287`](../../../../../../../crates/renovate-core/src/datasources/golang_version.rs#L287) |
| 112 | returns null for error 404 | ported | [`crates/renovate-core/src/datasources/golang_version.rs:327`](../../../../../../../crates/renovate-core/src/datasources/golang_version.rs#L327) |
| 122 | throws externalhosterror for invalid release format beginning | ported | [`crates/renovate-core/src/datasources/golang_version.rs:301`](../../../../../../../crates/renovate-core/src/datasources/golang_version.rs#L301) |
| 132 | throws externalhosterror for invalid release format | ported | [`crates/renovate-core/src/datasources/golang_version.rs:314`](../../../../../../../crates/renovate-core/src/datasources/golang_version.rs#L314) |

