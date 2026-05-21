# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gradle-wrapper/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle-wrapper/util.spec.ts
**Total tests:** 14 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `getJavaConstraint() › returns Java constraint based on gradle support`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $gradleVersion \| $javaConstraint | 20 | ported | `gradle_wrapper.rs` | `java_constraint_from_gradle_version_cases` (11 cases) | — |
| returns toolChainVersion constraint if daemon JVM configured | 43 | not-applicable | — | — | tests Gradle wrapper utility using Node.js os/fs modules; Rust would use std equivalents differently |
| returns languageVersion constraint if found | 52 | not-applicable | — | — | tests Gradle wrapper utility using Node.js os/fs modules; Rust would use std equivalents differently |

### `getJvmConfiguration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extracts toolChainVersion value | 63 | not-applicable | — | — | tests Gradle wrapper utility using Node.js os/fs modules; Rust would use std equivalents differently |
| returns null if gradle-daemon-jvm.properties file not found | 72 | not-applicable | — | — | tests Gradle wrapper utility using Node.js os/fs modules; Rust would use std equivalents differently |

### `getJavaLanguageVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| extract languageVersion value | 83 | not-applicable | — | — | tests Gradle wrapper utility using Node.js os/fs modules; Rust would use std equivalents differently |
| returns null if build.gradle or build.gradle.kts file not found | 92 | not-applicable | — | — | tests Gradle wrapper utility using Node.js os/fs modules; Rust would use std equivalents differently |
| returns null if build.gradle does not include languageVersion | 102 | not-applicable | — | — | tests Gradle wrapper utility using Node.js os/fs modules; Rust would use std equivalents differently |

### `extractGradleVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 113 | ported | `gradle_wrapper.rs` | `extract_gradle_version_returns_none_without_distribution_url` | — |
| returns gradle version | 121 | ported | `gradle_wrapper.rs` | `extract_gradle_version_returns_url_and_version` | — |

### `gradleWrapperFileName()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works on windows | 135 | not-applicable | — | — | tests Gradle wrapper utility using Node.js os/fs modules; Rust would use std equivalents differently |
| works on linux | 140 | not-applicable | — | — | tests Gradle wrapper utility using Node.js os/fs modules; Rust would use std equivalents differently |

### `prepareGradleCommand`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 147 | not-applicable | — | — | tests Gradle wrapper utility using Node.js os/fs modules; Rust would use std equivalents differently |
| returns null | 158 | not-applicable | — | — | tests Gradle wrapper utility using Node.js os/fs modules; Rust would use std equivalents differently |

---

