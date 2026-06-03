# `lib/modules/manager/maven-wrapper/extract.spec.ts`

[← `manager/maven-wrapper`](../../../../_by-module/manager/maven-wrapper.md) · [all modules](../../../../README.md)

**9/9 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 14 | extracts version for property file with distribution type "bin" in distributionurl | ported | `crates/renovate-core/src/extractors/maven_wrapper.rs:137` |
| 37 | extracts version for property file with only a wrapper url | ported | `crates/renovate-core/src/extractors/maven_wrapper.rs:158` |
| 51 | extracts version for property file with only a wrapper version | ported | `crates/renovate-core/src/extractors/maven_wrapper.rs:176` |
| 64 | extracts wrapper information from wrapperurl in precedence to wrapperversion | ported | `crates/renovate-core/src/extractors/maven_wrapper.rs:187` |
| 80 | extracts maven warapper version from mvnw file | ported | `crates/renovate-core/src/extractors/maven_wrapper.rs:201` |
| 93 | extracts maven warapper version from mvnw file - windows | ported | `crates/renovate-core/src/extractors/maven_wrapper.rs:212` |
| 106 | returns null for invalid wrapper version string in from mvnw file | ported | `crates/renovate-core/src/extractors/maven_wrapper.rs:223` |
| 111 | extracts version for property file with only a maven url | ported | `crates/renovate-core/src/extractors/maven_wrapper.rs:230` |
| 125 | should return null when there is no string matching the maven properties regex | ported | `crates/renovate-core/src/extractors/maven_wrapper.rs:245` |

