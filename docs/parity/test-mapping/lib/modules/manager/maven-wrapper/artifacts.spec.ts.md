# `lib/modules/manager/maven-wrapper/artifacts.spec.ts`

[← `manager/maven-wrapper`](../../../../_by-module/manager/maven-wrapper.md) · [all modules](../../../../README.md)

**0/30 ported** (30 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 63 | should not update if there is no dep with maven:wrapper | pending | — |
| 75 | docker should use java 8 if version is lower then 2.0.0 | pending | — |
| 105 | should update when it is maven wrapper | pending | — |
| 150 | should not update deps when maven-wrapper.properties is not in git change | pending | — |
| 185 | updates with docker | pending | — |
| 247 | should return null when cmd is not found | pending | — |
| 262 | should throw an error when it cant execute | pending | — |
| 282 | updates with binarysource install | pending | — |
| 332 | updates with binarysource install after detecting wrapper version from mvnw script | pending | — |
| 374 | should run wrapper:wrapper with mvnw_repourl if it is a custom artifactory | pending | — |
| 416 | should run not include mvnw_repourl when run with default maven repo url | pending | — |
| 458 | should run not include mvnw_repourl when run with a malformed replacestring | pending | — |
| 506 | should not delete wrapper jar when only maven distribution checksum is updated | pending | — |
| 528 | should update distribution checksum when maven version changes | pending | — |
| 557 | should use cached distribution checksum when available | pending | — |
| 588 | should skip checksum update when current content is missing | pending | — |
| 602 | should update both checksums when wrapper version changes | pending | — |
| 642 | should not attempt to delete old jar file if doing a maven wrapper update | pending | — |
| 675 | should preserve old checksum when fetch fails | pending | — |
| 702 | should restore distribution checksum when fetch fails after stripping | pending | — |
| 727 | should skip http when no checksums in properties file | pending | — |
| 748 | should return null when only maven is updated without checksums | pending | — |
| 761 | should construct wrapper url from version when wrapperurl is missing | pending | — |
| 792 | should add distribution checksum when it does not exist | pending | — |
| 823 | should add wrapper checksum when it does not exist | pending | — |
| 858 | should preserve wrapper checksum when fetch fails | pending | — |
| 888 | should restore wrapper checksum when fetch fails after stripping | pending | — |
| 919 | should unescape distributionurl, honor wrapperversion, and keep distributiontype | pending | — |
| 956 | should skip distribution checksum update when distributionurl is missing | pending | — |
| 974 | should skip wrapper checksum update when wrapperversion is missing | pending | — |

