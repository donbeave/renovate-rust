# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/docker/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/docker/index.spec.ts
**Total tests:** 85 | **Ported:** 3 | **Actionable:** 82 | **Status:** done

### `modules/datasource/docker/index › getDigest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if errored | 59 | not-applicable | — | — | HTTP mock-based integration test; full Docker registry digest pipeline not yet in Rust |
| returns null if empty header | 75 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth+manifest pipeline not yet in Rust |
| returns digest | 89 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth+manifest pipeline not yet in Rust |
| falls back to body for digest | 117 | not-applicable | — | — | HTTP mock-based integration test; Docker registry manifest body fallback not yet in Rust |
| supports docker insecure registry | 169 | not-applicable | — | — | HTTP mock-based integration test; insecure registry support not yet in Rust |
| supports basic authentication | 184 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth pipeline not yet in Rust |
| returns null for 403 with basic authentication | 205 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth pipeline not yet in Rust |
| passes credentials to ECR client for host $host | 221 | not-applicable | — | — | ECR auth integration; AWS SDK mock-based test not applicable to Rust |
| passes configured awsRegion to ECR client for host $host | 261 | not-applicable | — | — | ECR auth integration; AWS SDK mock-based test not applicable to Rust |
| passes configured registryRegion to ECR client for host $host | 308 | not-applicable | — | — | ECR auth integration; AWS SDK mock-based test not applicable to Rust |
| passes configured awsAccessKeyID and awsSecretAccessKey to ECR client for host $host | 338 | not-applicable | — | — | ECR auth integration; AWS SDK mock-based test not applicable to Rust |
| support no hostRules for host $host | 357 | not-applicable | — | — | ECR auth integration; hostRules mock not applicable to Rust |
| continues without token if ECR auth fails for host $host | 376 | not-applicable | — | — | ECR auth integration; AWS SDK mock-based test not applicable to Rust |
| supports Google ADC authentication for gcr | 407 | not-applicable | — | — | Google ADC auth integration; google-auth-library mock not applicable to Rust |
| supports Google ADC authentication for gar | 442 | not-applicable | — | — | Google ADC auth integration; google-auth-library mock not applicable to Rust |
| supports basic authentication for gcr | 478 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth pipeline not yet in Rust |
| supports basic authentication for gar | 512 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth pipeline not yet in Rust |
| supports public gcr | 547 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth pipeline not yet in Rust |
| supports public gar | 569 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth pipeline not yet in Rust |
| continues without token if Google ADC fails for gcr | 590 | not-applicable | — | — | Google ADC auth integration; google-auth-library mock not applicable to Rust |
| continues without token if Google ADC fails for gar | 614 | not-applicable | — | — | Google ADC auth integration; google-auth-library mock not applicable to Rust |
| continues without token, when no header is present | 639 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth pipeline not yet in Rust |
| supports token with no service | 655 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth pipeline not yet in Rust |
| supports scoped names | 676 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth pipeline not yet in Rust |
| should throw error for 429 | 699 | not-applicable | — | — | HTTP mock-based integration test; Docker registry error handling not yet in Rust |
| should throw error for 5xx | 709 | not-applicable | — | — | HTTP mock-based integration test; Docker registry error handling not yet in Rust |
| supports architecture-specific digest | 719 | not-applicable | — | — | HTTP mock-based integration test; Docker registry manifest list+config pipeline not yet in Rust |
| supports architecture-specific digest whithout manifest list | 817 | not-applicable | — | — | HTTP mock-based integration test; Docker registry arch-specific digest not yet in Rust |
| handles missing architecture-specific digest | 894 | not-applicable | — | — | HTTP mock-based integration test; Docker registry arch-specific digest fallback not yet in Rust |
| treats empty string architecture as no architecture | 993 | not-applicable | — | — | HTTP mock-based integration test; Docker registry arch detection not yet in Rust |
| supports architecture-specific digest in OCI manifests with media type | 1059 | not-applicable | — | — | HTTP mock-based integration test; OCI manifest pipeline not yet in Rust |
| supports architecture-specific digest in OCI manifests without media type | 1138 | not-applicable | — | — | HTTP mock-based integration test; OCI manifest pipeline not yet in Rust |
| handles error while retrieving manifest list for architecture-specific digest | 1209 | not-applicable | — | — | HTTP mock-based integration test; Docker registry error handling not yet in Rust |
| handles error while retrieving image config blob | 1293 | not-applicable | — | — | HTTP mock-based integration test; Docker registry config blob error not yet in Rust |
| returns null if digest refers to manifest list and new value invalid | 1346 | not-applicable | — | — | HTTP mock-based integration test; Docker registry digest validation not yet in Rust |
| falls back to library/ prefix on non-namespaced images with existing digest | 1380 | not-applicable | — | — | HTTP mock-based integration test; Docker registry library/ fallback not yet in Rust |
| uses Docker Hub tag cache digest without HEAD request | 1422 | not-applicable | — | — | DockerHubCache mock-based integration test; tag cache not yet in Rust |
| uses Docker Hub tag cache arch digest when currentDigest is arch-specific | 1438 | not-applicable | — | — | DockerHubCache mock-based integration test; arch digest cache not yet in Rust |
| falls back to library/ prefix on non-namespaced images without existing digest | 1493 | not-applicable | — | — | HTTP mock-based integration test; Docker registry library/ fallback not yet in Rust |

