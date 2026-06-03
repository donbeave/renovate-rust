# `lib/modules/datasource/python-version/index.spec.ts`

[← `datasource/python-version`](../../../../_by-module/datasource/python-version.md) · [all modules](../../../../README.md)

**10/10 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | returns python eol data | ported | [`crates/renovate-core/src/datasources/python_version.rs:240`](../../../../../../../crates/renovate-core/src/datasources/python_version.rs#L240) |
| 63 | throws for 500 | ported | [`crates/renovate-core/src/datasources/python_version.rs:263`](../../../../../../../crates/renovate-core/src/datasources/python_version.rs#L263) |
| 73 | returns null for error | ported | [`crates/renovate-core/src/datasources/python_version.rs:281`](../../../../../../../crates/renovate-core/src/datasources/python_version.rs#L281) |
| 83 | falls back to prebuild releases on 429 | ported | [`crates/renovate-core/src/datasources/python_version.rs:297`](../../../../../../../crates/renovate-core/src/datasources/python_version.rs#L297) |
| 102 | returns null on 429 when prebuild releases are unavailable | ported | [`crates/renovate-core/src/datasources/python_version.rs:336`](../../../../../../../crates/renovate-core/src/datasources/python_version.rs#L336) |
| 116 | returns null for empty 200 ok | ported | [`crates/renovate-core/src/datasources/python_version.rs:355`](../../../../../../../crates/renovate-core/src/datasources/python_version.rs#L355) |
| 134 | returns the correct data | ported | [`crates/renovate-core/src/datasources/python_version.rs:374`](../../../../../../../crates/renovate-core/src/datasources/python_version.rs#L374) |
| 147 | only returns stable versions | ported | [`crates/renovate-core/src/datasources/python_version.rs:402`](../../../../../../../crates/renovate-core/src/datasources/python_version.rs#L402) |
| 158 | only returns versions that are prebuilt | ported | [`crates/renovate-core/src/datasources/python_version.rs:426`](../../../../../../../crates/renovate-core/src/datasources/python_version.rs#L426) |
| 170 | returns isdeprecated status for python 3 minor releases | ported | [`crates/renovate-core/src/datasources/python_version.rs:452`](../../../../../../../crates/renovate-core/src/datasources/python_version.rs#L452) |

