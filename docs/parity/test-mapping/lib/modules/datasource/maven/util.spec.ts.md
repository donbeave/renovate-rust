# `lib/modules/datasource/maven/util.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**14/16 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 53 | returns error for unsupported protocols | ported | [`crates/renovate-core/src/datasources/maven.rs:3004`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3004) |
| 64 | returns error for xml parse error | ported | [`crates/renovate-core/src/datasources/maven.rs:3012`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3012) |
| 85 | returns the downloaded text body | ported | [`crates/renovate-core/src/datasources/maven.rs:3027`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3027) |
| 102 | returns error for non-s3 urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3042`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3042) |
| 118 | _(it.each / template — verify manually)_ | ? | — |
| 168 | returns empty for host_disabled error | ported | [`crates/renovate-core/src/datasources/maven.rs:3454`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3454) |
| 179 | returns empty for host error | ported | [`crates/renovate-core/src/datasources/maven.rs:3398`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3398) |
| 190 | returns empty for temporary error | ported | [`crates/renovate-core/src/datasources/maven.rs:3407`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3407) |
| 213 | _(it.each / template — verify manually)_ | ? | — |
| 237 | throws externalhosterror for 429 status without redis cache | ported | [`crates/renovate-core/src/datasources/maven.rs:3416`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3416) |
| 258 | throws externalhosterror for non-429 temporary error on maven central | ported | [`crates/renovate-core/src/datasources/maven.rs:3426`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3426) |
| 273 | returns empty for connection error | ported | [`crates/renovate-core/src/datasources/maven.rs:3436`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3436) |
| 284 | returns empty for unsupported error | ported | [`crates/renovate-core/src/datasources/maven.rs:3445`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3445) |
| 302 | caches 404 for maven-metadata.xml urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3491`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3491) |
| 328 | does not cache 404 for non-metadata urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3509`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3509) |
| 344 | returns cached not-found without making http request | ported | [`crates/renovate-core/src/datasources/maven.rs:3526`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3526) |

