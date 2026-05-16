# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/docker/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/docker/index.spec.ts
**Total tests:** 85 | **Ported:** 3 | **Actionable:** 3 | **Status:** ported

### `modules/datasource/docker/index › getDigest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if errored | 59 | not-applicable | — | — | Renovate's Docker registry manifest digest lookup is not implemented in Rust; Rust Docker support currently targets Docker Hub tag pages and update summaries. |
| returns null if empty header | 75 | not-applicable | — | — | Renovate's Docker manifest HEAD digest-header handling is not implemented in Rust. |
| returns digest | 89 | not-applicable | — | — | Renovate's Docker manifest digest lookup with Bearer auth negotiation is not implemented in Rust. |
| falls back to body for digest | 117 | not-applicable | — | — | Renovate's Docker manifest-body digest fallback is not implemented in Rust. |
| supports docker insecure registry | 169 | not-applicable | — | — | Renovate's Docker insecure registry digest lookup is not implemented in Rust. |
| supports basic authentication | 184 | not-applicable | — | — | Renovate's Docker Basic-auth digest lookup is not implemented in Rust. |
| returns null for 403 with basic authentication | 205 | not-applicable | — | — | Renovate's Docker Basic-auth 403 digest handling is not implemented in Rust. |
| passes credentials to ECR client for host $host | 221 | not-applicable | — | — | Renovate's ECR credential provider integration is not implemented in Rust. |
| passes configured awsRegion to ECR client for host $host | 261 | not-applicable | — | — | Renovate's ECR region selection is not implemented in Rust. |
| passes configured registryRegion to ECR client for host $host | 308 | not-applicable | — | — | Renovate's ECR registryRegion selection is not implemented in Rust. |
| passes configured awsAccessKeyID and awsSecretAccessKey to ECR client for host $host | 338 | not-applicable | — | — | Renovate's ECR explicit AWS credential mapping is not implemented in Rust. |
| support no hostRules for host $host | 357 | not-applicable | — | — | Renovate's ECR no-hostRules auth flow is not implemented in Rust. |
| continues without token if ECR auth fails for host $host | 376 | not-applicable | — | — | Renovate's ECR auth failure fallback is not implemented in Rust. |
| supports Google ADC authentication for gcr | 407 | not-applicable | — | — | Renovate's Google ADC registry authentication is not implemented in Rust. |
| supports Google ADC authentication for gar | 442 | not-applicable | — | — | Renovate's Google Artifact Registry ADC authentication is not implemented in Rust. |
| supports basic authentication for gcr | 478 | not-applicable | — | — | Renovate's GCR Basic-auth digest lookup is not implemented in Rust. |
| supports basic authentication for gar | 512 | not-applicable | — | — | Renovate's GAR Basic-auth digest lookup is not implemented in Rust. |
| supports public gcr | 547 | not-applicable | — | — | Renovate's public GCR digest lookup is not implemented in Rust. |
| supports public gar | 569 | not-applicable | — | — | Renovate's public GAR digest lookup is not implemented in Rust. |
| continues without token if Google ADC fails for gcr | 590 | not-applicable | — | — | Renovate's GCR ADC failure fallback is not implemented in Rust. |
| continues without token if Google ADC fails for gar | 614 | not-applicable | — | — | Renovate's GAR ADC failure fallback is not implemented in Rust. |
| continues without token, when no header is present | 639 | not-applicable | — | — | Renovate's Docker auth challenge fallback for digest lookup is not implemented in Rust. |
| supports token with no service | 655 | not-applicable | — | — | Renovate's Docker Bearer token exchange without service is not implemented in Rust. |
| supports scoped names | 676 | not-applicable | — | — | Renovate's Docker scoped repository digest lookup is not implemented in Rust. |
| should throw error for 429 | 699 | not-applicable | — | — | Renovate's Docker digest ExternalHostError policy is not implemented in Rust. |
| should throw error for 5xx | 709 | not-applicable | — | — | Renovate's Docker digest ExternalHostError policy is not implemented in Rust. |
| supports architecture-specific digest | 719 | not-applicable | — | — | Renovate's manifest-list architecture-specific digest resolution is not implemented in Rust. |
| supports architecture-specific digest whithout manifest list | 817 | not-applicable | — | — | Renovate's image-config architecture digest resolution is not implemented in Rust. |
| handles missing architecture-specific digest | 894 | not-applicable | — | — | Renovate's architecture-specific digest fallback is not implemented in Rust. |
| treats empty string architecture as no architecture | 993 | not-applicable | — | — | Renovate's architecture option handling for digest lookup is not implemented in Rust. |
| supports architecture-specific digest in OCI manifests with media type | 1059 | not-applicable | — | — | Renovate's OCI manifest architecture-specific digest resolution is not implemented in Rust. |
| supports architecture-specific digest in OCI manifests without media type | 1138 | not-applicable | — | — | Renovate's OCI manifest architecture-specific digest resolution is not implemented in Rust. |
| handles error while retrieving manifest list for architecture-specific digest | 1209 | not-applicable | — | — | Renovate's manifest-list fetch error handling for digest lookup is not implemented in Rust. |
| handles error while retrieving image config blob | 1293 | not-applicable | — | — | Renovate's image-config blob error handling for digest lookup is not implemented in Rust. |
| returns null if digest refers to manifest list and new value invalid | 1346 | not-applicable | — | — | Renovate's digest-reference validation against manifest lists is not implemented in Rust. |
| falls back to library/ prefix on non-namespaced images with existing digest | 1380 | not-applicable | — | — | Renovate's digest lookup fallback to `library/` on custom registries is not implemented in Rust. |
| uses Docker Hub tag cache digest without HEAD request | 1422 | not-applicable | — | — | Renovate's Docker Hub tag digest cache is not implemented in Rust. |
| uses Docker Hub tag cache arch digest when currentDigest is arch-specific | 1438 | not-applicable | — | — | Renovate's Docker Hub architecture digest cache is not implemented in Rust. |
| falls back to library/ prefix on non-namespaced images without existing digest | 1493 | not-applicable | — | — | Renovate's digest lookup fallback to `library/` on custom registries is not implemented in Rust. |

