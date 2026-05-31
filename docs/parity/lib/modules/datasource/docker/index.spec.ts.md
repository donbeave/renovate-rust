# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/docker/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/docker/index.spec.ts
**Total tests:** 85 | **Ported:** 3 | **Actionable:** 82 | **Status:** pending

### `modules/datasource/docker/index › getDigest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if errored | 59 | pending | — | — | No corresponding Rust source|
| returns null if empty header | 75 | pending | — | — | No corresponding Rust source|
| returns digest | 89 | pending | — | — | No corresponding Rust source|
| falls back to body for digest | 117 | pending | — | — | No corresponding Rust source|
| supports docker insecure registry | 169 | pending | — | — | No corresponding Rust source|
| supports basic authentication | 184 | pending | — | — | No corresponding Rust source|
| returns null for 403 with basic authentication | 205 | pending | — | — | No corresponding Rust source|
| passes credentials to ECR client for host $host | 221 | pending | — | — | No corresponding Rust source|
| passes configured awsRegion to ECR client for host $host | 261 | pending | — | — | No corresponding Rust source|
| passes configured registryRegion to ECR client for host $host | 308 | pending | — | — | No corresponding Rust source|
| passes configured awsAccessKeyID and awsSecretAccessKey to ECR client for host $host | 338 | pending | — | — | No corresponding Rust source|
| support no hostRules for host $host | 357 | pending | — | — | No corresponding Rust source|
| continues without token if ECR auth fails for host $host | 376 | pending | — | — | No corresponding Rust source|
| supports Google ADC authentication for gcr | 407 | pending | — | — | No corresponding Rust source|
| supports Google ADC authentication for gar | 442 | pending | — | — | No corresponding Rust source|
| supports basic authentication for gcr | 478 | pending | — | — | No corresponding Rust source|
| supports basic authentication for gar | 512 | pending | — | — | No corresponding Rust source|
| supports public gcr | 547 | pending | — | — | No corresponding Rust source|
| supports public gar | 569 | pending | — | — | No corresponding Rust source|
| continues without token if Google ADC fails for gcr | 590 | pending | — | — | No corresponding Rust source|
| continues without token if Google ADC fails for gar | 614 | pending | — | — | No corresponding Rust source|
| continues without token, when no header is present | 639 | pending | — | — | No corresponding Rust source|
| supports token with no service | 655 | pending | — | — | No corresponding Rust source|
| supports scoped names | 676 | pending | — | — | No corresponding Rust source|
| should throw error for 429 | 699 | pending | — | — | No corresponding Rust source|
| should throw error for 5xx | 709 | pending | — | — | No corresponding Rust source|
| supports architecture-specific digest | 719 | pending | — | — | No corresponding Rust source|
| supports architecture-specific digest whithout manifest list | 817 | pending | — | — | No corresponding Rust source|
| handles missing architecture-specific digest | 894 | pending | — | — | No corresponding Rust source|
| treats empty string architecture as no architecture | 993 | pending | — | — | No corresponding Rust source|
| supports architecture-specific digest in OCI manifests with media type | 1059 | pending | — | — | No corresponding Rust source|
| supports architecture-specific digest in OCI manifests without media type | 1138 | pending | — | — | No corresponding Rust source|
| handles error while retrieving manifest list for architecture-specific digest | 1209 | pending | — | — | No corresponding Rust source|
| handles error while retrieving image config blob | 1293 | pending | — | — | No corresponding Rust source|
| returns null if digest refers to manifest list and new value invalid | 1346 | pending | — | — | No corresponding Rust source|
| falls back to library/ prefix on non-namespaced images with existing digest | 1380 | pending | — | — | No corresponding Rust source|
| uses Docker Hub tag cache digest without HEAD request | 1422 | pending | — | — | No corresponding Rust source|
| uses Docker Hub tag cache arch digest when currentDigest is arch-specific | 1438 | pending | — | — | No corresponding Rust source|
| falls back to library/ prefix on non-namespaced images without existing digest | 1493 | pending | — | — | No corresponding Rust source|

