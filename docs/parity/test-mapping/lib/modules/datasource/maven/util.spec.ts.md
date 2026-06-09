# `lib/modules/datasource/maven/util.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**14/16 in-scope tests ported** (2 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 53 | returns error for unsupported protocols | ported | [`crates/renovate-core/src/datasources/maven.rs:2907`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2907) |
| 64 | returns error for xml parse error | ported | [`crates/renovate-core/src/datasources/maven.rs:2915`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2915) |
| 85 | returns the downloaded text body | ported | [`crates/renovate-core/src/datasources/maven.rs:2930`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2930) |
| 102 | returns error for non-s3 urls | ported | [`crates/renovate-core/src/datasources/maven.rs:2945`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2945) |
| 118 | _(it.each / template — verify manually)_ | ? | — |
| 168 | returns empty for host_disabled error | ported | [`crates/renovate-core/src/datasources/maven.rs:3357`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3357) |
| 179 | returns empty for host error | ported | [`crates/renovate-core/src/datasources/maven.rs:3301`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3301) |
| 190 | returns empty for temporary error | ported | [`crates/renovate-core/src/datasources/maven.rs:3310`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3310) |
| 213 | _(it.each / template — verify manually)_ | ? | — |
| 237 | throws externalhosterror for 429 status without redis cache | ported | [`crates/renovate-core/src/datasources/maven.rs:3319`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3319) |
| 258 | throws externalhosterror for non-429 temporary error on maven central | ported | [`crates/renovate-core/src/datasources/maven.rs:3329`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3329) |
| 273 | returns empty for connection error | ported | [`crates/renovate-core/src/datasources/maven.rs:3339`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3339) |
| 284 | returns empty for unsupported error | ported | [`crates/renovate-core/src/datasources/maven.rs:3348`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3348) |
| 302 | caches 404 for maven-metadata.xml urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3394`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3394) |
| 328 | does not cache 404 for non-metadata urls | ported | [`crates/renovate-core/src/datasources/maven.rs:3412`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3412) |
| 344 | returns cached not-found without making http request | ported | [`crates/renovate-core/src/datasources/maven.rs:3429`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3429) |

