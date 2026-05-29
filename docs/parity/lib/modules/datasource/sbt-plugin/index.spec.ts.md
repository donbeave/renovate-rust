# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/sbt-plugin/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/sbt-plugin/index.spec.ts
**Total tests:** 8 | **Ported:** 7 | **Actionable:** 8 | **Status:** partial

### `modules/datasource/sbt-plugin/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses Maven index directory | 15 | ported | `crates/renovate-core/src/datasources/sbt_plugin.rs` | `parses_maven_index_directory` | reuses shared fixture from testdata/sbt/ |
| parses sbt index directory | 23 | ported | `crates/renovate-core/src/datasources/sbt_plugin.rs` | `parses_sbt_index_directory` | reuses shared fixture from testdata/sbt/ |
| uses proper hostType | 31 | pending | — | — | —|
| returns null in case of errors | 39 | ported | `crates/renovate-core/src/datasources/sbt_plugin.rs` | `returns_null_in_case_of_errors` | all endpoints 404 → None |
| fetches sbt plugins | 88 | ported | `crates/renovate-core/src/datasources/sbt_plugin.rs` | `fetches_sbt_plugins` | 3-level Ivy layout: artifact/scala_VERSION/sbt_VERSION/VERSION/ |
| fetches sbt plugins 2 | 157 | ported | `crates/renovate-core/src/datasources/sbt_plugin.rs` | `fetches_sbt_plugins_2` | same with scalaVersion suffix in package name |
| extracts URL from Maven POM file | 226 | ported | `crates/renovate-core/src/datasources/sbt_plugin.rs` | `extracts_url_from_maven_pom_file` | fallback to flat listing; POM homepage + sourceUrl |
| handles absolute and root relative paths | 312 | ported | `crates/renovate-core/src/datasources/sbt_plugin.rs` | `handles_absolute_and_root_relative_paths` | absolute and root-relative hrefs handled via last-segment extraction |

---
