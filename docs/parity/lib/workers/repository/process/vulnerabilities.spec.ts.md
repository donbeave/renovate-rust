# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/vulnerabilities.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/vulnerabilities.spec.ts
**Total tests:** 38 | **Ported:** 0 | **Actionable:** 38 | **Status:** pending

### `workers/repository/process/vulnerabilities › create()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works, and is a singleton | 29 | pending | — | — | — |
| throws when osv-offline error | 38 | pending | — | — | — |

### `workers/repository/process/vulnerabilities › fetchVulnerabilities()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return list of Vulnerabilities | 62 | pending | — | — | — |

### `workers/repository/process/vulnerabilities › fetchVulnerabilities() › malicious packages`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| are marked for dependencies with a MAL- advisory ID against their current version with malicious-version-in-use | 129 | pending | — | — | — |
| are logged | 251 | pending | — | — | — |
| are not counted if the affected versions do not match | 369 | pending | — | — | — |
| handles a MAL- advisory with no affected field | 426 | pending | — | — | — |
| handles a malicious dependency where updates is undefined | 477 | pending | — | — | — |

### `workers/repository/process/vulnerabilities › fetchVulnerabilities() › malicious packages › when a malicious dependency update is proposed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| applies to dependency updates, and sets malicious-update-proposed | 526 | pending | — | — | — |
| logs | 590 | pending | — | — | — |
| falls back to update.newValue when newVersion is missing, and skips updates that are not malicious | 662 | pending | — | — | — |

### `workers/repository/process/vulnerabilities › appendVulnerabilityPackageRules()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unsupported datasource | 760 | pending | — | — | — |
| package found but no vulnerabilities | 780 | pending | — | — | — |
| vulnerability without affected field | 801 | pending | — | — | — |
| withdrawn vulnerability | 826 | pending | — | — | — |
| invalid dep version | 855 | pending | — | — | — |
| exception while fetching vulnerabilities | 882 | pending | — | — | — |
| log event with invalid version | 911 | pending | — | — | — |
| no version or range affected | 960 | pending | — | — | — |
| no fixed version available | 990 | pending | — | — | — |
| does not accidentally downgrade versions due to fixed version for other range | 1024 | pending | — | — | — |
| vulnerability with multiple unsorted events | 1067 | pending | — | — | — |
| vulnerability with multiple affected entries and version ranges | 1131 | pending | — | — | — |
| package rules are sorted by fixed version even if affected is unsorted | 1203 | pending | — | — | — |
| filters not applicable vulnerability | 1272 | pending | — | — | — |
| returns a single packageRule for regex manager | 1292 | pending | — | — | — |
| returns multiple packageRules for different vulnerabilities | 1395 | pending | — | — | — |
| returns packageRules for Hackage | 1455 | pending | — | — | — |
| filters not applicable vulnerability based on last_affected version | 1512 | pending | — | — | — |
| describe fixed version as ecosystem-specific version constraint | 1555 | pending | — | — | — |
| describe last_affected version as ecosystem-specific version constraint | 1680 | pending | — | — | — |
| returns packageRule for deps-edn package using OSV Maven ecosystem | 1734 | pending | — | — | — |
| returns packageRule based on last_affected version | 1786 | pending | — | — | — |
| handles invalid CVSS scores gracefully | 1862 | pending | — | — | — |
| prefer CVSS_V4 scores over CVSS_V3 | 1945 | pending | — | — | — |
| show severity text in GHSA advisories without CVSS score | 2036 | pending | — | — | — |
| formats headings of vulnerability details | 2097 | pending | — | — | — |

### `workers/repository/process/vulnerabilities › evaluateCvssVector`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 2194 | pending | — | — | — |

---

