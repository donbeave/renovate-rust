# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gradle-wrapper/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle-wrapper/artifacts.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Custom Gradle Wrapper heap settings are populated | 77 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| replaces existing value | 119 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| aborts if allowedUnsafeExecutions does not include `toolSettings` | 167 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| gradlew not found | 200 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| gradlew failed | 220 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates distributionSha256Sum (docker) | 243 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates distributionSha256Sum (install) | 301 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| distributionSha256Sum 404 | 342 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| handles gradle-wrapper in subdirectory | 368 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

### `updateBuildFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates wrapper configuration in gradle build file | 418 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| gradle build file update skips missing distributionSha256Sum property | 448 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| gradle build file update returns early if file not found | 476 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

### `updateLockFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns early if build script file not found | 495 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| includes gradle lockfile in result | 506 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---

