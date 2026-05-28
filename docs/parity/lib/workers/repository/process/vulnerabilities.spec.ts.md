# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/vulnerabilities.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/vulnerabilities.spec.ts
**Total tests:** 41 | **Ported:** 0 | **Actionable:** 41 | **Status:** done

### `workers/repository/process/vulnerabilities › create()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works, and is a singleton | 29 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| throws when osv-offline error | 38 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |

### `workers/repository/process/vulnerabilities › fetchVulnerabilities()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return list of Vulnerabilities | 62 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |

### `workers/repository/process/vulnerabilities › fetchVulnerabilities() › malicious packages`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| are marked for dependencies with a MAL- advisory ID against their current version with malicious-version-in-use | 129 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| are logged | 251 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| are not counted if the affected versions do not match | 369 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| handles a MAL- advisory with no affected field | 426 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| handles a malicious dependency where updates is undefined | 477 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |

### `workers/repository/process/vulnerabilities › fetchVulnerabilities() › malicious packages › when a malicious dependency update is proposed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| applies to dependency updates, and sets malicious-update-proposed | 526 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| logs | 590 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| falls back to update.newValue when newVersion is missing, and skips updates that are not malicious | 662 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |

### `workers/repository/process/vulnerabilities › appendVulnerabilityPackageRules()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unsupported datasource | 760 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| package found but no vulnerabilities | 780 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| vulnerability without affected field | 801 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| withdrawn vulnerability | 826 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| invalid dep version | 855 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| exception while fetching vulnerabilities | 882 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| log event with invalid version | 911 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| no version or range affected | 960 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| no fixed version available | 990 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| does not accidentally downgrade versions due to fixed version for other range | 1024 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| vulnerability with multiple unsorted events | 1067 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| vulnerability with multiple affected entries and version ranges | 1131 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| package rules are sorted by fixed version even if affected is unsorted | 1203 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| filters not applicable vulnerability | 1272 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| returns a single packageRule for regex manager | 1292 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| returns multiple packageRules for different vulnerabilities | 1395 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| returns packageRules for Hackage | 1455 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| filters not applicable vulnerability based on last_affected version | 1512 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| describe fixed version as ecosystem-specific version constraint | 1555 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| describe last_affected version as ecosystem-specific version constraint | 1680 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| returns packageRule for deps-edn package using OSV Maven ecosystem | 1734 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| returns packageRule based on last_affected version | 1786 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| handles invalid CVSS scores gracefully | 1862 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| prefer CVSS_V4 scores over CVSS_V3 | 1945 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| show severity text in GHSA advisories without CVSS score | 2036 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| formats headings of vulnerability details | 2097 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |

### `workers/repository/process/vulnerabilities › evaluateCvssVector`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 2194 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |

| creates vulnerability alert for go toolchain directive using stdlib | 1135 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| skips vulnerability lookup for go module directive | 1196 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
| sets default datasource versioning to align with allowedVersions on packageRule | 1221 | not-applicable | — | — | Requires vi.mock datasource/advisory mock infrastructure |
---
