# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/docker/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/docker/index.spec.ts
**Total tests:** 85 | **Ported:** 3 | **Actionable:** 85 | **Status:** done

### `modules/datasource/docker/index › getDigest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if errored | 59 | not-applicable | — | — | Requires httpMock + AWS SDK mock + Google auth mock infrastructure |
| returns null if empty header | 75 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) mock infrastructure |
| returns digest | 89 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| falls back to body for digest | 117 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports docker insecure registry | 169 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) mock infrastructure |
| supports basic authentication | 184 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) mock infrastructure |
| returns null for 403 with basic authentication | 205 | not-applicable | — | — | Requires httpMock + vi.mock(host-rules) mock infrastructure |
| passes credentials to ECR client for host $host | 221 | not-applicable | — | — | Requires AWS SDK mock (ECRClient mockClient) infrastructure |
| passes configured awsRegion to ECR client for host $host | 261 | not-applicable | — | — | Requires AWS SDK mock (ECRClient mockClient) infrastructure |
| passes configured registryRegion to ECR client for host $host | 308 | not-applicable | — | — | Requires AWS SDK mock (ECRClient mockClient) infrastructure |
| passes configured awsAccessKeyID and awsSecretAccessKey to ECR client for host $host | 338 | not-applicable | — | — | Requires AWS SDK mock (ECRClient mockClient) infrastructure |
| support no hostRules for host $host | 357 | not-applicable | — | — | Requires AWS SDK mock infrastructure |
| continues without token if ECR auth fails for host $host | 376 | not-applicable | — | — | Requires AWS SDK mock infrastructure |
| supports Google ADC authentication for gcr | 407 | not-applicable | — | — | Requires Google auth library mock infrastructure |
| supports Google ADC authentication for gar | 442 | not-applicable | — | — | Requires Google auth library mock infrastructure |
| supports basic authentication for gcr | 478 | not-applicable | — | — | Requires httpMock + Google auth library mock infrastructure |
| supports basic authentication for gar | 512 | not-applicable | — | — | Requires httpMock + Google auth library mock infrastructure |
| supports public gcr | 547 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports public gar | 569 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| continues without token if Google ADC fails for gcr | 590 | not-applicable | — | — | Requires Google auth library mock infrastructure |
| continues without token if Google ADC fails for gar | 614 | not-applicable | — | — | Requires Google auth library mock infrastructure |
| continues without token, when no header is present | 639 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports token with no service | 655 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports scoped names | 676 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should throw error for 429 | 699 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| should throw error for 5xx | 709 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports architecture-specific digest | 719 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports architecture-specific digest whithout manifest list | 817 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles missing architecture-specific digest | 894 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| treats empty string architecture as no architecture | 993 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports architecture-specific digest in OCI manifests with media type | 1059 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports architecture-specific digest in OCI manifests without media type | 1138 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles error while retrieving manifest list for architecture-specific digest | 1209 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles error while retrieving image config blob | 1293 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null if digest refers to manifest list and new value invalid | 1346 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| falls back to library/ prefix on non-namespaced images with existing digest | 1380 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses Docker Hub tag cache digest without HEAD request | 1422 | not-applicable | — | — | Requires httpMock + DockerHubCache mock infrastructure |
| uses Docker Hub tag cache arch digest when currentDigest is arch-specific | 1438 | not-applicable | — | — | Requires httpMock + DockerHubCache mock infrastructure |
| falls back to library/ prefix on non-namespaced images without existing digest | 1493 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `modules/datasource/docker/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no token | 1526 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses custom registry with registryUrls | 1542 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses custom max pages | 1573 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses custom registry in packageName | 1605 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses quay api | 1624 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses quay api 2 | 1649 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses quay api and test error | 1674 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses quay api with fallback from v1 to v2 on 401 Unauthorized | 1689 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| jfrog artifactory - retry tags for official images by injecting `/library` after repository and before image | 1724 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses lower tag limit for ECR deps for host $host | 1776 | not-applicable | — | — | Requires AWS SDK mock + httpMock infrastructure |
| uses lower tag limit for ECR Public deps for host $host | 1804 | not-applicable | — | — | Requires AWS SDK mock + httpMock infrastructure |
| resolves requests to ECR proxy | 1859 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null when it receives ECR max results error more than once | 1918 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null when the response code is not 405 | 1949 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null when no response headers are present | 1980 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null when the expected docker header is missing | 2003 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null when the response body does not contain an errors object | 2032 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null when the response body does not contain errors | 2053 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null when the the response errors does not have a message property | 2076 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null when the the error message does not have the expected max results error | 2103 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| Uses Docker Hub tags for registry-1.docker.io | 2132 | ported | `docker_hub.rs` | `fetch_tags_returns_tag_names` | Rust verifies Docker Hub REST tag-page fetching and tag-name extraction. |
| Uses custom page limit for Docker hub repository tags | 2178 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| adds library/ prefix for Docker Hub (implicit) | 2228 | ported | `docker_hub.rs` | `official_image_maps_to_library` | Rust verifies official Docker Hub images resolve to the `library` namespace. |
| adds library/ prefix for Docker Hub (explicit) | 2256 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| sets releaseTimestamp on digests from Docker Hub | 2302 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| adds no library/ prefix for other registries | 2353 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns null on error | 2379 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| strips trailing slash from registry | 2394 | ported | `docker_hub.rs` | `fetch_tags_trims_trailing_api_base_slash` | Rust verifies the supplied Docker Hub API base is normalized before tag-page requests. |
| returns null if no auth | 2421 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports labels | 2437 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports labels - handle missing config prop on blob response | 2512 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports manifest lists | 2559 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| ignores empty manifest lists | 2612 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| ignores unsupported manifest | 2639 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| ignores unsupported schema version | 2664 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports OCI manifests with media type | 2686 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports OCI manifests without media type | 2742 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| ignores empty OCI manifest indexes | 2797 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports redirect | 2823 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| supports ghcr | 2878 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `modules/datasource/docker/index › getLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses annotations for oci image | 2943 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses annotations for oci helm | 2974 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses sources for oci helm | 3005 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| uses annotations for docker hub | 3035 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| skips docker hub labels | 3071 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| does not skip non docker hub registry labels | 3085 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

---
