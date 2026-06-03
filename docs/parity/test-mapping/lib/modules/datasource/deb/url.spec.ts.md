# `lib/modules/datasource/deb/url.spec.ts`

[← `datasource/deb`](../../../../_by-module/datasource/deb.md) · [all modules](../../../../README.md)

**4/6 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 11 | constructs urls correctly from registry url with suite | ported | [`crates/renovate-core/src/datasources/deb.rs:102`](../../../../../../../crates/renovate-core/src/datasources/deb.rs#L102) |
| 22 | constructs urls correctly from registry url with deprecated release | ported | [`crates/renovate-core/src/datasources/deb.rs:117`](../../../../../../../crates/renovate-core/src/datasources/deb.rs#L117) |
| 33 | throws an error if required parameters are missing | ported | [`crates/renovate-core/src/datasources/deb.rs:192`](../../../../../../../crates/renovate-core/src/datasources/deb.rs#L192) |
| 41 | returns empty array for invalid registry url | ported | [`crates/renovate-core/src/datasources/deb.rs:184`](../../../../../../../crates/renovate-core/src/datasources/deb.rs#L184) |
| 49 | should return true for different status code | pending | — |
| 64 | should return true if request failed | pending | — |

