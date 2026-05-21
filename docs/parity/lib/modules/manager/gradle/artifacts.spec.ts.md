# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gradle/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle/artifacts.spec.ts
**Total tests:** 27 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `isGradleExecutionAllowed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false when allowedUnsafeExecutions is empty (as it not enabled by default option) | 90 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns true when allowedUnsafeExecutions includes `gradleWrapper` | 101 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns false when allowedUnsafeExecutions does not include `gradleWrapper` | 112 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| logs when allowedUnsafeExecutions does not include `gradleWrapper` | 123 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

### `lockfile tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| aborts if no lockfile is found | 138 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| aborts if lock file exists but no gradle wrapper | 157 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| aborts if allowedUnsafeExecutions does not include `gradleWrapper` | 176 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| uses custom JVM heap settings when toolSettings are configured | 202 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates lock file | 247 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates lock file in win32 | 288 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| prefers packageName over depName if provided | 333 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| aborts lock file maintenance if packageFileName is not build.gradle(.kts) in root project | 378 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| performs lock file maintenance | 393 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| performs lock file maintenance (docker) | 431 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| performs lock file maintenance (install) | 495 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates all included projects | 534 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does not update lockfile if content is unchanged | 578 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| gradlew failed | 592 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| rethrows temporary error | 621 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| fallback to default Java version if Gradle version not extractable | 640 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

### `dependency verification tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates verification metadata file | 684 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| aborts verification metadata updates if allowedUnsafeExecutions does not include `gradleWrapper` | 731 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates existing checksums also if verify-checksums is disabled | 765 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates verification metadata and lock file | 820 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| uses sha256 as default if only weak hash algorithms are found | 894 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| uses pgp hashType if verify-signatures is enabled | 939 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does not write verification metadata, when no checksums exist and neither checksum nor signature verification is enabled | 983 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---

