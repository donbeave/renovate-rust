# `lib/modules/manager/sbt/extract.spec.ts`

[← `manager/sbt`](../../../../_by-module/manager/sbt.md) · [all modules](../../../../README.md)

**26/26 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 23 | returns null for empty | ported | [`crates/renovate-core/src/extractors/sbt.rs:1067`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1067) |
| 47 | extracts deps for generic use-cases | ported | [`crates/renovate-core/src/extractors/sbt.rs:561`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L561) |
| 74 | extracts deps when scala version is defined in a variable | ported | [`crates/renovate-core/src/extractors/sbt.rs:700`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L700) |
| 99 | extracts deps when scala version is defined in an object | ported | [`crates/renovate-core/src/extractors/sbt.rs:645`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L645) |
| 136 | skips deps when dotted symbolds do not resolve to anything | ported | [`crates/renovate-core/src/extractors/sbt.rs:680`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L680) |
| 159 | extracts packagefileversion when scala version is defined in a variable | ported | [`crates/renovate-core/src/extractors/sbt.rs:937`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L937) |
| 170 | extracts typed variables | ported | [`crates/renovate-core/src/extractors/sbt.rs:801`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L801) |
| 185 | skips deps when scala version is missing | ported | [`crates/renovate-core/src/extractors/sbt.rs:764`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L764) |
| 213 | extract deps from native scala file with variables | ported | [`crates/renovate-core/src/extractors/sbt.rs:818`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L818) |
| 232 | extracts deps when scala version is defined with a trailing comma | ported | [`crates/renovate-core/src/extractors/sbt.rs:949`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L949) |
| 253 | extracts deps when scala version is defined in a variable with a trailing comma | ported | [`crates/renovate-core/src/extractors/sbt.rs:969`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L969) |
| 275 | extracts deps when scala version is defined with thisbuild scope | ported | [`crates/renovate-core/src/extractors/sbt.rs:989`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L989) |
| 294 | extracts correct scala library when dealing with scala 3 | ported | [`crates/renovate-core/src/extractors/sbt.rs:1006`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1006) |
| 309 | extracts deps correctly when dealing with scala 3 | ported | [`crates/renovate-core/src/extractors/sbt.rs:1019`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1019) |
| 329 | extracts deps when scala version is defined in a variable with thisbuild scope | ported | [`crates/renovate-core/src/extractors/sbt.rs:1036`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1036) |
| 349 | extract deps from native scala file with private variables | ported | [`crates/renovate-core/src/extractors/sbt.rs:864`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L864) |
| 371 | extract deps when they are defined in a new line | ported | [`crates/renovate-core/src/extractors/sbt.rs:902`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L902) |
| 412 | extract deps with comment | ported | [`crates/renovate-core/src/extractors/sbt.rs:616`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L616) |
| 452 | extract addcompilerplugin | ported | [`crates/renovate-core/src/extractors/sbt.rs:597`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L597) |
| 469 | extract sbt version | ported | [`crates/renovate-core/src/extractors/sbt.rs:1084`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1084) |
| 492 | extract sbt version if the file contains other properties | ported | [`crates/renovate-core/src/extractors/sbt.rs:1092`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1092) |
| 516 | ignores build.properties file if does not contain sbt version | ported | [`crates/renovate-core/src/extractors/sbt.rs:1100`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1100) |
| 529 | extracts proxy repositories | ported | [`crates/renovate-core/src/extractors/sbt.rs:1127`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1127) |
| 607 | should include default registryurls if no repositories file is provided | ported | [`crates/renovate-core/src/extractors/sbt.rs:1154`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1154) |
| 637 | should return empty packagefiles is no content is provided | ported | [`crates/renovate-core/src/extractors/sbt.rs:1107`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1107) |
| 643 | extracts build properties correctly | ported | [`crates/renovate-core/src/extractors/sbt.rs:1113`](../../../../../../../crates/renovate-core/src/extractors/sbt.rs#L1113) |