### `modules/datasource/docker/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no token | 1526 | not-applicable | — | — | Renovate's Docker registry v2 tag-list auth fallback is not implemented in Rust. |
| uses custom registry with registryUrls | 1542 | not-applicable | — | — | Renovate's arbitrary Docker registryUrl resolver and v2 tag-list pagination are not implemented in Rust. |
| uses custom max pages | 1573 | not-applicable | — | — | Renovate's configurable Docker tag-list page limit is not implemented in Rust; Rust fetches at most two Docker Hub REST pages. |
| uses custom registry in packageName | 1605 | not-applicable | — | — | Renovate's custom registry extraction from packageName is not implemented in Rust; non-Docker-Hub registries are rejected. |
| uses quay api | 1624 | not-applicable | — | — | Renovate's Quay v1 tag API integration is not implemented in Rust. |
| uses quay api 2 | 1649 | not-applicable | — | — | Renovate's Quay registryUrl path handling is not implemented in Rust. |
| uses quay api and test error | 1674 | not-applicable | — | — | Renovate's Quay ExternalHostError policy is not implemented in Rust. |
| uses quay api with fallback from v1 to v2 on 401 Unauthorized | 1689 | not-applicable | — | — | Renovate's Quay v1-to-v2 fallback is not implemented in Rust. |
| jfrog artifactory - retry tags for official images by injecting `/library` after repository and before image | 1724 | not-applicable | — | — | Renovate's Artifactory Docker tag-list retry and link rewriting are not implemented in Rust. |
| uses lower tag limit for ECR deps for host $host | 1776 | not-applicable | — | — | Renovate's ECR-specific Docker tag-list limit is not implemented in Rust. |
| uses lower tag limit for ECR Public deps for host $host | 1804 | not-applicable | — | — | Renovate's public ECR tag-list limit and auth handling are not implemented in Rust. |
| resolves requests to ECR proxy | 1859 | not-applicable | — | — | Renovate's ECR proxy max-results retry and label lookup are not implemented in Rust. |
| returns null when it receives ECR max results error more than once | 1918 | not-applicable | — | — | Renovate's ECR proxy max-results retry failure path is not implemented in Rust. |
| returns null when the response code is not 405 | 1949 | not-applicable | — | — | Renovate's ECR proxy max-results response classifier is not implemented in Rust. |
| returns null when no response headers are present | 1980 | not-applicable | — | — | Renovate's ECR proxy max-results response classifier is not implemented in Rust. |
| returns null when the expected docker header is missing | 2003 | not-applicable | — | — | Renovate's ECR proxy max-results response classifier is not implemented in Rust. |
| returns null when the response body does not contain an errors object | 2032 | not-applicable | — | — | Renovate's ECR proxy max-results response classifier is not implemented in Rust. |
| returns null when the response body does not contain errors | 2053 | not-applicable | — | — | Renovate's ECR proxy max-results response classifier is not implemented in Rust. |
| returns null when the the response errors does not have a message property | 2076 | not-applicable | — | — | Renovate's ECR proxy max-results response classifier is not implemented in Rust. |
| returns null when the the error message does not have the expected max results error | 2103 | not-applicable | — | — | Renovate's ECR proxy max-results response classifier is not implemented in Rust. |
| Uses Docker Hub tags for registry-1.docker.io | 2132 | ported | `docker_hub.rs` | `fetch_tags_returns_tag_names` | Rust verifies Docker Hub REST tag-page fetching and tag-name extraction. |
| Uses custom page limit for Docker hub repository tags | 2178 | not-applicable | — | — | Renovate's configurable Docker Hub page limit is not implemented in Rust; Rust fetches at most two pages. |
| adds library/ prefix for Docker Hub (implicit) | 2228 | ported | `docker_hub.rs` | `official_image_maps_to_library` | Rust verifies official Docker Hub images resolve to the `library` namespace. |
| adds library/ prefix for Docker Hub (explicit) | 2256 | not-applicable | — | — | Renovate's Docker Hub host alias normalization from `docker.io/node` is not implemented in Rust. |
| sets releaseTimestamp on digests from Docker Hub | 2302 | not-applicable | — | — | Renovate's Docker Hub release timestamp and digest metadata mapping are not implemented in Rust. |
| adds no library/ prefix for other registries | 2353 | not-applicable | — | — | Renovate's non-Docker-Hub registry release lookup is not implemented in Rust; Rust rejects non-Docker-Hub registries. |
| returns null on error | 2379 | not-applicable | — | — | Renovate's null-on-registry-error release-list contract differs from Rust, which returns an error for failed Docker Hub REST pages. |
| strips trailing slash from registry | 2394 | ported | `docker_hub.rs` | `fetch_tags_trims_trailing_api_base_slash` | Rust verifies the supplied Docker Hub API base is normalized before tag-page requests. |
| returns null if no auth | 2421 | not-applicable | — | — | Renovate's Docker Basic-auth tag-list fallback is not implemented in Rust. |
| supports labels | 2437 | not-applicable | — | — | Renovate's Docker manifest label extraction and metadata mapping are not implemented in Rust. |
| supports labels - handle missing config prop on blob response | 2512 | not-applicable | — | — | Renovate's Docker label blob fallback behavior is not implemented in Rust. |
| supports manifest lists | 2559 | not-applicable | — | — | Renovate's Docker manifest-list traversal for release metadata is not implemented in Rust. |
| ignores empty manifest lists | 2612 | not-applicable | — | — | Renovate's empty manifest-list handling for release metadata is not implemented in Rust. |
| ignores unsupported manifest | 2639 | not-applicable | — | — | Renovate's unsupported manifest handling for release metadata is not implemented in Rust. |
| ignores unsupported schema version | 2664 | not-applicable | — | — | Renovate's unsupported schema-version handling for release metadata is not implemented in Rust. |
| supports OCI manifests with media type | 2686 | not-applicable | — | — | Renovate's OCI manifest metadata extraction is not implemented in Rust. |
| supports OCI manifests without media type | 2742 | not-applicable | — | — | Renovate's OCI manifest metadata extraction is not implemented in Rust. |
| ignores empty OCI manifest indexes | 2797 | not-applicable | — | — | Renovate's empty OCI manifest-index handling is not implemented in Rust. |
| supports redirect | 2823 | not-applicable | — | — | Renovate's Docker blob redirect handling for labels is not implemented in Rust. |
| supports ghcr | 2878 | not-applicable | — | — | Renovate's GHCR registry auth, tag-list, and label lookup are not implemented in Rust. |

### `modules/datasource/docker/index › getLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses annotations for oci image | 2943 | not-applicable | — | — | Renovate's Docker/OCI label and annotation lookup helper is not implemented in Rust. |
| uses annotations for oci helm | 2974 | not-applicable | — | — | Renovate's OCI Helm annotation lookup is not implemented in Rust. |
| uses sources for oci helm | 3005 | not-applicable | — | — | Renovate's OCI Helm config source extraction is not implemented in Rust. |
| uses annotations for docker hub | 3035 | not-applicable | — | — | Renovate's Docker Hub annotation lookup is not implemented in Rust. |
| skips docker hub labels | 3071 | not-applicable | — | — | Renovate's Docker Hub label lookup disable flag is not implemented in Rust. |
| does not skip non docker hub registry labels | 3085 | not-applicable | — | — | Renovate's non-Docker-Hub label lookup under the Docker Hub disable flag is not implemented in Rust. |

---