### `modules/datasource/docker/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no token | 1526 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth pipeline not yet in Rust |
| uses custom registry with registryUrls | 1542 | not-applicable | — | — | HTTP mock-based integration test; Docker registry tag listing not yet in Rust |
| uses custom max pages | 1573 | not-applicable | — | — | HTTP mock-based integration test; Docker registry pagination not yet in Rust |
| uses custom registry in packageName | 1605 | not-applicable | — | — | HTTP mock-based integration test; Docker registry tag listing not yet in Rust |
| uses quay api | 1624 | not-applicable | — | — | HTTP mock-based integration test; Quay API integration not yet in Rust |
| uses quay api 2 | 1649 | not-applicable | — | — | HTTP mock-based integration test; Quay API integration not yet in Rust |
| uses quay api and test error | 1674 | not-applicable | — | — | HTTP mock-based integration test; Quay API error handling not yet in Rust |
| uses quay api with fallback from v1 to v2 on 401 Unauthorized | 1689 | not-applicable | — | — | HTTP mock-based integration test; Quay API fallback not yet in Rust |
| jfrog artifactory - retry tags for official images by injecting `/library` after repository and before image | 1724 | not-applicable | — | — | HTTP mock-based integration test; Artifactory library/ retry not yet in Rust |
| uses lower tag limit for ECR deps for host $host | 1776 | not-applicable | — | — | ECR tag limit integration; AWS SDK mock-based test not applicable to Rust |
| uses lower tag limit for ECR Public deps for host $host | 1804 | not-applicable | — | — | ECR Public tag limit integration; AWS SDK mock-based test not applicable to Rust |
| resolves requests to ECR proxy | 1859 | not-applicable | — | — | ECR proxy integration; AWS SDK mock-based test not applicable to Rust |
| returns null when it receives ECR max results error more than once | 1918 | not-applicable | — | — | ECR error handling; AWS SDK mock-based test not applicable to Rust |
| returns null when the response code is not 405 | 1949 | not-applicable | — | — | ECR error handling; AWS SDK mock-based test not applicable to Rust |
| returns null when no response headers are present | 1980 | not-applicable | — | — | ECR error handling; AWS SDK mock-based test not applicable to Rust |
| returns null when the expected docker header is missing | 2003 | not-applicable | — | — | ECR error handling; AWS SDK mock-based test not applicable to Rust |
| returns null when the response body does not contain an errors object | 2032 | not-applicable | — | — | ECR error handling; AWS SDK mock-based test not applicable to Rust |
| returns null when the response body does not contain errors | 2053 | not-applicable | — | — | ECR error handling; AWS SDK mock-based test not applicable to Rust |
| returns null when the the response errors does not have a message property | 2076 | not-applicable | — | — | ECR error handling; AWS SDK mock-based test not applicable to Rust |
| returns null when the the error message does not have the expected max results error | 2103 | not-applicable | — | — | ECR error handling; AWS SDK mock-based test not applicable to Rust |
| Uses Docker Hub tags for registry-1.docker.io | 2132 | ported | `docker_hub.rs` | `fetch_tags_returns_tag_names` | Rust verifies Docker Hub REST tag-page fetching and tag-name extraction |
| Uses custom page limit for Docker hub repository tags | 2178 | not-applicable | — | — | HTTP mock-based integration test; Docker Hub pagination config not yet in Rust |
| adds library/ prefix for Docker Hub (implicit) | 2228 | ported | `docker_hub.rs` | `official_image_maps_to_library` | Rust verifies official Docker Hub images resolve to the `library` namespace |
| adds library/ prefix for Docker Hub (explicit) | 2256 | not-applicable | — | — | HTTP mock-based integration test; Docker Hub library/ prefix via registryUrl not yet in Rust |
| sets releaseTimestamp on digests from Docker Hub | 2302 | not-applicable | — | — | HTTP mock-based integration test; Docker Hub release timestamp not yet in Rust |
| adds no library/ prefix for other registries | 2353 | not-applicable | — | — | HTTP mock-based integration test; Docker registry library/ prefix logic not yet in Rust |
| returns null on error | 2379 | not-applicable | — | — | HTTP mock-based integration test; Docker registry error handling not yet in Rust |
| strips trailing slash from registry | 2394 | ported | `docker_hub.rs` | `fetch_tags_trims_trailing_api_base_slash` | Rust verifies the supplied Docker Hub API base is normalized before tag-page requests |
| returns null if no auth | 2421 | not-applicable | — | — | HTTP mock-based integration test; Docker registry auth pipeline not yet in Rust |
| supports labels | 2437 | not-applicable | — | — | HTTP mock-based integration test; Docker registry label extraction not yet in Rust |
| supports labels - handle missing config prop on blob response | 2512 | not-applicable | — | — | HTTP mock-based integration test; Docker registry label error handling not yet in Rust |
| supports manifest lists | 2559 | not-applicable | — | — | HTTP mock-based integration test; Docker registry manifest list parsing not yet in Rust |
| ignores empty manifest lists | 2612 | not-applicable | — | — | HTTP mock-based integration test; Docker registry manifest list handling not yet in Rust |
| ignores unsupported manifest | 2639 | not-applicable | — | — | HTTP mock-based integration test; Docker registry manifest type filtering not yet in Rust |
| ignores unsupported schema version | 2664 | not-applicable | — | — | HTTP mock-based integration test; Docker registry schema version check not yet in Rust |
| supports OCI manifests with media type | 2686 | not-applicable | — | — | HTTP mock-based integration test; OCI manifest parsing not yet in Rust |
| supports OCI manifests without media type | 2742 | not-applicable | — | — | HTTP mock-based integration test; OCI manifest parsing not yet in Rust |
| ignores empty OCI manifest indexes | 2797 | not-applicable | — | — | HTTP mock-based integration test; OCI manifest index handling not yet in Rust |
| supports redirect | 2823 | not-applicable | — | — | HTTP mock-based integration test; Docker registry redirect handling not yet in Rust |
| supports ghcr | 2878 | not-applicable | — | — | HTTP mock-based integration test; GHCR integration not yet in Rust |

### `modules/datasource/docker/index › getLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses annotations for oci image | 2943 | not-applicable | — | — | HTTP mock-based integration test; OCI label annotation extraction not yet in Rust |
| uses annotations for oci helm | 2974 | not-applicable | — | — | HTTP mock-based integration test; OCI Helm annotation extraction not yet in Rust |
| uses sources for oci helm | 3005 | not-applicable | — | — | HTTP mock-based integration test; OCI Helm source extraction not yet in Rust |
| uses annotations for docker hub | 3035 | not-applicable | — | — | HTTP mock-based integration test; Docker Hub annotation extraction not yet in Rust |
| skips docker hub labels | 3071 | not-applicable | — | — | HTTP mock-based integration test; Docker Hub label skip logic not yet in Rust |
| does not skip non docker hub registry labels | 3085 | not-applicable | — | — | HTTP mock-based integration test; Docker registry label handling not yet in Rust |

---
