# `lib/modules/datasource/maven/index.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**42/44 in-scope tests ported** (2 pending, 2 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 123 | returns null when metadata is not found | ported | [`crates/renovate-core/src/datasources/maven.rs:1990`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1990) |
| 136 | when using primary registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2486`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2486) |
| 145 | when using mirror url | ported | [`crates/renovate-core/src/datasources/maven.rs:2500`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2500) |
| 156 | when using primary registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2486`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2486) |
| 165 | when using mirror url | ported | [`crates/renovate-core/src/datasources/maven.rs:2500`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2500) |
| 176 | fetches gradle plugins from non-maven-central registries | ported | [`crates/renovate-core/src/datasources/maven.rs:2542`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2542) |
| 190 | returns releases | ported | [`crates/renovate-core/src/datasources/maven.rs:1961`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1961) |
| 198 | returns releases when only snapshot | ported | [`crates/renovate-core/src/datasources/maven.rs:2137`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2137) |
| 229 | handles invalid snapshot | ported | [`crates/renovate-core/src/datasources/maven.rs:2176`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2176) |
| 265 | returns releases from custom repository | ported | [`crates/renovate-core/src/datasources/maven.rs:2209`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2209) |
| 273 | falls back to next registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2047`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2047) |
| 304 | merges releases from multiple registries | ported | [`crates/renovate-core/src/datasources/maven.rs:2085`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2085) |
| 325 | throws external_host_error for 50x | ported | [`crates/renovate-core/src/datasources/maven.rs:3371`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3371) |
| 334 | ignores unsupported protocols | ported | [`crates/renovate-core/src/datasources/maven.rs:2007`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2007) |
| 347 | skips registry with invalid metadata structure | ported | [`crates/renovate-core/src/datasources/maven.rs:2235`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2235) |
| 363 | skips registry with invalid xml | ported | [`crates/renovate-core/src/datasources/maven.rs:2021`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2021) |
| 379 | handles optional slash at the end of registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2255`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2255) |
| 389 | returns null for invalid registryurls | ported | [`crates/renovate-core/src/datasources/maven.rs:2314`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2314) |
| 398 | supports scm.url values prefixed with "scm:" | ported | [`crates/renovate-core/src/datasources/maven.rs:2430`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2430) |
| 408 | with only groupid present | ported | [`crates/renovate-core/src/datasources/maven.rs:2322`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2322) |
| 428 | with only artifactid present | ported | [`crates/renovate-core/src/datasources/maven.rs:2333`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2333) |
| 448 | with all elments present | ported | [`crates/renovate-core/src/datasources/maven.rs:2404`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2404) |
| 473 | removes authentication header after redirect | ported | [`crates/renovate-core/src/datasources/maven.rs:1709`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1709) |
| 513 | supports artifactregistry urls with auth | opt-out | tests special Google Artifact Registry (artifactregistry) URL support and oauth2accesstoken basic auth (the token is obtained via google-auth-library / gcloud; the header is the specific base64 for 'oauth2accesstoken:some-token'); the Rust maven datasource implements generic hostRule basic auth + redirect stripping (covered by the ported 'removes authentication header after redirect'), but the AR-specific token acquisition and .maven artifactregistry endpoint handling is not yet implemented. |
| 574 | supports artifactregistry urls without auth | opt-out | tests artifactregistry urls without requiring the special AR token (googleAuth mock still involved in the test setup); same reason as the sibling 'supports artifactregistry urls with auth' — AR token machinery not implemented in Rust maven auth layer. |
| 635 | should get source and homepage from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2581`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2581) |
| 651 | should deal with missing parent fields | ported | [`crates/renovate-core/src/datasources/maven.rs:2650`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2650) |
| 669 | should deal with circular hierarchy | ported | [`crates/renovate-core/src/datasources/maven.rs:2692`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2692) |
| 704 | should get source from own pom and homepage from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2759`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2759) |
| 720 | should get homepage from own pom and source from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2825`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2825) |
| 736 | should get homepage and source from own pom | ported | [`crates/renovate-core/src/datasources/maven.rs:2344`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2344) |
| 751 | should be able to detect git@github.com:child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2439`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2439) |
| 765 | should be able to detect git@github.com/child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2361`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2361) |
| 779 | should be able to detect git://@github.com/child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2370`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2370) |
| 795 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/maven.rs:2281`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2281) |
| 806 | returns original value for unknown error | ported | [`crates/renovate-core/src/datasources/maven.rs:2906`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2906) |
| 821 | returns original value for 200 response | ported | [`crates/renovate-core/src/datasources/maven.rs:2922`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2922) |
| 833 | returns original value for 200 response with versionorig | ported | [`crates/renovate-core/src/datasources/maven.rs:2979`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2979) |
| 845 | returns original value for invalid configs | ported | [`crates/renovate-core/src/datasources/maven.rs:2940`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2940) |
| 861 | adds releasetimestamp | ported | [`crates/renovate-core/src/datasources/maven.rs:2954`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2954) |
| 892 | checks package | ported | [`crates/renovate-core/src/datasources/maven.rs:3275`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3275) |
| 910 | supports timestamp | ported | [`crates/renovate-core/src/datasources/maven.rs:3293`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3293) |
| 934 | returns null for deleted object | ported | [`crates/renovate-core/src/datasources/maven.rs:3311`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3311) |
| 952 | returns null for notfound response | ported | [`crates/renovate-core/src/datasources/maven.rs:3329`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3329) |
| 970 | returns null for nosuchkey response | ported | [`crates/renovate-core/src/datasources/maven.rs:3343`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3343) |
| 988 | returns original value for any other error | ported | [`crates/renovate-core/src/datasources/maven.rs:3357`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3357) |

