# `lib/modules/datasource/maven/util.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**14/16 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 53 | returns error for unsupported protocols | ported | [`crates/renovate-core/src/datasources/maven.rs:2868`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2868) |
| 64 | returns error for xml parse error | ported | [`crates/renovate-core/src/datasources/maven.rs:2876`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2876) |
| 85 | returns the downloaded text body | ported | [`crates/renovate-core/src/datasources/maven.rs:2891`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2891) |
| 102 | returns error for non-s3 urls | ported | [`crates/renovate-core/src/datasources/maven.rs:2906`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2906) |
| 118 | _(it.each / template — verify manually)_ | ? | — |
| 168 | returns empty for host_disabled error | ported | [`crates/renovate-core/src/datasources/maven.rs:3318`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3318) |
| 179 | returns empty for host error | ported | [`crates/renovate-core/src/datasources/maven.rs:3262`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3262) |
| 190 | returns empty for temporary error | ported | [`crates/renovate-core/src/datasources/maven.rs:3271`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3271) |
| 213 | _(it.each / template — verify manually)_ | ? | — |
| 237 | throws externalhosterror for 429 status without redis cache | ported | [`crates/renovate-core/src/datasources/maven.rs:3280`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3280) |
| 258 | throws externalhosterror for non-429 temporary error on maven central | ported | [`crates/renovate-core/src/datasources/maven.rs:3290`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3290) |
| 273 | returns empty for connection error | ported | [`crates/renovate-core/src/datasources/maven.rs:3300`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3300) |
| 284 | returns empty for unsupported error | ported | [`crates/renovate-core/src/datasources/maven.rs:3309`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3309) |
| 302 | caches 404 for maven-metadata.xml urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3355`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3355) |
| 328 | does not cache 404 for non-metadata urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3373`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3373) |
| 344 | returns cached not-found without making http request | ported | [`crates/renovate-core/src/datasources/maven.rs:3390`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3390) |

