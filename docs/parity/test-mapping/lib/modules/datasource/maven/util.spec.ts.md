# `lib/modules/datasource/maven/util.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**14/16 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 53 | returns error for unsupported protocols | ported | [`crates/renovate-core/src/datasources/maven.rs:2959`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2959) |
| 64 | returns error for xml parse error | ported | [`crates/renovate-core/src/datasources/maven.rs:2967`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2967) |
| 85 | returns the downloaded text body | ported | [`crates/renovate-core/src/datasources/maven.rs:2982`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2982) |
| 102 | returns error for non-s3 urls | ported | [`crates/renovate-core/src/datasources/maven.rs:2997`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2997) |
| 118 | _(it.each / template — verify manually)_ | ? | — |
| 168 | returns empty for host_disabled error | ported | [`crates/renovate-core/src/datasources/maven.rs:3409`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3409) |
| 179 | returns empty for host error | ported | [`crates/renovate-core/src/datasources/maven.rs:3353`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3353) |
| 190 | returns empty for temporary error | ported | [`crates/renovate-core/src/datasources/maven.rs:3362`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3362) |
| 213 | _(it.each / template — verify manually)_ | ? | — |
| 237 | throws externalhosterror for 429 status without redis cache | ported | [`crates/renovate-core/src/datasources/maven.rs:3371`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3371) |
| 258 | throws externalhosterror for non-429 temporary error on maven central | ported | [`crates/renovate-core/src/datasources/maven.rs:3381`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3381) |
| 273 | returns empty for connection error | ported | [`crates/renovate-core/src/datasources/maven.rs:3391`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3391) |
| 284 | returns empty for unsupported error | ported | [`crates/renovate-core/src/datasources/maven.rs:3400`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3400) |
| 302 | caches 404 for maven-metadata.xml urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3446`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3446) |
| 328 | does not cache 404 for non-metadata urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3464`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3464) |
| 344 | returns cached not-found without making http request | ported | [`crates/renovate-core/src/datasources/maven.rs:3481`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3481) |

