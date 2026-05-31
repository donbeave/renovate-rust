# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/vulnerabilities.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/vulnerabilities.spec.ts
**Total tests:** 41 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `workers/repository/process/vulnerabilities › create()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works, and is a singleton  | 29 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| throws when osv-offline error  | 38 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |

### `workers/repository/process/vulnerabilities › fetchVulnerabilities()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return list of Vulnerabilities  | 62 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |

### `workers/repository/process/vulnerabilities › fetchVulnerabilities() › malicious packages`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| are marked for dependencies with a MAL- advisory ID against their current version with malicious-version-in-use  | 129 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| are logged  | 251 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| are not counted if the affected versions do not match  | 369 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| handles a MAL- advisory with no affected field  | 426 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| handles a malicious dependency where updates is undefined  | 477 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |

### `workers/repository/process/vulnerabilities › fetchVulnerabilities() › malicious packages › when a malicious dependency update is proposed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| applies to dependency updates, and sets malicious-update-proposed  | 526 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| logs  | 590 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| falls back to update.newValue when newVersion is missing, and skips updates that are not malicious  | 662 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |

### `workers/repository/process/vulnerabilities › appendVulnerabilityPackageRules()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unsupported datasource  | 760 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| package found but no vulnerabilities  | 780 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| vulnerability without affected field  | 801 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| withdrawn vulnerability  | 826 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| invalid dep version  | 855 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| exception while fetching vulnerabilities  | 882 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| log event with invalid version  | 911 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| no version or range affected  | 960 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| no fixed version available  | 990 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| does not accidentally downgrade versions due to fixed version for other range  | 1024 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| vulnerability with multiple unsorted events  | 1067 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| vulnerability with multiple affected entries and version ranges  | 1131 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| package rules are sorted by fixed version even if affected is unsorted  | 1203 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| filters not applicable vulnerability  | 1272 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| returns a single packageRule for regex manager  | 1292 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| returns multiple packageRules for different vulnerabilities  | 1395 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| returns packageRules for Hackage  | 1455 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| filters not applicable vulnerability based on last_affected version  | 1512 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| describe fixed version as ecosystem-specific version constraint  | 1555 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| describe last_affected version as ecosystem-specific version constraint  | 1680 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| returns packageRule for deps-edn package using OSV Maven ecosystem  | 1734 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| returns packageRule based on last_affected version  | 1786 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| handles invalid CVSS scores gracefully  | 1862 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| prefer CVSS_V4 scores over CVSS_V3  | 1945 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| show severity text in GHSA advisories without CVSS score  | 2036 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| formats headings of vulnerability details  | 2097 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |

### `workers/repository/process/vulnerabilities › evaluateCvssVector`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input  | 2194 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |

| creates vulnerability alert for go toolchain directive using stdlib  | 1135 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| skips vulnerability lookup for go module directive  | 1196 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
| sets default datasource versioning to align with allowedVersions on packageRule  | 1221 | not-applicable | Mock framework internals — tests vulnerabilities via vitest-mocked datasource/HTTP; Rust tests this at different layer | — | vulnerability scanning behavior is in scope |
---
