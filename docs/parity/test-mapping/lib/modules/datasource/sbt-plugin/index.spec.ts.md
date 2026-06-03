# `lib/modules/datasource/sbt-plugin/index.spec.ts`

[← `datasource/sbt-plugin`](../../../../_by-module/datasource/sbt-plugin.md) · [all modules](../../../../README.md)

**7/8 in-scope tests ported** (1 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 15 | parses maven index directory | ported | [`crates/renovate-core/src/datasources/sbt_plugin.rs:337`](../../../../../../../crates/renovate-core/src/datasources/sbt_plugin.rs#L337) |
| 23 | parses sbt index directory | ported | [`crates/renovate-core/src/datasources/sbt_plugin.rs:354`](../../../../../../../crates/renovate-core/src/datasources/sbt_plugin.rs#L354) |
| 31 | uses proper hosttype | pending | — |
| 40 | returns null in case of errors | ported | [`crates/renovate-core/src/datasources/sbt_plugin.rs:370`](../../../../../../../crates/renovate-core/src/datasources/sbt_plugin.rs#L370) |
| 88 | fetches sbt plugins | ported | [`crates/renovate-core/src/datasources/sbt_plugin.rs:399`](../../../../../../../crates/renovate-core/src/datasources/sbt_plugin.rs#L399) |
| 157 | fetches sbt plugins 2 | ported | [`crates/renovate-core/src/datasources/sbt_plugin.rs:454`](../../../../../../../crates/renovate-core/src/datasources/sbt_plugin.rs#L454) |
| 226 | extracts url from maven pom file | ported | [`crates/renovate-core/src/datasources/sbt_plugin.rs:508`](../../../../../../../crates/renovate-core/src/datasources/sbt_plugin.rs#L508) |
| 312 | handles absolute and root relative paths | ported | [`crates/renovate-core/src/datasources/sbt_plugin.rs:609`](../../../../../../../crates/renovate-core/src/datasources/sbt_plugin.rs#L609) |

