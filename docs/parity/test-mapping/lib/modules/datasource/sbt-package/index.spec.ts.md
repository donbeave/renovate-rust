# `lib/modules/datasource/sbt-package/index.spec.ts`

[← `datasource/sbt-package`](../../../../_by-module/datasource/sbt-package.md) · [all modules](../../../../README.md)

**11/13 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 18 | parses maven index directory | ported | `crates/renovate-core/src/datasources/sbt_package.rs:417` |
| 26 | parses sbt index directory | ported | `crates/renovate-core/src/datasources/sbt_package.rs:618` |
| 34 | uses proper hosttype | pending | — |
| 43 | returns null in case of errors | ported | `crates/renovate-core/src/datasources/sbt_package.rs:979` |
| 63 | returns null if there is no version | ported | `crates/renovate-core/src/datasources/sbt_package.rs:1009` |
| 91 | fetches releases from maven | ported | `crates/renovate-core/src/datasources/sbt_package.rs:1048` |
| 142 | fetches maven releases with scala version | ported | `crates/renovate-core/src/datasources/sbt_package.rs:1118` |
| 171 | fetches releases from confluent | ported | `crates/renovate-core/src/datasources/sbt_package.rs:1161` |
| 211 | extracts url from maven pom file | ported | `crates/renovate-core/src/datasources/sbt_package.rs:1207` |
| 245 | falls back to maven for orgarization root folder non-listable repositories | ported | `crates/renovate-core/src/datasources/sbt_package.rs:1253` |
| 285 | continues when parseurl returns null for packagerooturl | ported | `crates/renovate-core/src/datasources/sbt_package.rs:1322` |
| 323 | skips pkgurl when parseurl returns null for it | ported | `crates/renovate-core/src/datasources/sbt_package.rs:1365` |
| 366 | extracts url from maven pom file | ported | `crates/renovate-core/src/datasources/sbt_package.rs:1207` |

