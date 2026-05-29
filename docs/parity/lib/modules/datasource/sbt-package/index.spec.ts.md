# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/sbt-package/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/sbt-package/index.spec.ts
**Total tests:** 13 | **Ported:** 10 | **Actionable:** 13 | **Status:** partial

### `modules/datasource/sbt-package/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses Maven index directory | 16 | ported | `crates/renovate-core/src/datasources/sbt_package.rs` | `parses_maven_index_directory` | fixture-driven regex snapshot test |
| parses sbt index directory | 24 | ported | `crates/renovate-core/src/datasources/sbt_package.rs` | `parses_sbt_index_directory` | fixture-driven regex snapshot test |
| uses proper hostType | 32 | pending | — | — | —|
| returns null in case of errors | 41 | ported | `crates/renovate-core/src/datasources/sbt_package.rs` | `returns_null_in_case_of_errors` | all endpoints 404 → None |
| returns null if there is no version | 61 | ported | `crates/renovate-core/src/datasources/sbt_package.rs` | `returns_null_if_there_is_no_version` | empty artifact dirs → None; maven fallback 404 → None |
| fetches releases from Maven | 89 | ported | `crates/renovate-core/src/datasources/sbt_package.rs` | `fetches_releases_from_maven` | directory listing → [1.2.0, 1.2.3] |
| fetches Maven releases with Scala version | 142 | ported | `crates/renovate-core/src/datasources/sbt_package.rs` | `fetches_maven_releases_with_scala_version` | scalaVersion pin → only example_2.12 subdir |
| fetches releases from Confluent | 169 | ported | `crates/renovate-core/src/datasources/sbt_package.rs` | `fetches_releases_from_confluent` | absolute-path hrefs; POM with no URL tags |
| extracts URL from Maven POM file | 209 | ported | `crates/renovate-core/src/datasources/sbt_package.rs` | `extracts_url_from_maven_pom_file` | homepage + sourceUrl from POM; .git stripped |
| falls back to Maven for orgarization root folder non-listable repositories | 243 | ported | `crates/renovate-core/src/datasources/sbt_package.rs` | `falls_back_to_maven_for_non_listable_repositories` | directory listing 404 → maven-metadata.xml fallback |
| continues when parseUrl returns null for packageRootUrl | 284 | pending | — | — | —|
| skips pkgUrl when parseUrl returns null for it | 322 | pending | — | — | —|

### `modules/datasource/sbt-package/index > postprocessRelease`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts URL from Maven POM file | 364 | ported | `crates/renovate-core/src/datasources/sbt_package.rs` | `postprocess_extracts_release_timestamp` | last-modified header → ISO 8601 timestamp |

---
