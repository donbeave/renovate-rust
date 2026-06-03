# `lib/modules/datasource/golang-version/index.spec.ts`

[← `datasource/golang-version`](../../../../_by-module/datasource/golang-version.md) · [all modules](../../../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 19 | parses real data | ported | `crates/renovate-core/src/datasources/golang_version.rs:228` |
| 36 | supports custom registry url | ported | `crates/renovate-core/src/datasources/golang_version.rs:342` |
| 56 | throws externalhosterror for invalid release with no versions | ported | `crates/renovate-core/src/datasources/golang_version.rs:252` |
| 69 | throws externalhosterror for invalid release with wrong termination | ported | `crates/renovate-core/src/datasources/golang_version.rs:259` |
| 82 | throws externalhosterror for empty result | ported | `crates/renovate-core/src/datasources/golang_version.rs:361` |
| 92 | throws externalhosterror for zero releases extracted | ported | `crates/renovate-core/src/datasources/golang_version.rs:271` |
| 102 | throws externalhosterror for invalid release semver | ported | `crates/renovate-core/src/datasources/golang_version.rs:287` |
| 112 | returns null for error 404 | ported | `crates/renovate-core/src/datasources/golang_version.rs:327` |
| 122 | throws externalhosterror for invalid release format beginning | ported | `crates/renovate-core/src/datasources/golang_version.rs:301` |
| 132 | throws externalhosterror for invalid release format | ported | `crates/renovate-core/src/datasources/golang_version.rs:314` |

