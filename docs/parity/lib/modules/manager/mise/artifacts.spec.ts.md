# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/mise/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/mise/artifacts.spec.ts
**Total tests:** 23 | **Ported:** 8 | **Actionable:** 23 | **Status:** done

### `modules/manager/mise/artifacts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if lock file does not exist | 46 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns null if lock file unchanged after exec | 60 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns updated lock file on success | 81 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| returns artifactError on exec failure with combined output | 112 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| rethrows TEMPORARY_ERROR | 138 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| runs mise lock for lockFileMaintenance | 153 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| runs mise lock <tools> for targeted updates | 173 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| injects GITHUB_TOKEN when host rule found | 193 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| handles empty updatedDeps with fallback to full lock | 238 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| handles environment-specific lock files | 258 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| uses --local flag for local config files | 296 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| uses --local flag and MISE_ENV for env-specific local config files | 327 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| uses --local flag for lock file maintenance on local config | 354 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| prevents command injection | 378 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |
| handles subdirectory package files | 400 | not-applicable | — | — | Uses mockExecAll / exec mocking / platform/fs mocking; shell execution tests not portable to Rust |

### `modules/manager/mise/artifacts › updateLockedDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns already-updated when version matches | 441 | ported | `mise.rs` | `update_locked_already_updated_when_version_matches` | — |
| returns already-updated for tool with backend prefix | 454 | ported | `mise.rs` | `update_locked_already_updated_for_backend_prefix` | — |
| returns unsupported when version does not match | 467 | ported | `mise.rs` | `update_locked_unsupported_when_version_does_not_match` | — |
| returns unsupported when tool not in lock file | 480 | ported | `mise.rs` | `update_locked_unsupported_when_tool_not_in_lock_file` | — |
| returns unsupported when no lock file content | 493 | ported | `mise.rs` | `update_locked_unsupported_when_no_lock_file_content` | — |
| returns unsupported for invalid lock file content | 506 | ported | `mise.rs` | `update_locked_unsupported_for_invalid_lock_file_content` | — |
| returns unsupported when depName is undefined | 519 | ported | `mise.rs` | `update_locked_unsupported_when_dep_name_is_none` | — |
| returns update-failed in case of errors | 532 | ported | `mise.rs` | `update_locked_update_failed_on_panic` | — |
