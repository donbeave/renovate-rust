# `lib/modules/datasource/maven/index.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**41/46 in-scope tests ported** (5 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 123 | returns null when metadata is not found | ported | [`crates/renovate-core/src/datasources/maven.rs:1951`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1951) |
| 136 | when using primary registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2447`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2447) |
| 145 | when using mirror url | ported | [`crates/renovate-core/src/datasources/maven.rs:2461`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2461) |
| 156 | when using primary registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2447`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2447) |
| 165 | when using mirror url | ported | [`crates/renovate-core/src/datasources/maven.rs:2461`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2461) |
| 176 | fetches gradle plugins from non-maven-central registries | ported | [`crates/renovate-core/src/datasources/maven.rs:2503`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2503) |
| 190 | returns releases | ported | [`crates/renovate-core/src/datasources/maven.rs:1922`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1922) |
| 198 | returns releases when only snapshot | ported | [`crates/renovate-core/src/datasources/maven.rs:2098`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2098) |
| 229 | handles invalid snapshot | ported | [`crates/renovate-core/src/datasources/maven.rs:2137`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2137) |
| 265 | returns releases from custom repository | ported | [`crates/renovate-core/src/datasources/maven.rs:2170`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2170) |
| 273 | falls back to next registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2008`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2008) |
| 304 | merges releases from multiple registries | ported | [`crates/renovate-core/src/datasources/maven.rs:2046`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2046) |
| 325 | throws external_host_error for 50x | ported | [`crates/renovate-core/src/datasources/maven.rs:3332`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3332) |
| 334 | ignores unsupported protocols | ported | [`crates/renovate-core/src/datasources/maven.rs:1968`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1968) |
| 347 | skips registry with invalid metadata structure | ported | [`crates/renovate-core/src/datasources/maven.rs:2196`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2196) |
| 363 | skips registry with invalid xml | ported | [`crates/renovate-core/src/datasources/maven.rs:1982`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1982) |
| 379 | handles optional slash at the end of registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2216`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2216) |
| 389 | returns null for invalid registryurls | ported | [`crates/renovate-core/src/datasources/maven.rs:2275`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2275) |
| 398 | supports scm.url values prefixed with "scm:" | ported | [`crates/renovate-core/src/datasources/maven.rs:2391`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2391) |
| 408 | with only groupid present | ported | [`crates/renovate-core/src/datasources/maven.rs:2283`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2283) |
| 428 | with only artifactid present | ported | [`crates/renovate-core/src/datasources/maven.rs:2294`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2294) |
| 448 | with all elments present | ported | [`crates/renovate-core/src/datasources/maven.rs:2365`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2365) |
| 473 | removes authentication header after redirect | pending | — |
| 513 | supports artifactregistry urls with auth | pending | — |
| 574 | supports artifactregistry urls without auth | pending | — |
| 635 | should get source and homepage from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2542`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2542) |
| 651 | should deal with missing parent fields | ported | [`crates/renovate-core/src/datasources/maven.rs:2611`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2611) |
| 669 | should deal with circular hierarchy | ported | [`crates/renovate-core/src/datasources/maven.rs:2653`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2653) |
| 704 | should get source from own pom and homepage from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2720`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2720) |
| 720 | should get homepage from own pom and source from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2786`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2786) |
| 736 | should get homepage and source from own pom | ported | [`crates/renovate-core/src/datasources/maven.rs:2305`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2305) |
| 751 | should be able to detect git@github.com:child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2400`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2400) |
| 765 | should be able to detect git@github.com/child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2322`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2322) |
| 779 | should be able to detect git://@github.com/child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2331`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2331) |
| 795 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/maven.rs:2242`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2242) |
| 806 | returns original value for unknown error | ported | [`crates/renovate-core/src/datasources/maven.rs:2867`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2867) |
| 821 | returns original value for 200 response | ported | [`crates/renovate-core/src/datasources/maven.rs:2883`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2883) |
| 833 | returns original value for 200 response with versionorig | ported | [`crates/renovate-core/src/datasources/maven.rs:2940`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2940) |
| 845 | returns original value for invalid configs | ported | [`crates/renovate-core/src/datasources/maven.rs:2901`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2901) |
| 861 | adds releasetimestamp | ported | [`crates/renovate-core/src/datasources/maven.rs:2915`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2915) |
| 892 | checks package | ported | [`crates/renovate-core/src/datasources/maven.rs:3236`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3236) |
| 910 | supports timestamp | ported | [`crates/renovate-core/src/datasources/maven.rs:3254`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3254) |
| 934 | returns null for deleted object | ported | [`crates/renovate-core/src/datasources/maven.rs:3272`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3272) |
| 952 | returns null for notfound response | ported | [`crates/renovate-core/src/datasources/maven.rs:3290`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3290) |
| 970 | returns null for nosuchkey response | ported | [`crates/renovate-core/src/datasources/maven.rs:3304`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3304) |
| 988 | returns original value for any other error | ported | [`crates/renovate-core/src/datasources/maven.rs:3318`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3318) |

