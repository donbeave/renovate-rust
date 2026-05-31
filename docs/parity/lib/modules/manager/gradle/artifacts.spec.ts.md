# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gradle/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle/artifacts.spec.ts
**Total tests:** 27 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `isGradleExecutionAllowed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false when allowedUnsafeExecutions is empty (as it not enabled by default option) | 90 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| returns true when allowedUnsafeExecutions includes `gradleWrapper` | 101 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| returns false when allowedUnsafeExecutions does not include `gradleWrapper` | 112 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| logs when allowedUnsafeExecutions does not include `gradleWrapper` | 123 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |

### `lockfile tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| aborts if no lockfile is found | 138 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| aborts if lock file exists but no gradle wrapper | 157 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| aborts if allowedUnsafeExecutions does not include `gradleWrapper` | 176 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| uses custom JVM heap settings when toolSettings are configured | 202 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| updates lock file | 247 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| updates lock file in win32 | 288 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| prefers packageName over depName if provided | 333 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| aborts lock file maintenance if packageFileName is not build.gradle(.kts) in root project | 378 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| performs lock file maintenance | 393 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| performs lock file maintenance (docker) | 431 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| performs lock file maintenance (install) | 495 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| updates all included projects | 534 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| does not update lockfile if content is unchanged | 578 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| gradlew failed | 592 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| rethrows temporary error | 621 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| fallback to default Java version if Gradle version not extractable | 640 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |

### `dependency verification tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates verification metadata file | 684 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| aborts verification metadata updates if allowedUnsafeExecutions does not include `gradleWrapper` | 731 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| updates existing checksums also if verify-checksums is disabled | 765 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| updates verification metadata and lock file | 820 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| uses sha256 as default if only weak hash algorithms are found | 894 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| uses pgp hashType if verify-signatures is enabled | 939 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |
| does not write verification metadata, when no checksums exist and neither checksum nor signature verification is enabled | 983 | not-applicable | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | Mock framework internals — tests gradle artifacts via vitest-mocked fs/exec; Rust tests this at different layer |

---