### `modules/datasource/docker/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no token | 1526 | pending | — | — | No corresponding Rust source|
| uses custom registry with registryUrls | 1542 | pending | — | — | No corresponding Rust source|
| uses custom max pages | 1573 | pending | — | — | No corresponding Rust source|
| uses custom registry in packageName | 1605 | pending | — | — | No corresponding Rust source|
| uses quay api | 1624 | pending | — | — | No corresponding Rust source|
| uses quay api 2 | 1649 | pending | — | — | No corresponding Rust source|
| uses quay api and test error | 1674 | pending | — | — | No corresponding Rust source|
| uses quay api with fallback from v1 to v2 on 401 Unauthorized | 1689 | pending | — | — | No corresponding Rust source|
| jfrog artifactory - retry tags for official images by injecting `/library` after repository and before image | 1724 | pending | — | — | No corresponding Rust source|
| uses lower tag limit for ECR deps for host $host | 1776 | pending | — | — | No corresponding Rust source|
| uses lower tag limit for ECR Public deps for host $host | 1804 | pending | — | — | No corresponding Rust source|
| resolves requests to ECR proxy | 1859 | pending | — | — | No corresponding Rust source|
| returns null when it receives ECR max results error more than once | 1918 | pending | — | — | No corresponding Rust source|
| returns null when the response code is not 405 | 1949 | pending | — | — | No corresponding Rust source|
| returns null when no response headers are present | 1980 | pending | — | — | No corresponding Rust source|
| returns null when the expected docker header is missing | 2003 | pending | — | — | No corresponding Rust source|
| returns null when the response body does not contain an errors object | 2032 | pending | — | — | No corresponding Rust source|
| returns null when the response body does not contain errors | 2053 | pending | — | — | No corresponding Rust source|
| returns null when the the response errors does not have a message property | 2076 | pending | — | — | No corresponding Rust source|
| returns null when the the error message does not have the expected max results error | 2103 | pending | — | — | No corresponding Rust source|
| Uses Docker Hub tags for registry-1.docker.io | 2132 | ported | `docker_hub.rs` | `fetch_tags_returns_tag_names` | Rust verifies Docker Hub REST tag-page fetching and tag-name extraction. |
| Uses custom page limit for Docker hub repository tags | 2178 | pending | — | — | No corresponding Rust source|
| adds library/ prefix for Docker Hub (implicit) | 2228 | ported | `docker_hub.rs` | `official_image_maps_to_library` | Rust verifies official Docker Hub images resolve to the `library` namespace. |
| adds library/ prefix for Docker Hub (explicit) | 2256 | pending | — | — | No corresponding Rust source|
| sets releaseTimestamp on digests from Docker Hub | 2302 | pending | — | — | No corresponding Rust source|
| adds no library/ prefix for other registries | 2353 | pending | — | — | No corresponding Rust source|
| returns null on error | 2379 | pending | — | — | No corresponding Rust source|
| strips trailing slash from registry | 2394 | ported | `docker_hub.rs` | `fetch_tags_trims_trailing_api_base_slash` | Rust verifies the supplied Docker Hub API base is normalized before tag-page requests. |
| returns null if no auth | 2421 | pending | — | — | No corresponding Rust source|
| supports labels | 2437 | pending | — | — | No corresponding Rust source|
| supports labels - handle missing config prop on blob response | 2512 | pending | — | — | No corresponding Rust source|
| supports manifest lists | 2559 | pending | — | — | No corresponding Rust source|
| ignores empty manifest lists | 2612 | pending | — | — | No corresponding Rust source|
| ignores unsupported manifest | 2639 | pending | — | — | No corresponding Rust source|
| ignores unsupported schema version | 2664 | pending | — | — | No corresponding Rust source|
| supports OCI manifests with media type | 2686 | pending | — | — | No corresponding Rust source|
| supports OCI manifests without media type | 2742 | pending | — | — | No corresponding Rust source|
| ignores empty OCI manifest indexes | 2797 | pending | — | — | No corresponding Rust source|
| supports redirect | 2823 | pending | — | — | No corresponding Rust source|
| supports ghcr | 2878 | pending | — | — | No corresponding Rust source|

### `modules/datasource/docker/index › getLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses annotations for oci image | 2943 | pending | — | — | No corresponding Rust source|
| uses annotations for oci helm | 2974 | pending | — | — | No corresponding Rust source|
| uses sources for oci helm | 3005 | pending | — | — | No corresponding Rust source|
| uses annotations for docker hub | 3035 | pending | — | — | No corresponding Rust source|
| skips docker hub labels | 3071 | pending | — | — | No corresponding Rust source|
| does not skip non docker hub registry labels | 3085 | pending | — | — | No corresponding Rust source|

---
