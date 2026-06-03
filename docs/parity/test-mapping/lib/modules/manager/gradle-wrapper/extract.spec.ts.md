# `lib/modules/manager/gradle-wrapper/extract.spec.ts`

[← `manager/gradle-wrapper`](../../../../_by-module/manager/gradle-wrapper.md) · [all modules](../../../../README.md)

**8/8 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 24 | returns null for property file without distributionurl | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:238`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L238) |
| 28 | returns null for property file with unsupported distributionurl format | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:267`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L267) |
| 33 | extracts version for property file with distribution type "bin" in distributionurl | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:206`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L206) |
| 47 | extracts version for property file with distribution type "all" in distributionurl | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:215`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L215) |
| 61 | extracts version for property file with prerelease version in distributionurl | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:250`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L250) |
| 75 | extracts version for property file with unnecessary whitespace in distributionurl | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:258`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L258) |
| 89 | extracts version for property file with custom distribution of type "bin" in distributionurl | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:274`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L274) |
| 103 | extracts version for property file with custom distribution of type "all" in distributionurl | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:283`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L283) |

