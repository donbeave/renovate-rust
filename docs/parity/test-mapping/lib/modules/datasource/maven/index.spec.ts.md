# `lib/modules/datasource/maven/index.spec.ts`

[← `datasource/maven`](../../../../_by-module/datasource/maven.md) · [all modules](../../../../README.md)

**41/46 ported** (5 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 123 | returns null when metadata is not found | ported | `crates/renovate-core/src/datasources/maven.rs:1860` |
| 136 | when using primary registry url | ported | `crates/renovate-core/src/datasources/maven.rs:2356` |
| 145 | when using mirror url | ported | `crates/renovate-core/src/datasources/maven.rs:2370` |
| 156 | when using primary registry url | ported | `crates/renovate-core/src/datasources/maven.rs:2356` |
| 165 | when using mirror url | ported | `crates/renovate-core/src/datasources/maven.rs:2370` |
| 176 | fetches gradle plugins from non-maven-central registries | ported | `crates/renovate-core/src/datasources/maven.rs:2412` |
| 190 | returns releases | ported | `crates/renovate-core/src/datasources/maven.rs:1831` |
| 198 | returns releases when only snapshot | ported | `crates/renovate-core/src/datasources/maven.rs:2007` |
| 229 | handles invalid snapshot | ported | `crates/renovate-core/src/datasources/maven.rs:2046` |
| 265 | returns releases from custom repository | ported | `crates/renovate-core/src/datasources/maven.rs:2079` |
| 273 | falls back to next registry url | ported | `crates/renovate-core/src/datasources/maven.rs:1917` |
| 304 | merges releases from multiple registries | ported | `crates/renovate-core/src/datasources/maven.rs:1955` |
| 325 | throws external_host_error for 50x | ported | `crates/renovate-core/src/datasources/maven.rs:3241` |
| 334 | ignores unsupported protocols | ported | `crates/renovate-core/src/datasources/maven.rs:1877` |
| 347 | skips registry with invalid metadata structure | ported | `crates/renovate-core/src/datasources/maven.rs:2105` |
| 363 | skips registry with invalid xml | ported | `crates/renovate-core/src/datasources/maven.rs:1891` |
| 379 | handles optional slash at the end of registry url | ported | `crates/renovate-core/src/datasources/maven.rs:2125` |
| 389 | returns null for invalid registryurls | ported | `crates/renovate-core/src/datasources/maven.rs:2184` |
| 398 | supports scm.url values prefixed with "scm:" | ported | `crates/renovate-core/src/datasources/maven.rs:2300` |
| 408 | with only groupid present | ported | `crates/renovate-core/src/datasources/maven.rs:2192` |
| 428 | with only artifactid present | ported | `crates/renovate-core/src/datasources/maven.rs:2203` |
| 448 | with all elments present | ported | `crates/renovate-core/src/datasources/maven.rs:2274` |
| 473 | removes authentication header after redirect | pending | — |
| 513 | supports artifactregistry urls with auth | pending | — |
| 574 | supports artifactregistry urls without auth | pending | — |
| 635 | should get source and homepage from parent | ported | `crates/renovate-core/src/datasources/maven.rs:2451` |
| 651 | should deal with missing parent fields | ported | `crates/renovate-core/src/datasources/maven.rs:2520` |
| 669 | should deal with circular hierarchy | ported | `crates/renovate-core/src/datasources/maven.rs:2562` |
| 704 | should get source from own pom and homepage from parent | ported | `crates/renovate-core/src/datasources/maven.rs:2629` |
| 720 | should get homepage from own pom and source from parent | ported | `crates/renovate-core/src/datasources/maven.rs:2695` |
| 736 | should get homepage and source from own pom | ported | `crates/renovate-core/src/datasources/maven.rs:2214` |
| 751 | should be able to detect git@github.com:child-scm as valid sourceurl | ported | `crates/renovate-core/src/datasources/maven.rs:2309` |
| 765 | should be able to detect git@github.com/child-scm as valid sourceurl | ported | `crates/renovate-core/src/datasources/maven.rs:2231` |
| 779 | should be able to detect git://@github.com/child-scm as valid sourceurl | ported | `crates/renovate-core/src/datasources/maven.rs:2240` |
| 795 | returns null for 404 | ported | `crates/renovate-core/src/datasources/maven.rs:2151` |
| 806 | returns original value for unknown error | ported | `crates/renovate-core/src/datasources/maven.rs:2776` |
| 821 | returns original value for 200 response | ported | `crates/renovate-core/src/datasources/maven.rs:2792` |
| 833 | returns original value for 200 response with versionorig | ported | `crates/renovate-core/src/datasources/maven.rs:2849` |
| 845 | returns original value for invalid configs | ported | `crates/renovate-core/src/datasources/maven.rs:2810` |
| 861 | adds releasetimestamp | ported | `crates/renovate-core/src/datasources/maven.rs:2824` |
| 892 | checks package | ported | `crates/renovate-core/src/datasources/maven.rs:3145` |
| 910 | supports timestamp | ported | `crates/renovate-core/src/datasources/maven.rs:3163` |
| 934 | returns null for deleted object | ported | `crates/renovate-core/src/datasources/maven.rs:3181` |
| 952 | returns null for notfound response | ported | `crates/renovate-core/src/datasources/maven.rs:3199` |
| 970 | returns null for nosuchkey response | ported | `crates/renovate-core/src/datasources/maven.rs:3213` |
| 988 | returns original value for any other error | ported | `crates/renovate-core/src/datasources/maven.rs:3227` |

