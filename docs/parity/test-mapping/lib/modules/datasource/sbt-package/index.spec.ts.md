# `lib/modules/datasource/sbt-package/index.spec.ts`

[← `datasource/sbt-package`](../../../../_by-module/datasource/sbt-package.md) · [all modules](../../../../README.md)

**11/12 in-scope tests ported** (1 pending, 1 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | parses maven index directory | ported | [`crates/renovate-core/src/datasources/sbt_package.rs:417`](../../../../../../../crates/renovate-core/src/datasources/sbt_package.rs#L417) |
| 26 | parses sbt index directory | ported | [`crates/renovate-core/src/datasources/sbt_package.rs:618`](../../../../../../../crates/renovate-core/src/datasources/sbt_package.rs#L618) |
| 34 | uses proper hosttype | opt-out | asserts a TypeScript class property shape (http.hostType) with no Rust equivalent |
| 43 | returns null in case of errors | ported | [`crates/renovate-core/src/datasources/sbt_package.rs:979`](../../../../../../../crates/renovate-core/src/datasources/sbt_package.rs#L979) |
| 63 | returns null if there is no version | ported | [`crates/renovate-core/src/datasources/sbt_package.rs:1009`](../../../../../../../crates/renovate-core/src/datasources/sbt_package.rs#L1009) |
| 91 | fetches releases from maven | ported | [`crates/renovate-core/src/datasources/sbt_package.rs:1048`](../../../../../../../crates/renovate-core/src/datasources/sbt_package.rs#L1048) |
| 142 | fetches maven releases with scala version | ported | [`crates/renovate-core/src/datasources/sbt_package.rs:1118`](../../../../../../../crates/renovate-core/src/datasources/sbt_package.rs#L1118) |
| 171 | fetches releases from confluent | ported | [`crates/renovate-core/src/datasources/sbt_package.rs:1161`](../../../../../../../crates/renovate-core/src/datasources/sbt_package.rs#L1161) |
| 211 | extracts url from maven pom file | ported | [`crates/renovate-core/src/datasources/sbt_package.rs:1207`](../../../../../../../crates/renovate-core/src/datasources/sbt_package.rs#L1207) |
| 245 | falls back to maven for orgarization root folder non-listable repositories | ported | [`crates/renovate-core/src/datasources/sbt_package.rs:1253`](../../../../../../../crates/renovate-core/src/datasources/sbt_package.rs#L1253) |
| 285 | continues when parseurl returns null for packagerooturl | ported | [`crates/renovate-core/src/datasources/sbt_package.rs:1322`](../../../../../../../crates/renovate-core/src/datasources/sbt_package.rs#L1322) |
| 323 | skips pkgurl when parseurl returns null for it | ported | [`crates/renovate-core/src/datasources/sbt_package.rs:1365`](../../../../../../../crates/renovate-core/src/datasources/sbt_package.rs#L1365) |
| 366 | extracts url from maven pom file | ported | [`crates/renovate-core/src/datasources/sbt_package.rs:1207`](../../../../../../../crates/renovate-core/src/datasources/sbt_package.rs#L1207) |

