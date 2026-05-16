# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bazel/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel/extract.spec.ts
**Total tests:** 12 | **Ported:** 12 | **Actionable:** 12 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty if fails to parse | 10 | ported | `bazel.rs` | `empty_file_returns_empty` (+ invalid_content_returns_empty, git_repository_without_url_returns_empty) | — |
| returns empty if cannot parse dependency | 15 | ported | `bazel.rs` | `invalid_content_returns_empty` | — |
| returns empty for incomplete dependency | 20 | ported | `bazel.rs` | `http_archive_with_no_url_returns_dep_with_skip_reason` | — |
| extracts multiple types of dependencies | 25 | ported | `bazel.rs` | `workspace1_multiple_dependency_types` | — |
| extracts github tags | 31 | ported | `bazel.rs` | `extracts_github_archive_dep` (+ extracts_github_release_dep, extracts_multiple_archives) | — |
| handle comments and strings | 42 | ported | `bazel.rs` | `workspace3_comments_and_strings` | — |
| extracts dependencies from *.bzl files | 47 | ported | `bazel.rs` | `extracts_dependencies_from_bzl_files` | — |
| extracts dependencies for container_pull deptype | 65 | ported | `bazel.rs` | `container_pull_extracted` | — |
| extracts dependencies for oci_pull deptype | 90 | ported | `bazel.rs` | `oci_pull_extracted` | — |
| check remote option in go_repository | 113 | ported | `bazel.rs` | `go_repository_remote_option` | — |
| sequential http_archive | 166 | ported | `bazel.rs` | `singular_url_form_extracted` | — |
| http_archive with GitLab url | 190 | ported | `bazel.rs` | `gitlab_archive_with_version_extracted` (+ gitlab_archive_with_commit_digest_extracted) | — |

---

