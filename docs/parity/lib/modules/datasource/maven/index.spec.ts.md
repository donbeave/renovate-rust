# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/maven/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/maven/index.spec.ts
**Total tests:** 46 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/maven/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when metadata is not found | 123 | not-applicable | — | — | Renovate's Maven `getReleases` null/error contract is not implemented in Rust; Rust exposes latest-version lookup only. |
| returns releases | 134 | not-applicable | — | — | Renovate's Maven full release-list response mapping is not implemented in Rust; Rust exposes latest-version lookup only. |
| returns releases when only snapshot | 142 | not-applicable | — | — | Renovate's Maven snapshot release-list mapping is not implemented in Rust. |
| handles invalid snapshot | 173 | not-applicable | — | — | Renovate's Maven snapshot metadata validation is not implemented in Rust. |
| returns releases from custom repository | 209 | not-applicable | — | — | Renovate's Maven multi-registry `getReleases` contract is not implemented in Rust; Rust latest lookup accepts one registry base. |
| falls back to next registry url | 217 | not-applicable | — | — | Renovate's Maven registry fallback behavior is not implemented in Rust. |
| throws EXTERNAL_HOST_ERROR for 50x | 248 | not-applicable | — | — | Renovate's Maven external-host-error contract is not implemented in Rust. |
| ignores unsupported protocols | 257 | not-applicable | — | — | Renovate's Maven unsupported-protocol filtering is not implemented in Rust. |
| skips registry with invalid metadata structure | 270 | not-applicable | — | — | Renovate's Maven invalid-metadata skip contract is not implemented in Rust. |
| skips registry with invalid XML | 286 | not-applicable | — | — | Renovate's Maven invalid-XML skip contract is not implemented in Rust. |
| handles optional slash at the end of registry url | 302 | not-applicable | — | — | Renovate's Maven registry URL normalization contract is not exposed as a Rust API. |
| returns null for invalid registryUrls | 312 | not-applicable | — | — | Renovate's Maven registry URL validation/null contract is not implemented in Rust. |
| supports scm.url values prefixed with "scm:" | 321 | not-applicable | — | — | Renovate's Maven POM SCM URL extraction is not implemented in the Rust datasource. |
| with only groupId present | 331 | not-applicable | — | — | Renovate's Maven POM source/homepage extraction is not implemented in the Rust datasource. |
| with only artifactId present | 351 | not-applicable | — | — | Renovate's Maven POM source/homepage extraction is not implemented in the Rust datasource. |
| with all elments present | 371 | not-applicable | — | — | Renovate's Maven POM source/homepage extraction is not implemented in the Rust datasource. |
| removes authentication header after redirect | 396 | not-applicable | — | — | Renovate's Maven redirect/auth header behavior is not implemented in Rust. |
| supports artifactregistry urls with auth | 436 | not-applicable | — | — | Renovate's Google Artifact Registry Maven auth flow is not implemented in Rust. |
| supports artifactregistry urls without auth | 497 | not-applicable | — | — | Renovate's Google Artifact Registry Maven auth flow is not implemented in Rust. |
| should get source and homepage from parent | 558 | not-applicable | — | — | Renovate's Maven parent POM traversal for source/homepage is not implemented in the Rust datasource. |
| should deal with missing parent fields | 574 | not-applicable | — | — | Renovate's Maven parent POM traversal for source/homepage is not implemented in the Rust datasource. |
| should deal with circular hierarchy | 592 | not-applicable | — | — | Renovate's Maven parent POM traversal and circular hierarchy handling are not implemented in Rust. |
| should get source from own pom and homepage from parent | 627 | not-applicable | — | — | Renovate's Maven POM source/homepage extraction is not implemented in the Rust datasource. |
| should get homepage from own pom and source from parent | 643 | not-applicable | — | — | Renovate's Maven POM source/homepage extraction is not implemented in the Rust datasource. |
| should get homepage and source from own pom | 659 | not-applicable | — | — | Renovate's Maven POM source/homepage extraction is not implemented in the Rust datasource. |
| should be able to detect git@github.com:child-scm as valid sourceUrl | 674 | not-applicable | — | — | Renovate's Maven POM SCM URL extraction and normalization are not implemented in the Rust datasource. |
| should be able to detect git@github.com/child-scm as valid sourceUrl | 688 | not-applicable | — | — | Renovate's Maven POM SCM URL extraction and normalization are not implemented in the Rust datasource. |
| should be able to detect git://@github.com/child-scm as valid sourceUrl | 702 | not-applicable | — | — | Renovate's Maven POM SCM URL extraction and normalization are not implemented in the Rust datasource. |
| returns null for 404 | 718 | not-applicable | — | — | Renovate's Maven `postprocessRelease` null-on-404 contract is not implemented in Rust. |
| returns original value for unknown error | 729 | not-applicable | — | — | Renovate's Maven `postprocessRelease` error fallback contract is not implemented in Rust. |
| returns original value for 200 response | 744 | not-applicable | — | — | Renovate's Maven `postprocessRelease` timestamp hook is not implemented in Rust. |
| returns original value for 200 response with versionOrig | 756 | not-applicable | — | — | Renovate's Maven `postprocessRelease` timestamp hook is not implemented in Rust. |
| returns original value for invalid configs | 768 | not-applicable | — | — | Renovate's Maven `postprocessRelease` invalid-config handling is not implemented in Rust. |
| adds releaseTimestamp | 784 | not-applicable | — | — | Renovate's Maven `postprocessRelease` timestamp hook is not implemented in Rust; Rust timestamp lookup is best-effort inside update summaries. |
| checks package | 815 | not-applicable | — | — | Renovate's Maven `postprocessRelease` timestamp package check is not implemented in Rust. |
| supports timestamp | 833 | not-applicable | — | — | Renovate's Maven S3 timestamp object handling is not implemented in Rust. |
| returns null for deleted object | 857 | not-applicable | — | — | Renovate's Maven S3 timestamp object handling is not implemented in Rust. |
| returns null for NotFound response | 875 | not-applicable | — | — | Renovate's Maven S3 timestamp object handling is not implemented in Rust. |
| returns null for NoSuchKey response | 893 | not-applicable | — | — | Renovate's Maven S3 timestamp object handling is not implemented in Rust. |
| returns original value for any other error | 911 | not-applicable | — | — | Renovate's Maven S3 timestamp object error fallback is not implemented in Rust. |

| when using primary registry URL | 136 | not-applicable | — | — | Renovate's Maven datasource metadata fetching and scm/vcs enrichment are not exposed as a Rust API. |
| when using mirror URL | 145 | not-applicable | — | — | Renovate's Maven datasource metadata fetching and scm/vcs enrichment are not exposed as a Rust API. |
| when using primary registry URL | 156 | not-applicable | — | — | Renovate's Maven datasource metadata fetching and scm/vcs enrichment are not exposed as a Rust API. |
| when using mirror URL | 165 | not-applicable | — | — | Renovate's Maven datasource metadata fetching and scm/vcs enrichment are not exposed as a Rust API. |
| fetches Gradle plugins from non-Maven-Central registries | 176 | not-applicable | — | — | Renovate's Maven datasource metadata fetching and scm/vcs enrichment are not exposed as a Rust API. |
| merges releases from multiple registries | 304 | not-applicable | — | — | Renovate's Maven datasource metadata fetching and scm/vcs enrichment are not exposed as a Rust API. |
---

