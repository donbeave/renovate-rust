# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/vulnerabilities.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/vulnerabilities.spec.ts
**Total tests:** 41 | **Ported:** 0 | **Actionable:** 41 | **Status:** pending-applicable

### `workers/repository/process/vulnerabilities › create()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works, and is a singleton  | 29 | pending | — | — | vulnerability scanning behavior is in scope |
| throws when osv-offline error  | 38 | pending | — | — | vulnerability scanning behavior is in scope |

### `workers/repository/process/vulnerabilities › fetchVulnerabilities()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return list of Vulnerabilities  | 62 | pending | — | — | vulnerability scanning behavior is in scope |

### `workers/repository/process/vulnerabilities › fetchVulnerabilities() › malicious packages`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| are marked for dependencies with a MAL- advisory ID against their current version with malicious-version-in-use  | 129 | pending | — | — | vulnerability scanning behavior is in scope |
| are logged  | 251 | pending | — | — | vulnerability scanning behavior is in scope |
| are not counted if the affected versions do not match  | 369 | pending | — | — | vulnerability scanning behavior is in scope |
| handles a MAL- advisory with no affected field  | 426 | pending | — | — | vulnerability scanning behavior is in scope |
| handles a malicious dependency where updates is undefined  | 477 | pending | — | — | vulnerability scanning behavior is in scope |

### `workers/repository/process/vulnerabilities › fetchVulnerabilities() › malicious packages › when a malicious dependency update is proposed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| applies to dependency updates, and sets malicious-update-proposed  | 526 | pending | — | — | vulnerability scanning behavior is in scope |
| logs  | 590 | pending | — | — | vulnerability scanning behavior is in scope |
| falls back to update.newValue when newVersion is missing, and skips updates that are not malicious  | 662 | pending | — | — | vulnerability scanning behavior is in scope |

### `workers/repository/process/vulnerabilities › appendVulnerabilityPackageRules()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unsupported datasource  | 760 | pending | — | — | vulnerability scanning behavior is in scope |
| package found but no vulnerabilities  | 780 | pending | — | — | vulnerability scanning behavior is in scope |
| vulnerability without affected field  | 801 | pending | — | — | vulnerability scanning behavior is in scope |
| withdrawn vulnerability  | 826 | pending | — | — | vulnerability scanning behavior is in scope |
| invalid dep version  | 855 | pending | — | — | vulnerability scanning behavior is in scope |
| exception while fetching vulnerabilities  | 882 | pending | — | — | vulnerability scanning behavior is in scope |
| log event with invalid version  | 911 | pending | — | — | vulnerability scanning behavior is in scope |
| no version or range affected  | 960 | pending | — | — | vulnerability scanning behavior is in scope |
| no fixed version available  | 990 | pending | — | — | vulnerability scanning behavior is in scope |
| does not accidentally downgrade versions due to fixed version for other range  | 1024 | pending | — | — | vulnerability scanning behavior is in scope |
| vulnerability with multiple unsorted events  | 1067 | pending | — | — | vulnerability scanning behavior is in scope |
| vulnerability with multiple affected entries and version ranges  | 1131 | pending | — | — | vulnerability scanning behavior is in scope |
| package rules are sorted by fixed version even if affected is unsorted  | 1203 | pending | — | — | vulnerability scanning behavior is in scope |
| filters not applicable vulnerability  | 1272 | pending | — | — | vulnerability scanning behavior is in scope |
| returns a single packageRule for regex manager  | 1292 | pending | — | — | vulnerability scanning behavior is in scope |
| returns multiple packageRules for different vulnerabilities  | 1395 | pending | — | — | vulnerability scanning behavior is in scope |
| returns packageRules for Hackage  | 1455 | pending | — | — | vulnerability scanning behavior is in scope |
| filters not applicable vulnerability based on last_affected version  | 1512 | pending | — | — | vulnerability scanning behavior is in scope |
| describe fixed version as ecosystem-specific version constraint  | 1555 | pending | — | — | vulnerability scanning behavior is in scope |
| describe last_affected version as ecosystem-specific version constraint  | 1680 | pending | — | — | vulnerability scanning behavior is in scope |
| returns packageRule for deps-edn package using OSV Maven ecosystem  | 1734 | pending | — | — | vulnerability scanning behavior is in scope |
| returns packageRule based on last_affected version  | 1786 | pending | — | — | vulnerability scanning behavior is in scope |
| handles invalid CVSS scores gracefully  | 1862 | pending | — | — | vulnerability scanning behavior is in scope |
| prefer CVSS_V4 scores over CVSS_V3  | 1945 | pending | — | — | vulnerability scanning behavior is in scope |
| show severity text in GHSA advisories without CVSS score  | 2036 | pending | — | — | vulnerability scanning behavior is in scope |
| formats headings of vulnerability details  | 2097 | pending | — | — | vulnerability scanning behavior is in scope |

### `workers/repository/process/vulnerabilities › evaluateCvssVector`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input  | 2194 | pending | — | — | vulnerability scanning behavior is in scope |

| creates vulnerability alert for go toolchain directive using stdlib  | 1135 | pending | — | — | vulnerability scanning behavior is in scope |
| skips vulnerability lookup for go module directive  | 1196 | pending | — | — | vulnerability scanning behavior is in scope |
| sets default datasource versioning to align with allowedVersions on packageRule  | 1221 | pending | — | — | vulnerability scanning behavior is in scope |
---
