# `lib/modules/datasource/maven/util.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**14/16 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 53 | returns error for unsupported protocols | ported | [`crates/renovate-core/src/datasources/maven.rs:2978`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2978) |
| 64 | returns error for xml parse error | ported | [`crates/renovate-core/src/datasources/maven.rs:2986`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2986) |
| 85 | returns the downloaded text body | ported | [`crates/renovate-core/src/datasources/maven.rs:3001`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3001) |
| 102 | returns error for non-s3 urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3016`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3016) |
| 118 | _(it.each / template — verify manually)_ | ? | — |
| 168 | returns empty for host_disabled error | ported | [`crates/renovate-core/src/datasources/maven.rs:3428`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3428) |
| 179 | returns empty for host error | ported | [`crates/renovate-core/src/datasources/maven.rs:3372`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3372) |
| 190 | returns empty for temporary error | ported | [`crates/renovate-core/src/datasources/maven.rs:3381`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3381) |
| 213 | _(it.each / template — verify manually)_ | ? | — |
| 237 | throws externalhosterror for 429 status without redis cache | ported | [`crates/renovate-core/src/datasources/maven.rs:3390`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3390) |
| 258 | throws externalhosterror for non-429 temporary error on maven central | ported | [`crates/renovate-core/src/datasources/maven.rs:3400`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3400) |
| 273 | returns empty for connection error | ported | [`crates/renovate-core/src/datasources/maven.rs:3410`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3410) |
| 284 | returns empty for unsupported error | ported | [`crates/renovate-core/src/datasources/maven.rs:3419`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3419) |
| 302 | caches 404 for maven-metadata.xml urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3465`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3465) |
| 328 | does not cache 404 for non-metadata urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3483`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3483) |
| 344 | returns cached not-found without making http request | ported | [`crates/renovate-core/src/datasources/maven.rs:3500`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3500) |

