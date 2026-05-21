# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/maven-wrapper/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/maven-wrapper/artifacts.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Should not update if there is no dep with maven:wrapper | 60 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| Docker should use java 8 if version is lower then 2.0.0 | 72 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| Should update when it is maven wrapper | 102 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| Should not update deps when maven-wrapper.properties is not in git change | 147 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates with docker | 182 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| Should return null when cmd is not found | 244 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| Should throw an error when it cant execute | 259 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates with binarySource install | 279 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| updates with binarySource install after detecting wrapper version from mvnw script | 329 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should run wrapper:wrapper with MVNW_REPOURL if it is a custom artifactory | 371 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should run not include MVNW_REPOURL when run with default maven repo url | 413 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should run not include MVNW_REPOURL when run with a malformed replaceString | 455 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

### `checksum updates`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should not delete wrapper jar when only maven distribution checksum is updated | 503 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should update distribution checksum when maven version changes | 525 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should use cached distribution checksum when available | 554 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should skip checksum update when current content is missing | 585 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should update both checksums when wrapper version changes | 599 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should not attempt to delete old JAR file if doing a Maven Wrapper update | 639 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should preserve old checksum when fetch fails | 672 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should restore distribution checksum when fetch fails after stripping | 699 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should skip HTTP when no checksums in properties file | 724 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should return null when only maven is updated without checksums | 745 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should construct wrapper URL from version when wrapperUrl is missing | 758 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should add distribution checksum when it does not exist | 789 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should add wrapper checksum when it does not exist | 820 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should preserve wrapper checksum when fetch fails | 855 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should restore wrapper checksum when fetch fails after stripping | 885 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should unescape distributionUrl, honor wrapperVersion, and keep distributionType | 916 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should skip distribution checksum update when distributionUrl is missing | 953 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| should skip wrapper checksum update when wrapperVersion is missing | 971 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---

