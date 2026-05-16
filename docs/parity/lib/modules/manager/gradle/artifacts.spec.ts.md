# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gradle/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle/artifacts.spec.ts
**Total tests:** 27 | **Ported:** 0 | **Actionable:** 27 | **Status:** pending

### `isGradleExecutionAllowed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false when allowedUnsafeExecutions is empty (as it not enabled by default option) | 90 | pending | — | — | — |
| returns true when allowedUnsafeExecutions includes `gradleWrapper` | 101 | pending | — | — | — |
| returns false when allowedUnsafeExecutions does not include `gradleWrapper` | 112 | pending | — | — | — |
| logs when allowedUnsafeExecutions does not include `gradleWrapper` | 123 | pending | — | — | — |

### `lockfile tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| aborts if no lockfile is found | 138 | pending | — | — | — |
| aborts if lock file exists but no gradle wrapper | 157 | pending | — | — | — |
| aborts if allowedUnsafeExecutions does not include `gradleWrapper` | 176 | pending | — | — | — |
| uses custom JVM heap settings when toolSettings are configured | 202 | pending | — | — | — |
| updates lock file | 247 | pending | — | — | — |
| updates lock file in win32 | 288 | pending | — | — | — |
| prefers packageName over depName if provided | 333 | pending | — | — | — |
| aborts lock file maintenance if packageFileName is not build.gradle(.kts) in root project | 378 | pending | — | — | — |
| performs lock file maintenance | 393 | pending | — | — | — |
| performs lock file maintenance (docker) | 431 | pending | — | — | — |
| performs lock file maintenance (install) | 495 | pending | — | — | — |
| updates all included projects | 534 | pending | — | — | — |
| does not update lockfile if content is unchanged | 578 | pending | — | — | — |
| gradlew failed | 592 | pending | — | — | — |
| rethrows temporary error | 621 | pending | — | — | — |
| fallback to default Java version if Gradle version not extractable | 640 | pending | — | — | — |

### `dependency verification tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates verification metadata file | 684 | pending | — | — | — |
| aborts verification metadata updates if allowedUnsafeExecutions does not include `gradleWrapper` | 731 | pending | — | — | — |
| updates existing checksums also if verify-checksums is disabled | 765 | pending | — | — | — |
| updates verification metadata and lock file | 820 | pending | — | — | — |
| uses sha256 as default if only weak hash algorithms are found | 894 | pending | — | — | — |
| uses pgp hashType if verify-signatures is enabled | 939 | pending | — | — | — |
| does not write verification metadata, when no checksums exist and neither checksum nor signature verification is enabled | 983 | pending | — | — | — |

---

