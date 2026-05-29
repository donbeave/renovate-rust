# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gradle-wrapper/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle-wrapper/util.spec.ts
**Total tests:** 14 | **Ported:** 12 | **Actionable:** 12 | **Status:** ported

### `getJavaConstraint() › returns Java constraint based on gradle support`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $gradleVersion \| $javaConstraint | 20 | ported | `gradle_wrapper.rs` | `java_constraint_from_gradle_version_cases` (11 cases) | — |
| returns toolChainVersion constraint if daemon JVM configured | 43 | ported | `gradle_wrapper.rs` | `toolchain_version_constraint_from_daemon_jvm_content` | — |
| returns languageVersion constraint if found | 52 | ported | `gradle_wrapper.rs` | `language_version_constraint_from_build_gradle_content` | — |

### `getJvmConfiguration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts toolChainVersion value | 63 | ported | `gradle_wrapper.rs` | `parse_jvm_toolchain_version_extracts_value` | — |
| returns null if gradle-daemon-jvm.properties file not found | 72 | ported | `gradle_wrapper.rs` | `parse_jvm_toolchain_version_returns_none_for_missing` | — |

### `getJavaLanguageVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extract languageVersion value | 83 | ported | `gradle_wrapper.rs` | `parse_java_language_version_extracts_value` | — |
| returns null if build.gradle or build.gradle.kts file not found | 92 | ported | `gradle_wrapper.rs` | `language_version_returns_none_for_empty_content` | — |
| returns null if build.gradle does not include languageVersion | 102 | ported | `gradle_wrapper.rs` | `parse_java_language_version_returns_none_for_no_pattern` | — |

### `extractGradleVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 113 | ported | `gradle_wrapper.rs` | `extract_gradle_version_returns_none_without_distribution_url` | — |
| returns gradle version | 121 | ported | `gradle_wrapper.rs` | `extract_gradle_version_returns_url_and_version` | — |

### `gradleWrapperFileName()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works on windows | 135 | ported | `gradle_wrapper.rs` | `gradle_wrapper_filename_windows` | cfg(target_os="windows") |
| works on linux | 140 | ported | `gradle_wrapper.rs` | `gradle_wrapper_filename_linux` | cfg(not(target_os="windows")) |

### `prepareGradleCommand`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 147 | not-applicable | — | — | mocking framework internals — platform.mockReturnValue + fs.statLocalFile.mockResolvedValue; TypeScript filesystem stat mock for executable check |
| returns null | 158 | not-applicable | — | — | mocking framework internals — fs.statLocalFile.mockResolvedValue for non-file stat; TypeScript filesystem mock |

---
