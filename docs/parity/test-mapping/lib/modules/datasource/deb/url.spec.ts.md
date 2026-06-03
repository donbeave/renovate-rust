# `lib/modules/datasource/deb/url.spec.ts`

[← `datasource/deb`](../../../../_by-module/datasource/deb.md) · [all modules](../../../../README.md)

**4/6 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 11 | constructs urls correctly from registry url with suite | ported | `crates/renovate-core/src/datasources/deb.rs:102` |
| 22 | constructs urls correctly from registry url with deprecated release | ported | `crates/renovate-core/src/datasources/deb.rs:117` |
| 33 | throws an error if required parameters are missing | ported | `crates/renovate-core/src/datasources/deb.rs:192` |
| 41 | returns empty array for invalid registry url | ported | `crates/renovate-core/src/datasources/deb.rs:184` |
| 49 | should return true for different status code | pending | — |
| 64 | should return true if request failed | pending | — |

