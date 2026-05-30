# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/maven/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/maven/index.spec.ts
**Total tests:** 46 | **Ported:** 8 | **Actionable:** 38 | **Status:** pending

### `modules/datasource/maven/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when metadata is not found | 123 | ported | `maven.rs` | `fetch_releases_404_returns_none` | —|
| returns releases | 134 | ported | `maven.rs` | `fetch_releases_returns_versions` | —|
| returns releases when only snapshot | 142 | ported | `maven.rs` | `fetch_releases_returns_versions` | —|
| handles invalid snapshot | 173 | pending | — | — | —|
| returns releases from custom repository | 209 | ported | `maven.rs` | `fetch_releases_returns_versions` | —|
| falls back to next registry url | 217 | pending | — | — | —|
| throws EXTERNAL_HOST_ERROR for 50x | 248 | pending | — | — | —|
| ignores unsupported protocols | 257 | ported | `maven.rs` | `fetch_releases_unsupported_protocol_returns_none` | —|
| skips registry with invalid metadata structure | 270 | pending | — | — | —|
| skips registry with invalid XML | 286 | ported | `maven.rs` | `fetch_releases_invalid_xml_returns_none` | —|
| handles optional slash at the end of registry url | 302 | pending | — | — | —|
| returns null for invalid registryUrls | 312 | ported | `maven.rs` | `fetch_releases_invalid_dep_name_returns_none` | —|
| supports scm.url values prefixed with "scm:" | 321 | pending | — | — | —|
| with only groupId present | 331 | pending | — | — | —|
| with only artifactId present | 351 | pending | — | — | —|
| with all elments present | 371 | pending | — | — | —|
| removes authentication header after redirect | 396 | pending | — | — | —|
| supports artifactregistry urls with auth | 436 | pending | — | — | —|
| supports artifactregistry urls without auth | 497 | pending | — | — | —|
| should get source and homepage from parent | 558 | pending | — | — | —|
| should deal with missing parent fields | 574 | pending | — | — | —|
| should deal with circular hierarchy | 592 | pending | — | — | —|
| should get source from own pom and homepage from parent | 627 | pending | — | — | —|
| should get homepage from own pom and source from parent | 643 | pending | — | — | —|
| should get homepage and source from own pom | 659 | pending | — | — | —|
| should be able to detect git@github.com:child-scm as valid sourceUrl | 674 | pending | — | — | —|
| should be able to detect git@github.com/child-scm as valid sourceUrl | 688 | pending | — | — | —|
| should be able to detect git://@github.com/child-scm as valid sourceUrl | 702 | pending | — | — | —|
| returns null for 404 | 718 | ported | `maven.rs` | `fetch_releases_404_returns_none` | —|
| returns original value for unknown error | 729 | pending | — | — | —|
| returns original value for 200 response | 744 | pending | — | — | —|
| returns original value for 200 response with versionOrig | 756 | pending | — | — | —|
| returns original value for invalid configs | 768 | pending | — | — | —|
| adds releaseTimestamp | 784 | pending | — | — | —|
| checks package | 815 | pending | — | — | —|
| supports timestamp | 833 | pending | — | — | —|
| returns null for deleted object | 857 | pending | — | — | —|
| returns null for NotFound response | 875 | pending | — | — | —|
| returns null for NoSuchKey response | 893 | pending | — | — | —|
| returns original value for any other error | 911 | pending | — | — | —|
| when using primary registry URL | 136 | pending | — | — | —|
| when using mirror URL | 145 | pending | — | — | —|
| when using primary registry URL | 156 | pending | — | — | —|
| when using mirror URL | 165 | pending | — | — | —|
| fetches Gradle plugins from non-Maven-Central registries | 176 | pending | — | — | —|
| merges releases from multiple registries | 304 | pending | — | — | —|

---
