# `lib/modules/datasource/maven/index.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**42/46 in-scope tests ported** (4 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 123 | returns null when metadata is not found | ported | [`crates/renovate-core/src/datasources/maven.rs:1970`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1970) |
| 136 | when using primary registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2466`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2466) |
| 145 | when using mirror url | ported | [`crates/renovate-core/src/datasources/maven.rs:2480`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2480) |
| 156 | when using primary registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2466`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2466) |
| 165 | when using mirror url | ported | [`crates/renovate-core/src/datasources/maven.rs:2480`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2480) |
| 176 | fetches gradle plugins from non-maven-central registries | ported | [`crates/renovate-core/src/datasources/maven.rs:2522`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2522) |
| 190 | returns releases | ported | [`crates/renovate-core/src/datasources/maven.rs:1941`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1941) |
| 198 | returns releases when only snapshot | ported | [`crates/renovate-core/src/datasources/maven.rs:2117`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2117) |
| 229 | handles invalid snapshot | ported | [`crates/renovate-core/src/datasources/maven.rs:2156`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2156) |
| 265 | returns releases from custom repository | ported | [`crates/renovate-core/src/datasources/maven.rs:2189`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2189) |
| 273 | falls back to next registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2027`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2027) |
| 304 | merges releases from multiple registries | ported | [`crates/renovate-core/src/datasources/maven.rs:2065`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2065) |
| 325 | throws external_host_error for 50x | ported | [`crates/renovate-core/src/datasources/maven.rs:3351`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3351) |
| 334 | ignores unsupported protocols | ported | [`crates/renovate-core/src/datasources/maven.rs:1987`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1987) |
| 347 | skips registry with invalid metadata structure | ported | [`crates/renovate-core/src/datasources/maven.rs:2215`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2215) |
| 363 | skips registry with invalid xml | ported | [`crates/renovate-core/src/datasources/maven.rs:2001`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2001) |
| 379 | handles optional slash at the end of registry url | ported | [`crates/renovate-core/src/datasources/maven.rs:2235`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2235) |
| 389 | returns null for invalid registryurls | ported | [`crates/renovate-core/src/datasources/maven.rs:2294`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2294) |
| 398 | supports scm.url values prefixed with "scm:" | ported | [`crates/renovate-core/src/datasources/maven.rs:2410`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2410) |
| 408 | with only groupid present | ported | [`crates/renovate-core/src/datasources/maven.rs:2302`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2302) |
| 428 | with only artifactid present | ported | [`crates/renovate-core/src/datasources/maven.rs:2313`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2313) |
| 448 | with all elments present | ported | [`crates/renovate-core/src/datasources/maven.rs:2384`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2384) |
| 473 | removes authentication header after redirect | ported | [`crates/renovate-core/src/datasources/maven.rs:1689`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L1689) |
| 513 | supports artifactregistry urls with auth | pending | — |
| 574 | supports artifactregistry urls without auth | pending | — |
| 635 | should get source and homepage from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2561`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2561) |
| 651 | should deal with missing parent fields | ported | [`crates/renovate-core/src/datasources/maven.rs:2630`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2630) |
| 669 | should deal with circular hierarchy | ported | [`crates/renovate-core/src/datasources/maven.rs:2672`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2672) |
| 704 | should get source from own pom and homepage from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2739`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2739) |
| 720 | should get homepage from own pom and source from parent | ported | [`crates/renovate-core/src/datasources/maven.rs:2805`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2805) |
| 736 | should get homepage and source from own pom | ported | [`crates/renovate-core/src/datasources/maven.rs:2324`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2324) |
| 751 | should be able to detect git@github.com:child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2419`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2419) |
| 765 | should be able to detect git@github.com/child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2341`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2341) |
| 779 | should be able to detect git://@github.com/child-scm as valid sourceurl | ported | [`crates/renovate-core/src/datasources/maven.rs:2350`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2350) |
| 795 | returns null for 404 | ported | [`crates/renovate-core/src/datasources/maven.rs:2261`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2261) |
| 806 | returns original value for unknown error | ported | [`crates/renovate-core/src/datasources/maven.rs:2886`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2886) |
| 821 | returns original value for 200 response | ported | [`crates/renovate-core/src/datasources/maven.rs:2902`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2902) |
| 833 | returns original value for 200 response with versionorig | ported | [`crates/renovate-core/src/datasources/maven.rs:2959`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2959) |
| 845 | returns original value for invalid configs | ported | [`crates/renovate-core/src/datasources/maven.rs:2920`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2920) |
| 861 | adds releasetimestamp | ported | [`crates/renovate-core/src/datasources/maven.rs:2934`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L2934) |
| 892 | checks package | ported | [`crates/renovate-core/src/datasources/maven.rs:3255`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3255) |
| 910 | supports timestamp | ported | [`crates/renovate-core/src/datasources/maven.rs:3273`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3273) |
| 934 | returns null for deleted object | ported | [`crates/renovate-core/src/datasources/maven.rs:3291`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3291) |
| 952 | returns null for notfound response | ported | [`crates/renovate-core/src/datasources/maven.rs:3309`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3309) |
| 970 | returns null for nosuchkey response | ported | [`crates/renovate-core/src/datasources/maven.rs:3323`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3323) |
| 988 | returns original value for any other error | ported | [`crates/renovate-core/src/datasources/maven.rs:3337`](../../../../../../../crates/renovate-core/src/datasources/maven.rs#L3337) |

