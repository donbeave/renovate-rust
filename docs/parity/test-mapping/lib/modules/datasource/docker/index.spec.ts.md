# `lib/modules/datasource/docker/index.spec.ts`

[← `datasource/docker`](../../../../_by-module/datasource/docker.md) · [all modules](../../../../README.md)

**3/87 ported** (84 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 59 | returns null if errored | pending | — |
| 75 | returns null if empty header | pending | — |
| 89 | returns digest | pending | — |
| 117 | falls back to body for digest | pending | — |
| 169 | supports docker insecure registry | pending | — |
| 184 | supports basic authentication | pending | — |
| 205 | returns null for 403 with basic authentication | pending | — |
| 221 | does not cache null digest results | pending | — |
| 250 | _(it.each / template — verify manually)_ | ? | — |
| 290 | _(it.each / template — verify manually)_ | ? | — |
| 337 | _(it.each / template — verify manually)_ | ? | — |
| 367 | _(it.each / template — verify manually)_ | ? | — |
| 386 | _(it.each / template — verify manually)_ | ? | — |
| 405 | _(it.each / template — verify manually)_ | ? | — |
| 436 | supports google adc authentication for gcr | pending | — |
| 471 | supports google adc authentication for gar | pending | — |
| 507 | supports basic authentication for gcr | pending | — |
| 541 | supports basic authentication for gar | pending | — |
| 576 | supports public gcr | pending | — |
| 598 | supports public gar | pending | — |
| 619 | continues without token if google adc fails for gcr | pending | — |
| 643 | continues without token if google adc fails for gar | pending | — |
| 668 | continues without token, when no header is present | pending | — |
| 684 | supports token with no service | pending | — |
| 705 | supports scoped names | pending | — |
| 728 | should throw error for 429 | pending | — |
| 738 | should throw error for 5xx | pending | — |
| 748 | supports architecture-specific digest | pending | — |
| 846 | supports architecture-specific digest whithout manifest list | pending | — |
| 923 | handles missing architecture-specific digest | pending | — |
| 1022 | treats empty string architecture as no architecture | pending | — |
| 1088 | supports architecture-specific digest in oci manifests with media type | pending | — |
| 1167 | supports architecture-specific digest in oci manifests without media type | pending | — |
| 1238 | handles error while retrieving manifest list for architecture-specific digest | pending | — |
| 1322 | handles error while retrieving image config blob | pending | — |
| 1375 | returns null if digest refers to manifest list and new value invalid | pending | — |
| 1409 | falls back to library/ prefix on non-namespaced images with existing digest | pending | — |
| 1451 | uses docker hub tag cache digest without head request | pending | — |
| 1467 | uses docker hub tag cache arch digest when currentdigest is arch-specific | pending | — |
| 1522 | falls back to library/ prefix on non-namespaced images without existing digest | pending | — |
| 1555 | does not cache null architecture results | pending | — |
| 1613 | returns null if no token | pending | — |
| 1629 | uses custom registry with registryurls | pending | — |
| 1660 | uses custom max pages | pending | — |
| 1692 | uses custom registry in packagename | pending | — |
| 1711 | uses quay api | pending | — |
| 1736 | uses quay api 2 | pending | — |
| 1761 | uses quay api and test error | pending | — |
| 1776 | uses quay api with fallback from v1 to v2 on 401 unauthorized | pending | — |
| 1811 | jfrog artifactory - retry tags for official images by injecting `/library` after repository and before image | pending | — |
| 1863 | _(it.each / template — verify manually)_ | ? | — |
| 1891 | _(it.each / template — verify manually)_ | ? | — |
| 1946 | resolves requests to ecr proxy | pending | — |
| 2005 | returns null when it receives ecr max results error more than once | pending | — |
| 2036 | returns null when the response code is not 405 | pending | — |
| 2067 | returns null when no response headers are present | pending | — |
| 2090 | returns null when the expected docker header is missing | pending | — |
| 2119 | returns null when the response body does not contain an errors object | pending | — |
| 2140 | returns null when the response body does not contain errors | pending | — |
| 2163 | returns null when the the response errors does not have a message property | pending | — |
| 2190 | returns null when the the error message does not have the expected max results error | pending | — |
| 2219 | uses docker hub tags for registry-1.docker.io | ported | [`crates/renovate-core/src/datasources/docker_hub.rs:559`](../../../../../../../crates/renovate-core/src/datasources/docker_hub.rs#L559) |
| 2265 | uses custom page limit for docker hub repository tags | pending | — |
| 2315 | adds library/ prefix for docker hub (implicit) | ported | [`crates/renovate-core/src/datasources/docker_hub.rs:353`](../../../../../../../crates/renovate-core/src/datasources/docker_hub.rs#L353) |
| 2343 | adds library/ prefix for docker hub (explicit) | pending | — |
| 2389 | sets releasetimestamp on digests from docker hub | pending | — |
| 2440 | adds no library/ prefix for other registries | pending | — |
| 2466 | returns null on error | pending | — |
| 2481 | strips trailing slash from registry | ported | [`crates/renovate-core/src/datasources/docker_hub.rs:579`](../../../../../../../crates/renovate-core/src/datasources/docker_hub.rs#L579) |
| 2508 | returns null if no auth | pending | — |
| 2524 | supports labels | pending | — |
| 2599 | supports labels - handle missing config prop on blob response | pending | — |
| 2646 | supports manifest lists | pending | — |
| 2699 | ignores empty manifest lists | pending | — |
| 2726 | ignores unsupported manifest | pending | — |
| 2751 | ignores unsupported schema version | pending | — |
| 2773 | supports oci manifests with media type | pending | — |
| 2829 | supports oci manifests without media type | pending | — |
| 2884 | ignores empty oci manifest indexes | pending | — |
| 2910 | supports redirect | pending | — |
| 2965 | supports ghcr | pending | — |
| 3030 | uses annotations for oci image | pending | — |
| 3061 | uses annotations for oci helm | pending | — |
| 3092 | uses sources for oci helm | pending | — |
| 3122 | uses annotations for docker hub | pending | — |
| 3158 | skips docker hub labels | pending | — |
| 3172 | does not skip non docker hub registry labels | pending | — |

