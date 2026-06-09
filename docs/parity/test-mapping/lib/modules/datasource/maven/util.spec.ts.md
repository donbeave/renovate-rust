# `lib/modules/datasource/maven/util.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**14/16 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 53 | returns error for unsupported protocols | ported | [`crates/renovate-core/src/datasources/maven.rs:2925`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2925) |
| 64 | returns error for xml parse error | ported | [`crates/renovate-core/src/datasources/maven.rs:2933`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2933) |
| 85 | returns the downloaded text body | ported | [`crates/renovate-core/src/datasources/maven.rs:2948`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2948) |
| 102 | returns error for non-s3 urls | ported | [`crates/renovate-core/src/datasources/maven.rs:2963`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2963) |
| 118 | _(it.each / template — verify manually)_ | ? | — |
| 168 | returns empty for host_disabled error | ported | [`crates/renovate-core/src/datasources/maven.rs:3375`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3375) |
| 179 | returns empty for host error | ported | [`crates/renovate-core/src/datasources/maven.rs:3319`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3319) |
| 190 | returns empty for temporary error | ported | [`crates/renovate-core/src/datasources/maven.rs:3328`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3328) |
| 213 | _(it.each / template — verify manually)_ | ? | — |
| 237 | throws externalhosterror for 429 status without redis cache | ported | [`crates/renovate-core/src/datasources/maven.rs:3337`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3337) |
| 258 | throws externalhosterror for non-429 temporary error on maven central | ported | [`crates/renovate-core/src/datasources/maven.rs:3347`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3347) |
| 273 | returns empty for connection error | ported | [`crates/renovate-core/src/datasources/maven.rs:3357`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3357) |
| 284 | returns empty for unsupported error | ported | [`crates/renovate-core/src/datasources/maven.rs:3366`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3366) |
| 302 | caches 404 for maven-metadata.xml urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3412`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3412) |
| 328 | does not cache 404 for non-metadata urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3430`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3430) |
| 344 | returns cached not-found without making http request | ported | [`crates/renovate-core/src/datasources/maven.rs:3447`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3447) |

