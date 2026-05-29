# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/maven-wrapper/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/maven-wrapper/artifacts.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 30 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Should not update if there is no dep with maven:wrapper | 60 | pending | — | — | —|
| Docker should use java 8 if version is lower then 2.0.0 | 72 | pending | — | — | —|
| Should update when it is maven wrapper | 102 | pending | — | — | —|
| Should not update deps when maven-wrapper.properties is not in git change | 147 | pending | — | — | —|
| updates with docker | 182 | pending | — | — | —|
| Should return null when cmd is not found | 244 | pending | — | — | —|
| Should throw an error when it cant execute | 259 | pending | — | — | —|
| updates with binarySource install | 279 | pending | — | — | —|
| updates with binarySource install after detecting wrapper version from mvnw script | 329 | pending | — | — | —|
| should run wrapper:wrapper with MVNW_REPOURL if it is a custom artifactory | 371 | pending | — | — | —|
| should run not include MVNW_REPOURL when run with default maven repo url | 413 | pending | — | — | —|
| should run not include MVNW_REPOURL when run with a malformed replaceString | 455 | pending | — | — | —|

### `checksum updates`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should not delete wrapper jar when only maven distribution checksum is updated | 503 | pending | — | — | —|
| should update distribution checksum when maven version changes | 525 | pending | — | — | —|
| should use cached distribution checksum when available | 554 | pending | — | — | —|
| should skip checksum update when current content is missing | 585 | pending | — | — | —|
| should update both checksums when wrapper version changes | 599 | pending | — | — | —|
| should not attempt to delete old JAR file if doing a Maven Wrapper update | 639 | pending | — | — | —|
| should preserve old checksum when fetch fails | 672 | pending | — | — | —|
| should restore distribution checksum when fetch fails after stripping | 699 | pending | — | — | —|
| should skip HTTP when no checksums in properties file | 724 | pending | — | — | —|
| should return null when only maven is updated without checksums | 745 | pending | — | — | —|
| should construct wrapper URL from version when wrapperUrl is missing | 758 | pending | — | — | —|
| should add distribution checksum when it does not exist | 789 | pending | — | — | —|
| should add wrapper checksum when it does not exist | 820 | pending | — | — | —|
| should preserve wrapper checksum when fetch fails | 855 | pending | — | — | —|
| should restore wrapper checksum when fetch fails after stripping | 885 | pending | — | — | —|
| should unescape distributionUrl, honor wrapperVersion, and keep distributionType | 916 | pending | — | — | —|
| should skip distribution checksum update when distributionUrl is missing | 953 | pending | — | — | —|
| should skip wrapper checksum update when wrapperVersion is missing | 971 | pending | — | — | —|

---

