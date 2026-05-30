# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/gradle-wrapper/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/gradle-wrapper/artifacts.spec.ts
**Total tests:** 14 | **Ported:** 0 | **Actionable:** 14 | **Status:** pending-applicable

### `updateArtifacts()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Custom Gradle Wrapper heap settings are populated | 77 | pending | — | — | — |
| replaces existing value | 119 | pending | — | — | — |
| aborts if allowedUnsafeExecutions does not include `toolSettings` | 167 | pending | — | — | — |
| gradlew not found | 200 | pending | — | — | — |
| gradlew failed | 220 | pending | — | — | — |
| updates distributionSha256Sum (docker) | 243 | pending | — | — | — |
| updates distributionSha256Sum (install) | 301 | pending | — | — | — |
| distributionSha256Sum 404 | 342 | pending | — | — | — |
| handles gradle-wrapper in subdirectory | 368 | pending | — | — | — |

### `updateBuildFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates wrapper configuration in gradle build file | 418 | pending | — | — | — |
| gradle build file update skips missing distributionSha256Sum property | 448 | pending | — | — | — |
| gradle build file update returns early if file not found | 476 | pending | — | — | — |

### `updateLockFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns early if build script file not found | 495 | pending | — | — | — |
| includes gradle lockfile in result | 506 | pending | — | — | — |

---

