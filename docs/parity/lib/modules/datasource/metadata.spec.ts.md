# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/metadata.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/metadata.spec.ts
**Total tests:** 32 | **Ported:** 31 | **Actionable:** 32 | **Status:** partial

### `modules/datasource/metadata`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Should handle manualChangelogUrls | 19 | ported | `datasources.rs` | `add_metadata_manual_changelog_url` | —  | — | — | — |
| Should handle manualSourceUrls | 51 | ported | `datasources.rs` | `add_metadata_manual_source_url` | —  | — | — | — |
| Should handle parsing of sourceUrls correctly | 82 | ported | `datasources.rs` | `add_metadata_parses_github_tree_url` | —  | — | — | — |
| Should split the sourceDirectory out of sourceUrl for known platforms: $sourceUrl -> ($expectedSourceUrl, $expectedSourceDirectory) | 113 | ported | `datasources.rs` | `add_metadata_extracts_source_directory` | Ported bitnami/charts case; GitLab and custom host cases not yet ported  | — | — | — |
| Should fallback to massagedUrl for sourceUrl for non Github non HTTP(S) hosts: $sourceUrl -> $expectedSourceUrl | 134 | ported | `datasources.rs` | `add_metadata_fallback_to_massaged_url` | — |
| Should not split a sourceDirectory when one cannot be detected $sourceUrl | 158 | ported | `datasources.rs` | `add_metadata_no_source_directory_for_simple_urls` | Tests bitnami, gitlab simple URLs  | — | — | — |
| Should not overwrite any existing sourceDirectory | 180 | ported | `datasources.rs` | `add_metadata_preserves_existing_source_directory` | —  | — | — | — |
| Should massage github sourceUrls | 197 | ported | `datasources.rs` | `add_metadata_massage_github_pages_url` | — |
| Should handle parsing of sourceUrls correctly for GitLab also | 228 | ported | `datasources.rs` | `add_metadata_gitlab_tree_url` | —  | — | — | — |
| Should handle failed parsing of sourceUrls for GitLab | 251 | ported | `datasources.rs` | `add_metadata_gitlab_invalid_url_unchanged` | — |
| Should handle failed parsing of sourceUrls for other | 274 | ported | `datasources.rs` | `add_metadata_invalid_url_stays` | —  | — | — | — |
| Should handle non-url | 297 | ported | `datasources.rs` | `add_metadata_removes_non_url_source` | —  | — | — | — |
| Should handle parsing/converting of GitHub sourceUrls with http and www correctly | 319 | pending | — | — | — |
| Should move github homepage to sourceUrl | 331 | ported | `datasources.rs` | `add_metadata_github_homepage_to_source_url` | —  | — | — | — |
| Should handle parsing/converting of GitLab sourceUrls with http and www correctly | 345 | ported | `datasources.rs` | `add_metadata_gitlab_http_source_url` | —  | — | — | — |
| Should normalize releaseTimestamp | 357 | ported | `datasources.rs` | `add_metadata_github_tree_no_subdir` | Tests GitHub /tree/master without subdir  | — | — | — |
| Should return an empty string when massaging an invalid url | 385 | ported | `util.rs` | `test_massage_url_invalid` | — |
| massageUrl($url) === $expected | 389 | ported | `util.rs` | `test_massage_url_github` + `test_massage_url_gitlab` + `test_massage_url_other_host` | — |
| massageGithubUrl($url) === $expected | 403 | ported | `util.rs` | `test_massage_github_url_*` | — |
| massageGitlabUrl($url) === $expected | 415 | ported | `util.rs` | `test_massage_gitlab_url_*` | — |
| Should massage github git@ url to valid https url | 428 | ported | `util.rs` | `test_massage_github_url_git_at` | — |
| Should massage github http url to valid https url | 434 | ported | `util.rs` | `test_massage_github_url_http` | — |
| Should massage github http and git url to valid https url | 440 | ported | `util.rs` | `test_massage_github_url_http_git` | — |
| Should massage github ssh git@ url to valid https url | 446 | ported | `util.rs` | `test_massage_github_url_ssh` | — |
| Should massage github git url to valid https url | 452 | ported | `util.rs` | `test_massage_github_url_git` | — |
| Should massage gitlab git url to valid https url | 458 | ported | `util.rs` | `test_massage_gitlab_url_git` | — |
| Should remove homepage when homepage and sourceUrl are same | 464 | ported | `datasources.rs` | `add_metadata_removes_duplicate_homepage` | —  | — | — | — |
| Should delete gitlab homepage if its same as sourceUrl | 503 | ported | `datasources.rs` | `add_metadata_removes_duplicate_gitlab_homepage` | —  | — | — | — |
| does not set homepage to sourceURl when undefined | 542 | ported | `datasources.rs` | `add_metadata_no_homepage_promotion_without_homepage` | —  | — | — | — |
| does not set homepage to sourceURl when not github or gitlab | 580 | ported | `datasources.rs` | `add_metadata_non_github_homepage_not_promoted` | —  | — | — | — |
| shouldDeleteHomepage($homepage, $sourceUrl) === $expected | 618 | ported | `util.rs` | `test_should_delete_homepage` | — |
| should handle dep with no releases | 638 | ported | `datasources.rs` | `add_metadata_no_releases` | — |

---
