# `lib/modules/datasource/dotnet-version/index.spec.ts`

[← `datasource/dotnet-version`](../../../../_by-module/datasource/dotnet-version.md) · [all modules](../../../../README.md)

**3/9 ported** (6 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 18 | returns null for non-dotnet package | ported | `crates/renovate-core/src/datasources/dotnet_version.rs:210` |
| 27 | returns null for 404 for index | pending | — |
| 38 | returns null for 404 for version | pending | — |
| 54 | throws for 5xx for index | pending | — |
| 65 | throws for 5xx for version | pending | — |
| 81 | returns null for unknown error for index | pending | — |
| 92 | returns null for unknown error for version | pending | — |
| 108 | returns real data for sdk | ported | `crates/renovate-core/src/datasources/dotnet_version.rs:230` |
| 159 | returns real data for runtime | ported | `crates/renovate-core/src/datasources/dotnet_version.rs:274` |

