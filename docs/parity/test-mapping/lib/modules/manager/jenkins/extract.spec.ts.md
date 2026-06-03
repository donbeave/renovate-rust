# `lib/modules/manager/jenkins/extract.spec.ts`

[← `manager/jenkins`](../../../../_by-module/manager/jenkins.md) · [all modules](../../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 14 | returns empty list for an empty text file | ported | [`crates/renovate-core/src/extractors/jenkins.rs:294`](../../../../../../../crates/renovate-core/src/extractors/jenkins.rs#L294) |
| 19 | returns empty list for an empty yaml file | ported | [`crates/renovate-core/src/extractors/jenkins.rs:288`](../../../../../../../crates/renovate-core/src/extractors/jenkins.rs#L288) |
| 24 | returns empty list for an invalid yaml file | ported | [`crates/renovate-core/src/extractors/jenkins.rs:300`](../../../../../../../crates/renovate-core/src/extractors/jenkins.rs#L300) |
| 29 | extracts multiple image lines in text format | ported | [`crates/renovate-core/src/extractors/jenkins.rs:308`](../../../../../../../crates/renovate-core/src/extractors/jenkins.rs#L308) |
| 35 | extracts multiple image lines in yaml format | ported | [`crates/renovate-core/src/extractors/jenkins.rs:338`](../../../../../../../crates/renovate-core/src/extractors/jenkins.rs#L338) |

