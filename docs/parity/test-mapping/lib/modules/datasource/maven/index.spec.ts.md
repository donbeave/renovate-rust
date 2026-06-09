# `lib/modules/datasource/maven/index.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**41/46 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 123 | returns null when metadata is not found | ported | [`crates/renovate-core/src/datasources/maven.rs:1899`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1899) |
| 136 | when using primary registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2395`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2395) |
| 145 | when using mirror url | ported | [`crates/renovate-core/src/datasources/maven.rs:2409`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2409) |
| 156 | when using primary registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2395`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2395) |
| 165 | when using mirror url | ported | [`crates/renovate-core/src/datasources/maven.rs:2409`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2409) |
| 176 | fetches gradle plugins from non-maven-central registries | ported | [`crates/renovate-core/src/datasources/maven.rs:2451`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2451) |
| 190 | returns releases | ported | [`crates/renovate-core/src/datasources/maven.rs:1870`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1870) |
| 198 | returns releases when only snapshot | ported | [`crates/renovate-core/src/datasources/maven.rs:2046`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2046) |
| 229 | handles invalid snapshot | ported | [`crates/renovate-core/src/datasources/maven.rs:2085`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2085) |
| 265 | returns releases from custom repository | ported | [`crates/renovate-core/src/datasources/maven.rs:2118`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2118) |
| 273 | falls back to next registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:1956`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1956) |
| 304 | merges releases from multiple registries | ported | [`crates/renovate-core/src/datasources/maven.rs:1994`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1994) |
| 325 | throws external_host_error for 50x | ported | [`crates/renovate-core/src/datasources/maven.rs:3280`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3280) |
| 334 | ignores unsupported protocols | ported | [`crates/renovate-core/src/datasources/maven.rs:1916`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1916) |
| 347 | skips registry with invalid metadata structure | ported | [`crates/renovate-core/src/datasources/maven.rs:2144`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2144) |
| 363 | skips registry with invalid xml | ported | [`crates/renovate-core/src/datasources/maven.rs:1930`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1930) |
| 379 | handles optional slash at the end of registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2164`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2164) |
| 389 | returns null for invalid registryurls | ported | [`crates/renovate-core/src/datasources/maven.rs:2223`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2223) |
| 398 | supports scm.url values prefixed with "scm:" | ported | [`crates/renovate-core/src/datasources/maven.rs:2339`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2339) |
| 408 | with only groupid present | ported | [`crates/renovate-core/src/datasources/maven.rs:2231`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2231) |
| 428 | with only artifactid present | ported | [`crates/renovate-core/src/datasources/maven.rs:2242`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2242) |
| 448 | with all elments present | ported | [`crates/renovate-core/src/datasources/maven.rs:2313`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2313) |
| 473 | removes authentication header after redirect | pending | — |
| 513 | supports artifactregistry urls with auth | pending | — |
| 574 | supports artifactregistry urls without auth | pending | — |
| 635 | should get source and homepage from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2490`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2490) |
| 651 | should deal with missing parent fields | ported | [`crates/renovate-core/src/datasources/maven.rs:2559`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2559) |
| 669 | should deal with circular hierarchy | ported | [`crates/renovate-core/src/datasources/maven.rs:2601`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2601) |
| 704 | should get source from own pom and homepage from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2668`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2668) |
| 720 | should get homepage from own pom and source from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2734`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2734) |
| 736 | should get homepage and source from own pom | ported | [`crates/renovate-core/src/datasources/maven.rs:2253`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2253) |
| 751 | should be able to detect git@github.com:child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2348`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2348) |
| 765 | should be able to detect git@github.com/child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2270`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2270) |
| 779 | should be able to detect git://@github.com/child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2279`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2279) |
| 795 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/maven.rs:2190`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2190) |
| 806 | returns original value for unknown error | ported | [`crates/renovate-core/src/datasources/maven.rs:2815`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2815) |
| 821 | returns original value for 200 response | ported | [`crates/renovate-core/src/datasources/maven.rs:2831`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2831) |
| 833 | returns original value for 200 response with versionorig | ported | [`crates/renovate-core/src/datasources/maven.rs:2888`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2888) |
| 845 | returns original value for invalid configs | ported | [`crates/renovate-core/src/datasources/maven.rs:2849`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2849) |
| 861 | adds releasetimestamp | ported | [`crates/renovate-core/src/datasources/maven.rs:2863`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2863) |
| 892 | checks package | ported | [`crates/renovate-core/src/datasources/maven.rs:3184`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3184) |
| 910 | supports timestamp | ported | [`crates/renovate-core/src/datasources/maven.rs:3202`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3202) |
| 934 | returns null for deleted object | ported | [`crates/renovate-core/src/datasources/maven.rs:3220`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3220) |
| 952 | returns null for notfound response | ported | [`crates/renovate-core/src/datasources/maven.rs:3238`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3238) |
| 970 | returns null for nosuchkey response | ported | [`crates/renovate-core/src/datasources/maven.rs:3252`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3252) |
| 988 | returns original value for any other error | ported | [`crates/renovate-core/src/datasources/maven.rs:3266`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3266) |

