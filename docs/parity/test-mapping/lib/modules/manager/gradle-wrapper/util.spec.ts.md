# `lib/modules/manager/gradle-wrapper/util.spec.ts`

[← `manager/gradle-wrapper`](../../../../_by-module/manager/gradle-wrapper.md) · [all modules](../../../../README.md)

**12/14 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 20 | _(it.each / template — verify manually)_ | ? | — |
| 43 | returns toolchainversion constraint if daemon jvm configured | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:369`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L369) |
| 52 | returns languageversion constraint if found | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:377`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L377) |
| 63 | extracts toolchainversion value | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:340`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L340) |
| 72 | returns null if gradle-daemon-jvm.properties file not found | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:347`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L347) |
| 83 | extract languageversion value | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:362`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L362) |
| 92 | returns null if build.gradle or build.gradle.kts file not found | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:385`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L385) |
| 102 | returns null if build.gradle does not include languageversion | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:354`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L354) |
| 113 | returns null | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:294`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L294) |
| 121 | returns gradle version | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:301`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L301) |
| 135 | works on windows | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:392`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L392) |
| 140 | works on linux | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:399`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L399) |
| 147 | works | pending | — |
| 158 | returns null | ported | [`crates/renovate-core/src/extractors/gradle_wrapper.rs:294`](../../../../../../../crates/renovate-core/src/extractors/gradle_wrapper.rs#L294) |

