# `lib/modules/datasource/maven/index.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**41/46 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 123 | returns null when metadata is not found | ported | [`crates/renovate-core/src/datasources/maven.rs:1860`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1860) |
| 136 | when using primary registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2356`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2356) |
| 145 | when using mirror url | ported | [`crates/renovate-core/src/datasources/maven.rs:2370`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2370) |
| 156 | when using primary registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2356`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2356) |
| 165 | when using mirror url | ported | [`crates/renovate-core/src/datasources/maven.rs:2370`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2370) |
| 176 | fetches gradle plugins from non-maven-central registries | ported | [`crates/renovate-core/src/datasources/maven.rs:2412`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2412) |
| 190 | returns releases | ported | [`crates/renovate-core/src/datasources/maven.rs:1831`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1831) |
| 198 | returns releases when only snapshot | ported | [`crates/renovate-core/src/datasources/maven.rs:2007`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2007) |
| 229 | handles invalid snapshot | ported | [`crates/renovate-core/src/datasources/maven.rs:2046`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2046) |
| 265 | returns releases from custom repository | ported | [`crates/renovate-core/src/datasources/maven.rs:2079`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2079) |
| 273 | falls back to next registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:1917`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1917) |
| 304 | merges releases from multiple registries | ported | [`crates/renovate-core/src/datasources/maven.rs:1955`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1955) |
| 325 | throws external_host_error for 50x | ported | [`crates/renovate-core/src/datasources/maven.rs:3241`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3241) |
| 334 | ignores unsupported protocols | ported | [`crates/renovate-core/src/datasources/maven.rs:1877`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1877) |
| 347 | skips registry with invalid metadata structure | ported | [`crates/renovate-core/src/datasources/maven.rs:2105`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2105) |
| 363 | skips registry with invalid xml | ported | [`crates/renovate-core/src/datasources/maven.rs:1891`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1891) |
| 379 | handles optional slash at the end of registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2125`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2125) |
| 389 | returns null for invalid registryurls | ported | [`crates/renovate-core/src/datasources/maven.rs:2184`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2184) |
| 398 | supports scm.url values prefixed with "scm:" | ported | [`crates/renovate-core/src/datasources/maven.rs:2300`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2300) |
| 408 | with only groupid present | ported | [`crates/renovate-core/src/datasources/maven.rs:2192`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2192) |
| 428 | with only artifactid present | ported | [`crates/renovate-core/src/datasources/maven.rs:2203`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2203) |
| 448 | with all elments present | ported | [`crates/renovate-core/src/datasources/maven.rs:2274`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2274) |
| 473 | removes authentication header after redirect | pending | — |
| 513 | supports artifactregistry urls with auth | pending | — |
| 574 | supports artifactregistry urls without auth | pending | — |
| 635 | should get source and homepage from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2451`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2451) |
| 651 | should deal with missing parent fields | ported | [`crates/renovate-core/src/datasources/maven.rs:2520`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2520) |
| 669 | should deal with circular hierarchy | ported | [`crates/renovate-core/src/datasources/maven.rs:2562`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2562) |
| 704 | should get source from own pom and homepage from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2629`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2629) |
| 720 | should get homepage from own pom and source from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2695`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2695) |
| 736 | should get homepage and source from own pom | ported | [`crates/renovate-core/src/datasources/maven.rs:2214`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2214) |
| 751 | should be able to detect git@github.com:child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2309`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2309) |
| 765 | should be able to detect git@github.com/child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2231`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2231) |
| 779 | should be able to detect git://@github.com/child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2240`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2240) |
| 795 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/maven.rs:2151`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2151) |
| 806 | returns original value for unknown error | ported | [`crates/renovate-core/src/datasources/maven.rs:2776`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2776) |
| 821 | returns original value for 200 response | ported | [`crates/renovate-core/src/datasources/maven.rs:2792`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2792) |
| 833 | returns original value for 200 response with versionorig | ported | [`crates/renovate-core/src/datasources/maven.rs:2849`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2849) |
| 845 | returns original value for invalid configs | ported | [`crates/renovate-core/src/datasources/maven.rs:2810`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2810) |
| 861 | adds releasetimestamp | ported | [`crates/renovate-core/src/datasources/maven.rs:2824`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2824) |
| 892 | checks package | ported | [`crates/renovate-core/src/datasources/maven.rs:3145`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3145) |
| 910 | supports timestamp | ported | [`crates/renovate-core/src/datasources/maven.rs:3163`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3163) |
| 934 | returns null for deleted object | ported | [`crates/renovate-core/src/datasources/maven.rs:3181`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3181) |
| 952 | returns null for notfound response | ported | [`crates/renovate-core/src/datasources/maven.rs:3199`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3199) |
| 970 | returns null for nosuchkey response | ported | [`crates/renovate-core/src/datasources/maven.rs:3213`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3213) |
| 988 | returns original value for any other error | ported | [`crates/renovate-core/src/datasources/maven.rs:3227`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3227) |

