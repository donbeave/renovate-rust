# `lib/modules/versioning/nuget/parser.spec.ts`

[← `versioning/nuget`](../../../../_by-module/versioning/nuget.md) · [all modules](../../../../README.md)

**10/15 ported** (5 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 13 | returns null for invalid input | ported | `crates/renovate-core/src/versioning/nuget.rs:1152` |
| 18 | parses version | ported | `crates/renovate-core/src/versioning/nuget.rs:1159` |
| 32 | rejects invalid input | ported | `crates/renovate-core/src/versioning/nuget.rs:1177` |
| 39 | _(it.each / template — verify manually)_ | ? | — |
| 78 | _(it.each / template — verify manually)_ | ? | — |
| 115 | rejects invalid input | ported | `crates/renovate-core/src/versioning/nuget.rs:1177` |
| 123 | parses exact range | ported | `crates/renovate-core/src/versioning/nuget.rs:1462` |
| 137 | rejects invalid input | ported | `crates/renovate-core/src/versioning/nuget.rs:1177` |
| 147 | parses range without lower bound | ported | `crates/renovate-core/src/versioning/nuget.rs:1494` |
| 157 | parses range without upper bound | ported | `crates/renovate-core/src/versioning/nuget.rs:1515` |
| 168 | _(it.each / template — verify manually)_ | ? | — |
| 185 | handles whitespaces | ported | `crates/renovate-core/src/versioning/nuget.rs:1575` |
| 195 | handles floating ranges as lower bounds | ported | `crates/renovate-core/src/versioning/nuget.rs:1603` |
| 224 | _(it.each / template — verify manually)_ | ? | — |
| 242 | _(it.each / template — verify manually)_ | ? | — |

