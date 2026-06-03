# `lib/modules/manager/maven/extract.spec.ts`

[← `manager/maven`](../../../../_by-module/manager/maven.md) · [all modules](../../../../README.md)

**29/30 ported** (1 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 22 | returns null for invalid xml | ported | [`crates/renovate-core/src/extractors/maven.rs:1751`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L1751) |
| 29 | extract dependencies from any xml position | ported | [`crates/renovate-core/src/extractors/maven.rs:1630`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L1630) |
| 237 | extract dependencies with windows line endings | ported | [`crates/renovate-core/src/extractors/maven.rs:2494`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2494) |
| 249 | tries minimum manifests | ported | [`crates/renovate-core/src/extractors/maven.rs:2736`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2736) |
| 264 | tries minimum snapshot manifests | ported | [`crates/renovate-core/src/extractors/maven.rs:2750`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2750) |
| 279 | extracts builder and buildpack images from spring-boot plugin | ported | [`crates/renovate-core/src/extractors/maven.rs:2504`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2504) |
| 370 | extracts only builder if defaults are used in spring-boot plugin | ported | [`crates/renovate-core/src/extractors/maven.rs:2593`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2593) |
| 398 | returns no buildpack dependencies when image tag is missing in spring boot plugin configuration | ported | [`crates/renovate-core/src/extractors/maven.rs:2619`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2619) |
| 407 | returns no buildpack dependencies when dependencies are invalid in spring boot plugin | ported | [`crates/renovate-core/src/extractors/maven.rs:2639`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2639) |
| 418 | should apply props recursively | ported | [`crates/renovate-core/src/extractors/maven.rs:2318`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2318) |
| 433 | should apply props multiple times | ported | [`crates/renovate-core/src/extractors/maven.rs:2763`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2763) |
| 448 | should detect props infinitely recursing props | ported | [`crates/renovate-core/src/extractors/maven.rs:2382`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2382) |
| 471 | returns null for invalid xml | ported | [`crates/renovate-core/src/extractors/maven.rs:1751`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L1751) |
| 478 | extract registries from a simple mirror settings file | ported | [`crates/renovate-core/src/extractors/maven.rs:1760`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L1760) |
| 485 | extract registries from a simple profile settings file | ported | [`crates/renovate-core/src/extractors/maven.rs:1778`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L1778) |
| 492 | extract registries from a complex profile settings file | ported | [`crates/renovate-core/src/extractors/maven.rs:1800`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L1800) |
| 503 | extract registries from a settings file that uses a newer schema | ported | [`crates/renovate-core/src/extractors/maven.rs:1857`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L1857) |
| 527 | returns null for invalid xml files | ported | [`crates/renovate-core/src/extractors/maven.rs:1879`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L1879) |
| 548 | should return empty if package has no content | ported | [`crates/renovate-core/src/extractors/maven.rs:1894`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L1894) |
| 554 | should return empty for packages with invalid content | ported | [`crates/renovate-core/src/extractors/maven.rs:1900`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L1900) |
| 560 | should return packages with urls from a settings file | ported | [`crates/renovate-core/src/extractors/maven.rs:1906`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L1906) |
| 581 | should include registryurls from parent pom files | ported | [`crates/renovate-core/src/extractors/maven.rs:2040`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2040) |
| 791 | should include registryurls in the correct order | ported | [`crates/renovate-core/src/extractors/maven.rs:1942`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L1942) |
| 812 | should return package files info | ported | [`crates/renovate-core/src/extractors/maven.rs:1982`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L1982) |
| 888 | should extract from .mvn/extensions.xml file | ported | [`crates/renovate-core/src/extractors/maven.rs:2093`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2093) |
| 917 | should extract from pom.template.xml file | ported | [`crates/renovate-core/src/extractors/maven.rs:2785`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2785) |
| 998 | should return empty array if extensions file is invalid or empty | ported | [`crates/renovate-core/src/extractors/maven.rs:2114`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2114) |
| 1011 | should skip root pom.xml | ported | [`crates/renovate-core/src/extractors/maven.rs:2126`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2126) |
| 1045 | should skip root pom.xml when it has an external parent | ported | [`crates/renovate-core/src/extractors/maven.rs:2155`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2155) |
| 1087 | handles cross-referencing | ported | [`crates/renovate-core/src/extractors/maven.rs:2188`](../../../../../../../crates/renovate-core/src/extractors/maven.rs#L2188) |

