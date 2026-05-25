# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/metadata.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/metadata.spec.ts
**Total tests:** 32 | **Ported:** 0 | **Actionable:** 32 | **Status:** pending

### `modules/datasource/metadata`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Should handle manualChangelogUrls | 19 | pending | — | — | — |
| Should handle manualSourceUrls | 51 | pending | — | — | — |
| Should handle parsing of sourceUrls correctly | 82 | pending | — | — | — |
| Should split the sourceDirectory out of sourceUrl for known platforms: $sourceUrl -> ($expectedSourceUrl, $expectedSourceDirectory) | 113 | pending | — | — | — |
| Should fallback to massagedUrl for sourceUrl for non Github non HTTP(S) hosts: $sourceUrl -> $expectedSourceUrl | 134 | pending | — | — | — |
| Should not split a sourceDirectory when one cannot be detected $sourceUrl | 158 | pending | — | — | — |
| Should not overwrite any existing sourceDirectory | 180 | pending | — | — | — |
| Should massage github sourceUrls | 197 | pending | — | — | — |
| Should handle parsing of sourceUrls correctly for GitLab also | 228 | pending | — | — | — |
| Should handle failed parsing of sourceUrls for GitLab | 251 | pending | — | — | — |
| Should handle failed parsing of sourceUrls for other | 274 | pending | — | — | — |
| Should handle non-url | 297 | pending | — | — | — |
| Should handle parsing/converting of GitHub sourceUrls with http and www correctly | 319 | pending | — | — | — |
| Should move github homepage to sourceUrl | 331 | pending | — | — | — |
| Should handle parsing/converting of GitLab sourceUrls with http and www correctly | 345 | pending | — | — | — |
| Should normalize releaseTimestamp | 357 | pending | — | — | — |
| Should return an empty string when massaging an invalid url | 385 | pending | — | — | — |
| massageUrl($url) === $expected | 389 | pending | — | — | — |
| massageGithubUrl($url) === $expected | 403 | pending | — | — | — |
| massageGitlabUrl($url) === $expected | 415 | pending | — | — | — |
| Should massage github git@ url to valid https url | 428 | pending | — | — | — |
| Should massage github http url to valid https url | 434 | pending | — | — | — |
| Should massage github http and git url to valid https url | 440 | pending | — | — | — |
| Should massage github ssh git@ url to valid https url | 446 | pending | — | — | — |
| Should massage github git url to valid https url | 452 | pending | — | — | — |
| Should massage gitlab git url to valid https url | 458 | pending | — | — | — |
| Should remove homepage when homepage and sourceUrl are same | 464 | pending | — | — | — |
| Should delete gitlab homepage if its same as sourceUrl | 503 | pending | — | — | — |
| does not set homepage to sourceURl when undefined | 542 | pending | — | — | — |
| does not set homepage to sourceURl when not github or gitlab | 580 | pending | — | — | — |
| shouldDeleteHomepage($homepage, $sourceUrl) === $expected | 618 | pending | — | — | — |
| should handle dep with no releases | 638 | pending | — | — | — |

---

