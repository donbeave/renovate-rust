# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/mise/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/mise/artifacts.spec.ts
**Total tests:** 23 | **Ported:** 0 | **Actionable:** 8 | **Status:** pending

### `modules/manager/mise/artifacts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if lock file does not exist | 46 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` shell execution (`mise lock`); Rust mise support is extractor-only and has no artifact update API |
| returns null if lock file unchanged after exec | 60 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` shell execution (`mise lock`); Rust mise support is extractor-only and has no artifact update API |
| returns updated lock file on success | 81 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` shell execution (`mise lock`); Rust mise support is extractor-only and has no artifact update API |
| returns artifactError on exec failure with combined output | 112 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` shell execution (`mise lock`); Rust mise support is extractor-only and has no artifact update API |
| rethrows TEMPORARY_ERROR | 138 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` shell execution (`mise lock`); Rust mise support is extractor-only and has no artifact update API |
| runs mise lock for lockFileMaintenance | 153 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` shell execution (`mise lock`); Rust mise support is extractor-only and has no artifact update API |
| runs mise lock <tools> for targeted updates | 173 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` shell execution (`mise lock`); Rust mise support is extractor-only and has no artifact update API |
| injects GITHUB_TOKEN when host rule found | 193 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` shell execution (`mise lock`); Rust mise support is extractor-only and has no artifact update API |
| handles empty updatedDeps with fallback to full lock | 238 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` shell execution (`mise lock`); Rust mise support is extractor-only and has no artifact update API |
| handles environment-specific lock files | 258 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` shell execution (`mise lock`); Rust mise support is extractor-only and has no artifact update API |
| uses --local flag for local config files | 296 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` shell execution (`mise lock`); Rust mise support is extractor-only and has no artifact update API |
| uses --local flag and MISE_ENV for env-specific local config files | 327 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` shell execution (`mise lock`); Rust mise support is extractor-only and has no artifact update API |
| uses --local flag for lock file maintenance on local config | 354 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` shell execution (`mise lock`); Rust mise support is extractor-only and has no artifact update API |
| prevents command injection | 378 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` shell execution (`mise lock`); Rust mise support is extractor-only and has no artifact update API |
| handles subdirectory package files | 400 | not-applicable | — | — | Exercises Renovate `updateArtifacts()` shell execution (`mise lock`); Rust mise support is extractor-only and has no artifact update API |

### `modules/manager/mise/artifacts › updateLockedDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns already-updated when version matches | 441 | pending | — | — | — |
| returns already-updated for tool with backend prefix | 454 | pending | — | — | — |
| returns unsupported when version does not match | 467 | pending | — | — | — |
| returns unsupported when tool not in lock file | 480 | pending | — | — | — |
| returns unsupported when no lock file content | 493 | pending | — | — | — |
| returns unsupported for invalid lock file content | 506 | pending | — | — | — |
| returns unsupported when depName is undefined | 519 | pending | — | — | — |
| returns update-failed in case of errors | 532 | pending | — | — | — |
