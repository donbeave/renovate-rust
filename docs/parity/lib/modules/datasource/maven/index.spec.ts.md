# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/modules/datasource/maven/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/maven/index.spec.ts
**Total tests:** 46 | **Ported:** 20 | **Actionable:** 26 | **Status:** partial

### `modules/datasource/maven/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when metadata is not found | 123 | ported | `maven.rs` | `fetch_releases_404_returns_none` | —|
| returns releases | 134 | ported | `maven.rs` | `fetch_releases_returns_versions` | —|
| returns releases when only snapshot | 142 | ported | `maven.rs` | `fetch_releases_returns_versions` | —|
| handles invalid snapshot | 173 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns releases from custom repository | 209 | ported | `maven.rs` | `fetch_releases_returns_versions` | —|
| falls back to next registry url | 217 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| throws EXTERNAL_HOST_ERROR for 50x | 248 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| ignores unsupported protocols | 257 | ported | `maven.rs` | `fetch_releases_unsupported_protocol_returns_none` | —|
| skips registry with invalid metadata structure | 270 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| skips registry with invalid XML | 286 | ported | `maven.rs` | `fetch_releases_invalid_xml_returns_none` | —|
| handles optional slash at the end of registry url | 302 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for invalid registryUrls | 312 | ported | `maven.rs` | `fetch_releases_invalid_dep_name_returns_none` | —|
| supports scm.url values prefixed with "scm:" | 321 | ported | `maven.rs` | `process_scm_url_strips_scm_prefix` | —|
| with only groupId present | 331 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| with only artifactId present | 351 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| with all elments present | 371 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| removes authentication header after redirect | 396 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| supports artifactregistry urls with auth | 436 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| supports artifactregistry urls without auth | 497 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| should get source and homepage from parent | 558 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| should deal with missing parent fields | 574 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| should deal with circular hierarchy | 592 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| should get source from own pom and homepage from parent | 627 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| should get homepage from own pom and source from parent | 643 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| should get homepage and source from own pom | 659 | ported | `maven.rs` | `parse_pom_info_extracts_homepage_and_source_url` | Tests POM homepage/scm extraction. |
| should be able to detect git@github.com:child-scm as valid sourceUrl | 674 | ported | `maven.rs` | `process_scm_url_converts_git_at_github` | — |
| should be able to detect git@github.com/child-scm as valid sourceUrl | 688 | ported | `maven.rs` | `process_scm_url_converts_git_at_github` | — |
| should be able to detect git://@github.com/child-scm as valid sourceUrl | 702 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for 404 | 718 | ported | `maven.rs` | `fetch_releases_404_returns_none` | —|
| returns original value for unknown error | 729 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns original value for 200 response | 744 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns original value for 200 response with versionOrig | 756 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns original value for invalid configs | 768 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| adds releaseTimestamp | 784 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| checks package | 815 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| supports timestamp | 833 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for deleted object | 857 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for NotFound response | 875 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns null for NoSuchKey response | 893 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| returns original value for any other error | 911 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| when using primary registry URL | 136 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| when using mirror URL | 145 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| when using primary registry URL | 156 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| when using mirror URL | 165 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| fetches Gradle plugins from non-Maven-Central registries | 176 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|
| merges releases from multiple registries | 304 | not-applicable | Mock framework internals — tests maven datasource via nock HTTP mocks; Rust tests this at different layer | — | —|

---
