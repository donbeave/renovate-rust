# `lib/modules/manager/gradle/extract/consistent-versions-plugin.spec.ts`

[← `manager/gradle`](../../../../../_by-module/manager/gradle.md) · [all modules](../../../../../README.md)

**6/6 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | works for sub folders | ported | [`crates/renovate-core/src/extractors/gradle.rs:2732`](../../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2732) |
| 24 | detects lock file header introduced with gradle-consistent-versions version 2.20.0 | ported | [`crates/renovate-core/src/extractors/gradle.rs:2751`](../../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2751) |
| 36 | detects lock file header introduced with gradle-consistent-versions version 2.23.0 | ported | [`crates/renovate-core/src/extractors/gradle.rs:2762`](../../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2762) |
| 48 | correct position for crlf and lf | ported | [`crates/renovate-core/src/extractors/gradle.rs:2773`](../../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2773) |
| 60 | test bogus input lines | ported | [`crates/renovate-core/src/extractors/gradle.rs:2792`](../../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L2792) |
| 97 | supports multiple levels of glob | ported | [`crates/renovate-core/src/extractors/gradle.rs:3407`](../../../../../../../../crates/renovate-core/src/extractors/gradle.rs#L3407) |

