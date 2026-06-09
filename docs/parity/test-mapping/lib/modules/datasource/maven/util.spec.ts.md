# `lib/modules/datasource/maven/util.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**14/16 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 53 | returns error for unsupported protocols | ported | [`crates/renovate-core/src/datasources/maven.rs:2998`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2998) |
| 64 | returns error for xml parse error | ported | [`crates/renovate-core/src/datasources/maven.rs:3006`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3006) |
| 85 | returns the downloaded text body | ported | [`crates/renovate-core/src/datasources/maven.rs:3021`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3021) |
| 102 | returns error for non-s3 urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3036`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3036) |
| 118 | _(it.each / template — verify manually)_ | ? | — |
| 168 | returns empty for host_disabled error | ported | [`crates/renovate-core/src/datasources/maven.rs:3448`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3448) |
| 179 | returns empty for host error | ported | [`crates/renovate-core/src/datasources/maven.rs:3392`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3392) |
| 190 | returns empty for temporary error | ported | [`crates/renovate-core/src/datasources/maven.rs:3401`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3401) |
| 213 | _(it.each / template — verify manually)_ | ? | — |
| 237 | throws externalhosterror for 429 status without redis cache | ported | [`crates/renovate-core/src/datasources/maven.rs:3410`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3410) |
| 258 | throws externalhosterror for non-429 temporary error on maven central | ported | [`crates/renovate-core/src/datasources/maven.rs:3420`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3420) |
| 273 | returns empty for connection error | ported | [`crates/renovate-core/src/datasources/maven.rs:3430`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3430) |
| 284 | returns empty for unsupported error | ported | [`crates/renovate-core/src/datasources/maven.rs:3439`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3439) |
| 302 | caches 404 for maven-metadata.xml urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3485`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3485) |
| 328 | does not cache 404 for non-metadata urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3503`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3503) |
| 344 | returns cached not-found without making http request | ported | [`crates/renovate-core/src/datasources/maven.rs:3520`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3520) |

