# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/maven/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/maven/index.spec.ts
**Total tests:** 46 | **Ported:** 0 | **Actionable:** 46 | **Status:** done

### `modules/datasource/maven/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when metadata is not found | 123 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns releases | 134 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns releases when only snapshot | 142 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles invalid snapshot | 173 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns releases from custom repository | 209 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| falls back to next registry url | 217 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| throws EXTERNAL_HOST_ERROR for 50x | 248 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| ignores unsupported protocols | 257 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| skips registry with invalid metadata structure | 270 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| skips registry with invalid XML | 286 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles optional slash at the end of registry url | 302 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for invalid registryUrls | 312 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports scm.url values prefixed with "scm:" | 321 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| with only groupId present | 331 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| with only artifactId present | 351 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| with all elments present | 371 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| removes authentication header after redirect | 396 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports artifactregistry urls with auth | 436 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports artifactregistry urls without auth | 497 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should get source and homepage from parent | 558 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should deal with missing parent fields | 574 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should deal with circular hierarchy | 592 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should get source from own pom and homepage from parent | 627 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should get homepage from own pom and source from parent | 643 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should get homepage and source from own pom | 659 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should be able to detect git@github.com:child-scm as valid sourceUrl | 674 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should be able to detect git@github.com/child-scm as valid sourceUrl | 688 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should be able to detect git://@github.com/child-scm as valid sourceUrl | 702 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for 404 | 718 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns original value for unknown error | 729 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns original value for 200 response | 744 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns original value for 200 response with versionOrig | 756 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns original value for invalid configs | 768 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| adds releaseTimestamp | 784 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| checks package | 815 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports timestamp | 833 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null for deleted object | 857 | not-applicable | — | — | Requires httpMock + AWS SDK mock infrastructure |
| returns null for NotFound response | 875 | not-applicable | — | — | Requires httpMock + AWS SDK mock infrastructure |
| returns null for NoSuchKey response | 893 | not-applicable | — | — | Requires httpMock + AWS SDK mock infrastructure |
| returns original value for any other error | 911 | not-applicable | — | — | Requires httpMock + AWS SDK mock infrastructure |
| when using primary registry URL | 136 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| when using mirror URL | 145 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| when using primary registry URL | 156 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| when using mirror URL | 165 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| fetches Gradle plugins from non-Maven-Central registries | 176 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| merges releases from multiple registries | 304 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

---
