# `lib/modules/manager/maven-wrapper/extract.spec.ts`

[← `manager/maven-wrapper`](../../../../_by-module/manager/maven-wrapper.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | extracts version for property file with distribution type "bin" in distributionurl | ported | [`crates/renovate-core/src/extractors/maven_wrapper.rs:137`](../../../../../../../crates/renovate-core/src/extractors/maven_wrapper.rs#L137) |
| 37 | extracts version for property file with only a wrapper url | ported | [`crates/renovate-core/src/extractors/maven_wrapper.rs:158`](../../../../../../../crates/renovate-core/src/extractors/maven_wrapper.rs#L158) |
| 51 | extracts version for property file with only a wrapper version | ported | [`crates/renovate-core/src/extractors/maven_wrapper.rs:176`](../../../../../../../crates/renovate-core/src/extractors/maven_wrapper.rs#L176) |
| 64 | extracts wrapper information from wrapperurl in precedence to wrapperversion | ported | [`crates/renovate-core/src/extractors/maven_wrapper.rs:187`](../../../../../../../crates/renovate-core/src/extractors/maven_wrapper.rs#L187) |
| 80 | extracts maven warapper version from mvnw file | ported | [`crates/renovate-core/src/extractors/maven_wrapper.rs:201`](../../../../../../../crates/renovate-core/src/extractors/maven_wrapper.rs#L201) |
| 93 | extracts maven warapper version from mvnw file - windows | ported | [`crates/renovate-core/src/extractors/maven_wrapper.rs:212`](../../../../../../../crates/renovate-core/src/extractors/maven_wrapper.rs#L212) |
| 106 | returns null for invalid wrapper version string in from mvnw file | ported | [`crates/renovate-core/src/extractors/maven_wrapper.rs:223`](../../../../../../../crates/renovate-core/src/extractors/maven_wrapper.rs#L223) |
| 111 | extracts version for property file with only a maven url | ported | [`crates/renovate-core/src/extractors/maven_wrapper.rs:230`](../../../../../../../crates/renovate-core/src/extractors/maven_wrapper.rs#L230) |
| 125 | should return null when there is no string matching the maven properties regex | ported | [`crates/renovate-core/src/extractors/maven_wrapper.rs:245`](../../../../../../../crates/renovate-core/src/extractors/maven_wrapper.rs#L245) |

