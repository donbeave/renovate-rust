# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/extract/file-match.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/extract/file-match.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `workers/repository/extract/file-match › getIncludedFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns fileList if no includePaths | 8 | ported | `managers.rs` | `get_included_files_returns_all_when_no_include_paths` | — |
| returns exact matches | 13 | ported | `managers.rs` | `get_included_files_exact_match` | — |
| returns minimatch matches | 20 | ported | `managers.rs` | `get_included_files_glob_match` | — |

### `workers/repository/extract/file-match › filterIgnoredFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns fileList if no ignoredPaths | 29 | ported | `managers.rs` | `filter_ignored_files_returns_all_when_no_ignore_paths` | — |
| ignores partial matches | 34 | ported | `managers.rs` | `filter_ignored_files_ignores_substring_matches` | — |
| returns minimatch matches | 41 | ported | `managers.rs` | `filter_ignored_files_glob_match` | — |

### `workers/repository/extract/file-match › getMatchingFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns npm files | 57 | ported | `managers.rs` | `get_matching_files_npm_pattern` | — |
| deduplicates | 64 | ported | `managers.rs` | `get_matching_files_deduplicates` | — |

---

