# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/swift/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/swift/extract.spec.ts
**Total tests:** 21 | **Ported:** 21 | **Actionable:** 0 | **Status:** done

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty content | 7 | ported | `spm.rs` | `returns_null_for_empty_content` | — |
| returns null for content without dependencies | 11 | ported | `spm.rs` | `returns_null_for_content_without_dependencies` | — |
| extracts GitHub dependencies with github-tags datasource | 31 | ported | `spm.rs` | `extracts_github_dependencies_with_github_tags_datasource` | — |
| extracts GitLab dependencies with gitlab-tags datasource | 52 | ported | `spm.rs` | `extracts_gitlab_dependencies_with_gitlab_tags_datasource` | — |
| extracts self-hosted GitHub dependencies with registryUrls | 73 | ported | `spm.rs` | `extracts_self_hosted_github_with_registry_urls` | — |
| extracts self-hosted GitLab dependencies with registryUrls | 95 | ported | `spm.rs` | `extracts_self_hosted_gitlab_with_registry_urls` | — |
| extracts other dependencies with git-tags datasource | 117 | ported | `spm.rs` | `extracts_other_dependencies_with_git_tags_datasource` | — |
| extracts exact version dependencies | 138 | ported | `spm.rs` | `extracts_exact_version_dependencies` | — |
| extracts exact version with label syntax | 159 | ported | `spm.rs` | `extracts_exact_version_with_label_syntax` | — |
| extracts range version dependencies | 180 | ported | `spm.rs` | `extracts_range_version_dependencies` | — |
| extracts dependencies from sample package file | 201 | ported | `spm.rs` | `extracts_dependencies_from_sample_package_file` | — |
| handles malformed URLs gracefully | 236 | ported | `spm.rs` | `handles_malformed_urls_gracefully` | — |
| handles dependencies without version | 249 | ported | `spm.rs` | `handles_dependencies_without_version` | — |
| handles dependencies with local package | 262 | ported | `spm.rs` | `handles_dependencies_with_local_package` | — |
| handles dependencies with name (deprecated args) | 275 | ported | `spm.rs` | `handles_dependencies_with_name_deprecated_args` | — |
| extracts multiple dependencies with different datasources | 290 | ported | `spm.rs` | `extracts_multiple_dependencies_with_different_datasources` | — |
| extracts multiple dependencies with traits arguments | 308 | ported | `spm.rs` | `extracts_multiple_dependencies_with_traits_arguments` | — |
| extracts GitHub dependencies from SCP-style SSH URL | 116 | ported | `spm.rs` | `extracts_github_dependencies_from_scp_style_ssh_url` | — |
| extracts GitLab dependencies from SCP-style SSH URL | 137 | ported | `spm.rs` | `extracts_gitlab_dependencies_from_scp_style_ssh_url` | — |
| extracts dependencies from ssh:// URL | 158 | ported | `spm.rs` | `extracts_dependencies_from_ssh_url` | — |
| returns null for unparseable SSH URL | 179 | ported | `spm.rs` | `returns_null_for_unparseable_ssh_url` | — |

---

