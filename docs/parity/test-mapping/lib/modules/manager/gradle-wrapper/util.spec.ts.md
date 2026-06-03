# `lib/modules/manager/gradle-wrapper/util.spec.ts`

[← `manager/gradle-wrapper`](../../../../_by-module/manager/gradle-wrapper.md) · [all modules](../../../../README.md)

**12/14 ported** (2 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 20 | _(it.each / template — verify manually)_ | ? | — |
| 43 | returns toolchainversion constraint if daemon jvm configured | ported | `crates/renovate-core/src/extractors/gradle_wrapper.rs:369` |
| 52 | returns languageversion constraint if found | ported | `crates/renovate-core/src/extractors/gradle_wrapper.rs:377` |
| 63 | extracts toolchainversion value | ported | `crates/renovate-core/src/extractors/gradle_wrapper.rs:340` |
| 72 | returns null if gradle-daemon-jvm.properties file not found | ported | `crates/renovate-core/src/extractors/gradle_wrapper.rs:347` |
| 83 | extract languageversion value | ported | `crates/renovate-core/src/extractors/gradle_wrapper.rs:362` |
| 92 | returns null if build.gradle or build.gradle.kts file not found | ported | `crates/renovate-core/src/extractors/gradle_wrapper.rs:385` |
| 102 | returns null if build.gradle does not include languageversion | ported | `crates/renovate-core/src/extractors/gradle_wrapper.rs:354` |
| 113 | returns null | ported | `crates/renovate-core/src/extractors/gradle_wrapper.rs:294` |
| 121 | returns gradle version | ported | `crates/renovate-core/src/extractors/gradle_wrapper.rs:301` |
| 135 | works on windows | ported | `crates/renovate-core/src/extractors/gradle_wrapper.rs:392` |
| 140 | works on linux | ported | `crates/renovate-core/src/extractors/gradle_wrapper.rs:399` |
| 147 | works | pending | — |
| 158 | returns null | ported | `crates/renovate-core/src/extractors/gradle_wrapper.rs:294` |

