# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/xcodegen/extract.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/xcodegen/extract.spec.ts
**Total tests:** 24 | **Ported:** 24 | **Actionable:** 24 | **Status:** ported

### `extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty content | 7 | ported | `xcodegen.rs` | `empty_content_returns_empty` | — |
| returns null for invalid YAML | 11 | ported | `xcodegen.rs` | `invalid_yaml_returns_empty` | — |
| returns null for YAML without packages | 22 | ported | `xcodegen.rs` | `no_packages_returns_empty` | — |
| returns null for empty packages | 36 | ported | `xcodegen.rs` | `empty_packages_section_returns_empty` | — |
| extracts packages from a realistic project.yml | 44 | ported | `xcodegen.rs` | `multiple_packages` | — |
| extracts remote package with url and from | 71 | ported | `xcodegen.rs` | `extracts_github_url_with_from` (+ gitlab_url_detected) | — |
| extracts remote package with github shorthand | 92 | ported | `xcodegen.rs` | `extracts_github_shorthand` | — |
| extracts remote package with majorVersion | 113 | ported | `xcodegen.rs` | `extracts_major_version` | — |
| extracts remote package with minorVersion | 134 | ported | `xcodegen.rs` | `extracts_minor_version` | — |
| extracts remote package with exactVersion | 155 | ported | `xcodegen.rs` | `extracts_exact_version` | — |
| extracts remote package with version | 176 | ported | `xcodegen.rs` | `extracts_version_field` | — |
| skips local packages with path | 197 | ported | `xcodegen.rs` | `local_path_skipped` | — |
| skips packages with branch reference | 214 | ported | `xcodegen.rs` | `branch_only_skipped` | — |
| skips packages with revision reference | 233 | ported | `xcodegen.rs` | `revision_reference_skipped` | — |
| skips packages with minVersion/maxVersion range | 252 | ported | `xcodegen.rs` | `min_max_version_range_skipped` | — |
| uses gitlab-tags datasource for GitLab URLs | 272 | ported | `xcodegen.rs` | `gitlab_url_produces_gitlab_source` | — |
| uses github-tags datasource with registryUrls for self-hosted GHES | 293 | ported | `xcodegen.rs` | `self_hosted_ghes_registry_url` | — |
| uses gitlab-tags datasource with registryUrls for self-hosted GitLab | 314 | ported | `xcodegen.rs` | `self_hosted_gitlab_registry_url` | — |
| uses git-tags datasource for non-GitHub/GitLab URLs | 335 | ported | `xcodegen.rs` | `generic_url_produces_git_source` | — |
| skips packages without url or github | 356 | ported | `xcodegen.rs` | `package_without_url_or_github_skipped` | — |
| skips packages without version specifier | 373 | ported | `xcodegen.rs` | `no_version_specifier_skipped` | — |
| extracts multiple packages correctly | 390 | ported | `xcodegen.rs` | `extracts_multiple_packages_correctly` | — |
| handles github URL with .git suffix | 427 | ported | `xcodegen.rs` | `github_url_with_git_suffix` | — |
| handles numeric version values from YAML parsing | 448 | ported | `xcodegen.rs` | `numeric_version_from_yaml` | — |

---

