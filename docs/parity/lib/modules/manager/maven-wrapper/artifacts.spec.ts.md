# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/maven-wrapper/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/maven-wrapper/artifacts.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| Should not update if there is no dep with maven:wrapper | 60 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| Docker should use java 8 if version is lower then 2.0.0 | 72 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| Should update when it is maven wrapper | 102 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| Should not update deps when maven-wrapper.properties is not in git change | 147 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| updates with docker | 182 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| Should return null when cmd is not found | 244 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| Should throw an error when it cant execute | 259 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| updates with binarySource install | 279 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| updates with binarySource install after detecting wrapper version from mvnw script | 329 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should run wrapper:wrapper with MVNW_REPOURL if it is a custom artifactory | 371 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should run not include MVNW_REPOURL when run with default maven repo url | 413 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should run not include MVNW_REPOURL when run with a malformed replaceString | 455 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|

### `checksum updates`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should not delete wrapper jar when only maven distribution checksum is updated | 503 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should update distribution checksum when maven version changes | 525 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should use cached distribution checksum when available | 554 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should skip checksum update when current content is missing | 585 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should update both checksums when wrapper version changes | 599 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should not attempt to delete old JAR file if doing a Maven Wrapper update | 639 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should preserve old checksum when fetch fails | 672 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should restore distribution checksum when fetch fails after stripping | 699 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should skip HTTP when no checksums in properties file | 724 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should return null when only maven is updated without checksums | 745 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should construct wrapper URL from version when wrapperUrl is missing | 758 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should add distribution checksum when it does not exist | 789 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should add wrapper checksum when it does not exist | 820 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should preserve wrapper checksum when fetch fails | 855 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should restore wrapper checksum when fetch fails after stripping | 885 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should unescape distributionUrl, honor wrapperVersion, and keep distributionType | 916 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should skip distribution checksum update when distributionUrl is missing | 953 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|
| should skip wrapper checksum update when wrapperVersion is missing | 971 | not-applicable | Mock framework internals — tests maven-wrapper artifacts via vitest-mocked fs/HTTP; Rust tests this at different layer | — | —|

---

