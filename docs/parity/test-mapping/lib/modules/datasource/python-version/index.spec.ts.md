# `lib/modules/datasource/python-version/index.spec.ts`

[← `datasource/python-version`](../../../../_by-module/datasource/python-version.md) · [all modules](../../../../README.md)

**10/10 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 14 | returns python eol data | ported | `crates/renovate-core/src/datasources/python_version.rs:240` |
| 63 | throws for 500 | ported | `crates/renovate-core/src/datasources/python_version.rs:263` |
| 73 | returns null for error | ported | `crates/renovate-core/src/datasources/python_version.rs:281` |
| 83 | falls back to prebuild releases on 429 | ported | `crates/renovate-core/src/datasources/python_version.rs:297` |
| 102 | returns null on 429 when prebuild releases are unavailable | ported | `crates/renovate-core/src/datasources/python_version.rs:336` |
| 116 | returns null for empty 200 ok | ported | `crates/renovate-core/src/datasources/python_version.rs:355` |
| 134 | returns the correct data | ported | `crates/renovate-core/src/datasources/python_version.rs:374` |
| 147 | only returns stable versions | ported | `crates/renovate-core/src/datasources/python_version.rs:402` |
| 158 | only returns versions that are prebuilt | ported | `crates/renovate-core/src/datasources/python_version.rs:426` |
| 170 | returns isdeprecated status for python 3 minor releases | ported | `crates/renovate-core/src/datasources/python_version.rs:452` |

