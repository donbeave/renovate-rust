# `lib/modules/datasource/maven/index.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**41/46 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 123 | returns null when metadata is not found | ported | [`crates/renovate-core/src/datasources/maven.rs:1917`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1917) |
| 136 | when using primary registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2413`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2413) |
| 145 | when using mirror url | ported | [`crates/renovate-core/src/datasources/maven.rs:2427`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2427) |
| 156 | when using primary registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2413`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2413) |
| 165 | when using mirror url | ported | [`crates/renovate-core/src/datasources/maven.rs:2427`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2427) |
| 176 | fetches gradle plugins from non-maven-central registries | ported | [`crates/renovate-core/src/datasources/maven.rs:2469`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2469) |
| 190 | returns releases | ported | [`crates/renovate-core/src/datasources/maven.rs:1888`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1888) |
| 198 | returns releases when only snapshot | ported | [`crates/renovate-core/src/datasources/maven.rs:2064`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2064) |
| 229 | handles invalid snapshot | ported | [`crates/renovate-core/src/datasources/maven.rs:2103`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2103) |
| 265 | returns releases from custom repository | ported | [`crates/renovate-core/src/datasources/maven.rs:2136`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2136) |
| 273 | falls back to next registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:1974`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1974) |
| 304 | merges releases from multiple registries | ported | [`crates/renovate-core/src/datasources/maven.rs:2012`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2012) |
| 325 | throws external_host_error for 50x | ported | [`crates/renovate-core/src/datasources/maven.rs:3298`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3298) |
| 334 | ignores unsupported protocols | ported | [`crates/renovate-core/src/datasources/maven.rs:1934`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1934) |
| 347 | skips registry with invalid metadata structure | ported | [`crates/renovate-core/src/datasources/maven.rs:2162`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2162) |
| 363 | skips registry with invalid xml | ported | [`crates/renovate-core/src/datasources/maven.rs:1948`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1948) |
| 379 | handles optional slash at the end of registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2182`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2182) |
| 389 | returns null for invalid registryurls | ported | [`crates/renovate-core/src/datasources/maven.rs:2241`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2241) |
| 398 | supports scm.url values prefixed with "scm:" | ported | [`crates/renovate-core/src/datasources/maven.rs:2357`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2357) |
| 408 | with only groupid present | ported | [`crates/renovate-core/src/datasources/maven.rs:2249`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2249) |
| 428 | with only artifactid present | ported | [`crates/renovate-core/src/datasources/maven.rs:2260`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2260) |
| 448 | with all elments present | ported | [`crates/renovate-core/src/datasources/maven.rs:2331`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2331) |
| 473 | removes authentication header after redirect | pending | — |
| 513 | supports artifactregistry urls with auth | pending | — |
| 574 | supports artifactregistry urls without auth | pending | — |
| 635 | should get source and homepage from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2508`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2508) |
| 651 | should deal with missing parent fields | ported | [`crates/renovate-core/src/datasources/maven.rs:2577`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2577) |
| 669 | should deal with circular hierarchy | ported | [`crates/renovate-core/src/datasources/maven.rs:2619`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2619) |
| 704 | should get source from own pom and homepage from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2686`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2686) |
| 720 | should get homepage from own pom and source from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2752`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2752) |
| 736 | should get homepage and source from own pom | ported | [`crates/renovate-core/src/datasources/maven.rs:2271`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2271) |
| 751 | should be able to detect git@github.com:child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2366`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2366) |
| 765 | should be able to detect git@github.com/child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2288`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2288) |
| 779 | should be able to detect git://@github.com/child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2297`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2297) |
| 795 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/maven.rs:2208`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2208) |
| 806 | returns original value for unknown error | ported | [`crates/renovate-core/src/datasources/maven.rs:2833`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2833) |
| 821 | returns original value for 200 response | ported | [`crates/renovate-core/src/datasources/maven.rs:2849`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2849) |
| 833 | returns original value for 200 response with versionorig | ported | [`crates/renovate-core/src/datasources/maven.rs:2906`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2906) |
| 845 | returns original value for invalid configs | ported | [`crates/renovate-core/src/datasources/maven.rs:2867`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2867) |
| 861 | adds releasetimestamp | ported | [`crates/renovate-core/src/datasources/maven.rs:2881`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2881) |
| 892 | checks package | ported | [`crates/renovate-core/src/datasources/maven.rs:3202`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3202) |
| 910 | supports timestamp | ported | [`crates/renovate-core/src/datasources/maven.rs:3220`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3220) |
| 934 | returns null for deleted object | ported | [`crates/renovate-core/src/datasources/maven.rs:3238`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3238) |
| 952 | returns null for notfound response | ported | [`crates/renovate-core/src/datasources/maven.rs:3256`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3256) |
| 970 | returns null for nosuchkey response | ported | [`crates/renovate-core/src/datasources/maven.rs:3270`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3270) |
| 988 | returns original value for any other error | ported | [`crates/renovate-core/src/datasources/maven.rs:3284`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3284) |

