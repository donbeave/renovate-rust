# `lib/modules/datasource/maven/index.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**42/44 in-scope tests ported** (2 pending, 2 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 123 | returns null when metadata is not found | ported | [`crates/renovate-core/src/datasources/maven.rs:1996`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1996) |
| 136 | when using primary registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2492`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2492) |
| 145 | when using mirror url | ported | [`crates/renovate-core/src/datasources/maven.rs:2506`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2506) |
| 156 | when using primary registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2492`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2492) |
| 165 | when using mirror url | ported | [`crates/renovate-core/src/datasources/maven.rs:2506`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2506) |
| 176 | fetches gradle plugins from non-maven-central registries | ported | [`crates/renovate-core/src/datasources/maven.rs:2548`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2548) |
| 190 | returns releases | ported | [`crates/renovate-core/src/datasources/maven.rs:1967`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1967) |
| 198 | returns releases when only snapshot | ported | [`crates/renovate-core/src/datasources/maven.rs:2143`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2143) |
| 229 | handles invalid snapshot | ported | [`crates/renovate-core/src/datasources/maven.rs:2182`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2182) |
| 265 | returns releases from custom repository | ported | [`crates/renovate-core/src/datasources/maven.rs:2215`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2215) |
| 273 | falls back to next registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2053`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2053) |
| 304 | merges releases from multiple registries | ported | [`crates/renovate-core/src/datasources/maven.rs:2091`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2091) |
| 325 | throws external_host_error for 50x | ported | [`crates/renovate-core/src/datasources/maven.rs:3377`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3377) |
| 334 | ignores unsupported protocols | ported | [`crates/renovate-core/src/datasources/maven.rs:2013`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2013) |
| 347 | skips registry with invalid metadata structure | ported | [`crates/renovate-core/src/datasources/maven.rs:2241`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2241) |
| 363 | skips registry with invalid xml | ported | [`crates/renovate-core/src/datasources/maven.rs:2027`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2027) |
| 379 | handles optional slash at the end of registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2261`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2261) |
| 389 | returns null for invalid registryurls | ported | [`crates/renovate-core/src/datasources/maven.rs:2320`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2320) |
| 398 | supports scm.url values prefixed with "scm:" | ported | [`crates/renovate-core/src/datasources/maven.rs:2436`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2436) |
| 408 | with only groupid present | ported | [`crates/renovate-core/src/datasources/maven.rs:2328`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2328) |
| 428 | with only artifactid present | ported | [`crates/renovate-core/src/datasources/maven.rs:2339`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2339) |
| 448 | with all elments present | ported | [`crates/renovate-core/src/datasources/maven.rs:2410`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2410) |
| 473 | removes authentication header after redirect | ported | [`crates/renovate-core/src/datasources/maven.rs:1715`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1715) |
| 513 | supports artifactregistry urls with auth | opt-out | tests special Google Artifact Registry (artifactregistry) URL support and oauth2accesstoken basic auth (the token is obtained via google-auth-library / gcloud; the header is the specific base64 for 'oauth2accesstoken:some-token'); the Rust maven datasource implements generic hostRule basic auth + redirect stripping (covered by the ported 'removes authentication header after redirect'), but the AR-specific token acquisition and .maven artifactregistry endpoint handling is not yet implemented. |
| 574 | supports artifactregistry urls without auth | opt-out | tests artifactregistry urls without requiring the special AR token (googleAuth mock still involved in the test setup); same reason as the sibling 'supports artifactregistry urls with auth' — AR token machinery not implemented in Rust maven auth layer. |
| 635 | should get source and homepage from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2587`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2587) |
| 651 | should deal with missing parent fields | ported | [`crates/renovate-core/src/datasources/maven.rs:2656`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2656) |
| 669 | should deal with circular hierarchy | ported | [`crates/renovate-core/src/datasources/maven.rs:2698`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2698) |
| 704 | should get source from own pom and homepage from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2765`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2765) |
| 720 | should get homepage from own pom and source from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2831`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2831) |
| 736 | should get homepage and source from own pom | ported | [`crates/renovate-core/src/datasources/maven.rs:2350`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2350) |
| 751 | should be able to detect git@github.com:child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2445`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2445) |
| 765 | should be able to detect git@github.com/child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2367`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2367) |
| 779 | should be able to detect git://@github.com/child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2376`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2376) |
| 795 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/maven.rs:2287`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2287) |
| 806 | returns original value for unknown error | ported | [`crates/renovate-core/src/datasources/maven.rs:2912`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2912) |
| 821 | returns original value for 200 response | ported | [`crates/renovate-core/src/datasources/maven.rs:2928`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2928) |
| 833 | returns original value for 200 response with versionorig | ported | [`crates/renovate-core/src/datasources/maven.rs:2985`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2985) |
| 845 | returns original value for invalid configs | ported | [`crates/renovate-core/src/datasources/maven.rs:2946`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2946) |
| 861 | adds releasetimestamp | ported | [`crates/renovate-core/src/datasources/maven.rs:2960`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2960) |
| 892 | checks package | ported | [`crates/renovate-core/src/datasources/maven.rs:3281`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3281) |
| 910 | supports timestamp | ported | [`crates/renovate-core/src/datasources/maven.rs:3299`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3299) |
| 934 | returns null for deleted object | ported | [`crates/renovate-core/src/datasources/maven.rs:3317`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3317) |
| 952 | returns null for notfound response | ported | [`crates/renovate-core/src/datasources/maven.rs:3335`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3335) |
| 970 | returns null for nosuchkey response | ported | [`crates/renovate-core/src/datasources/maven.rs:3349`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3349) |
| 988 | returns original value for any other error | ported | [`crates/renovate-core/src/datasources/maven.rs:3363`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3363) |

