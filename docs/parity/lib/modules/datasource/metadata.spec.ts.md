# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/metadata.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/metadata.spec.ts
**Total tests:** 32 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/metadata`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Should handle manualChangelogUrls | 19 | not-applicable | — | — | Renovate's datasource metadata enrichment layer and manual metadata tables are not implemented in Rust. |
| Should handle manualSourceUrls | 51 | not-applicable | — | — | Renovate's datasource metadata enrichment layer and manual metadata tables are not implemented in Rust. |
| Should handle parsing of sourceUrls correctly | 82 | not-applicable | — | — | Renovate's sourceUrl/sourceDirectory metadata normalization is not implemented as a shared Rust layer. |
| Should split the sourceDirectory out of sourceUrl for known platforms: $sourceUrl -> ($expectedSourceUrl, $expectedSourceDirectory) | 113 | not-applicable | — | — | Renovate's shared sourceDirectory extraction from known platform URLs is not implemented in Rust. |
| Should fallback to massagedUrl for sourceUrl for non Github non HTTP(S) hosts: $sourceUrl -> $expectedSourceUrl | 134 | not-applicable | — | — | Renovate's shared git URL massaging through hostRules is not implemented in Rust. |
| Should not split a sourceDirectory when one cannot be detected $sourceUrl | 158 | not-applicable | — | — | Renovate's shared sourceDirectory extraction guard is not implemented in Rust. |
| Should not overwrite any existing sourceDirectory | 180 | not-applicable | — | — | Renovate's shared sourceDirectory preservation behavior is not implemented in Rust. |
| Should massage github sourceUrls | 197 | not-applicable | — | — | Renovate's shared GitHub sourceUrl massaging is not implemented in Rust. |
| Should handle parsing of sourceUrls correctly for GitLab also | 228 | not-applicable | — | — | Renovate's shared GitLab sourceUrl/sourceDirectory normalization is not implemented in Rust. |
| Should handle failed parsing of sourceUrls for GitLab | 251 | not-applicable | — | — | Renovate's shared GitLab sourceUrl parse-failure handling is not implemented in Rust. |
| Should handle failed parsing of sourceUrls for other | 274 | not-applicable | — | — | Renovate's shared sourceUrl parse-failure handling is not implemented in Rust. |
| Should handle non-url | 297 | not-applicable | — | — | Renovate's shared non-URL metadata handling is not implemented in Rust. |
| Should handle parsing/converting of GitHub sourceUrls with http and www correctly | 319 | not-applicable | — | — | Renovate's shared GitHub URL canonicalization is not implemented in Rust. |
| Should move github homepage to sourceUrl | 331 | not-applicable | — | — | Renovate's shared homepage-to-sourceUrl metadata promotion is not implemented in Rust. |
| Should handle parsing/converting of GitLab sourceUrls with http and www correctly | 345 | not-applicable | — | — | Renovate's shared GitLab URL canonicalization is not implemented in Rust. |
| Should normalize releaseTimestamp | 357 | not-applicable | — | — | Renovate's shared releaseTimestamp normalization is not implemented in Rust. |
| Should return an empty string when massaging an invalid url | 385 | not-applicable | — | — | Renovate's shared URL massaging helper is not implemented in Rust. |
| massageUrl($url) === $expected | 389 | not-applicable | — | — | Renovate's shared URL massaging helper is not implemented in Rust. |
| massageGithubUrl($url) === $expected | 403 | not-applicable | — | — | Renovate's shared GitHub URL massaging helper is not implemented in Rust. |
| massageGitlabUrl($url) === $expected | 415 | not-applicable | — | — | Renovate's shared GitLab URL massaging helper is not implemented in Rust. |
| Should massage github git@ url to valid https url | 428 | not-applicable | — | — | Renovate's shared GitHub SSH URL canonicalization is not implemented in Rust. |
| Should massage github http url to valid https url | 434 | not-applicable | — | — | Renovate's shared GitHub HTTP URL canonicalization is not implemented in Rust. |
| Should massage github http and git url to valid https url | 440 | not-applicable | — | — | Renovate's shared GitHub git URL canonicalization is not implemented in Rust. |
| Should massage github ssh git@ url to valid https url | 446 | not-applicable | — | — | Renovate's shared GitHub SSH URL canonicalization is not implemented in Rust. |
| Should massage github git url to valid https url | 452 | not-applicable | — | — | Renovate's shared GitHub git URL canonicalization is not implemented in Rust. |
| Should massage gitlab git url to valid https url | 458 | not-applicable | — | — | Renovate's shared GitLab git URL canonicalization is not implemented in Rust. |
| Should remove homepage when homepage and sourceUrl are same | 464 | not-applicable | — | — | Renovate's shared duplicate homepage/sourceUrl cleanup is not implemented in Rust. |
| Should delete gitlab homepage if its same as sourceUrl | 503 | not-applicable | — | — | Renovate's shared duplicate GitLab homepage/sourceUrl cleanup is not implemented in Rust. |
| does not set homepage to sourceURl when undefined | 542 | not-applicable | — | — | Renovate's shared homepage/sourceUrl cleanup guard is not implemented in Rust. |
| does not set homepage to sourceURl when not github or gitlab | 580 | not-applicable | — | — | Renovate's shared homepage/sourceUrl cleanup guard is not implemented in Rust. |
| shouldDeleteHomepage($homepage, $sourceUrl) === $expected | 618 | not-applicable | — | — | Renovate's shared duplicate homepage/sourceUrl predicate is not implemented in Rust. |
| should handle dep with no releases | 638 | not-applicable | — | — | Renovate's shared metadata enrichment for empty release results is not implemented in Rust. |

---

