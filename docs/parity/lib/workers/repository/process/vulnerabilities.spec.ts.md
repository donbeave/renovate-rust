# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/vulnerabilities.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/vulnerabilities.spec.ts
**Total tests:** 38 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/process/vulnerabilities › create()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works, and is a singleton | 29 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| throws when osv-offline error | 38 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |

### `workers/repository/process/vulnerabilities › fetchVulnerabilities()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return list of Vulnerabilities | 62 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |

### `workers/repository/process/vulnerabilities › fetchVulnerabilities() › malicious packages`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| are marked for dependencies with a MAL- advisory ID against their current version with malicious-version-in-use | 129 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| are logged | 251 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| are not counted if the affected versions do not match | 369 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| handles a MAL- advisory with no affected field | 426 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| handles a malicious dependency where updates is undefined | 477 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |

### `workers/repository/process/vulnerabilities › fetchVulnerabilities() › malicious packages › when a malicious dependency update is proposed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| applies to dependency updates, and sets malicious-update-proposed | 526 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| logs | 590 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| falls back to update.newValue when newVersion is missing, and skips updates that are not malicious | 662 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |

### `workers/repository/process/vulnerabilities › appendVulnerabilityPackageRules()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unsupported datasource | 760 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| package found but no vulnerabilities | 780 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| vulnerability without affected field | 801 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| withdrawn vulnerability | 826 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| invalid dep version | 855 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| exception while fetching vulnerabilities | 882 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| log event with invalid version | 911 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| no version or range affected | 960 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| no fixed version available | 990 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| does not accidentally downgrade versions due to fixed version for other range | 1024 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| vulnerability with multiple unsorted events | 1067 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| vulnerability with multiple affected entries and version ranges | 1131 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| package rules are sorted by fixed version even if affected is unsorted | 1203 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| filters not applicable vulnerability | 1272 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| returns a single packageRule for regex manager | 1292 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| returns multiple packageRules for different vulnerabilities | 1395 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| returns packageRules for Hackage | 1455 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| filters not applicable vulnerability based on last_affected version | 1512 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| describe fixed version as ecosystem-specific version constraint | 1555 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| describe last_affected version as ecosystem-specific version constraint | 1680 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| returns packageRule for deps-edn package using OSV Maven ecosystem | 1734 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| returns packageRule based on last_affected version | 1786 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| handles invalid CVSS scores gracefully | 1862 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| prefer CVSS_V4 scores over CVSS_V3 | 1945 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| show severity text in GHSA advisories without CVSS score | 2036 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |
| formats headings of vulnerability details | 2097 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |

### `workers/repository/process/vulnerabilities › evaluateCvssVector`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 2194 | not-applicable | — | — | tests vulnerability data fetching via Osv API HTTP calls; external API calls out of scope |

---
