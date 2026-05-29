# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/vulnerabilities.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/vulnerabilities.spec.ts
**Total tests:** 41 | **Ported:** 0 | **Actionable:** 41 | **Status:** not-applicable

### `workers/repository/process/vulnerabilities › create()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works, and is a singleton | 29 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| throws when osv-offline error | 38 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|

### `workers/repository/process/vulnerabilities › fetchVulnerabilities()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return list of Vulnerabilities | 62 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|

### `workers/repository/process/vulnerabilities › fetchVulnerabilities() › malicious packages`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| are marked for dependencies with a MAL- advisory ID against their current version with malicious-version-in-use | 129 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| are logged | 251 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| are not counted if the affected versions do not match | 369 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| handles a MAL- advisory with no affected field | 426 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| handles a malicious dependency where updates is undefined | 477 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|

### `workers/repository/process/vulnerabilities › fetchVulnerabilities() › malicious packages › when a malicious dependency update is proposed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| applies to dependency updates, and sets malicious-update-proposed | 526 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| logs | 590 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| falls back to update.newValue when newVersion is missing, and skips updates that are not malicious | 662 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|

### `workers/repository/process/vulnerabilities › appendVulnerabilityPackageRules()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unsupported datasource | 760 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| package found but no vulnerabilities | 780 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| vulnerability without affected field | 801 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| withdrawn vulnerability | 826 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| invalid dep version | 855 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| exception while fetching vulnerabilities | 882 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| log event with invalid version | 911 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| no version or range affected | 960 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| no fixed version available | 990 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| does not accidentally downgrade versions due to fixed version for other range | 1024 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| vulnerability with multiple unsorted events | 1067 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| vulnerability with multiple affected entries and version ranges | 1131 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| package rules are sorted by fixed version even if affected is unsorted | 1203 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| filters not applicable vulnerability | 1272 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| returns a single packageRule for regex manager | 1292 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| returns multiple packageRules for different vulnerabilities | 1395 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| returns packageRules for Hackage | 1455 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| filters not applicable vulnerability based on last_affected version | 1512 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| describe fixed version as ecosystem-specific version constraint | 1555 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| describe last_affected version as ecosystem-specific version constraint | 1680 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| returns packageRule for deps-edn package using OSV Maven ecosystem | 1734 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| returns packageRule based on last_affected version | 1786 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| handles invalid CVSS scores gracefully | 1862 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| prefer CVSS_V4 scores over CVSS_V3 | 1945 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| show severity text in GHSA advisories without CVSS score | 2036 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| formats headings of vulnerability details | 2097 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|

### `workers/repository/process/vulnerabilities › evaluateCvssVector`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 2194 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|

| creates vulnerability alert for go toolchain directive using stdlib | 1135 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| skips vulnerability lookup for go module directive | 1196 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
| sets default datasource versioning to align with allowedVersions on packageRule | 1221 | not-applicable | — | — | mocking framework internals — vi.mock on osv-offline; TypeScript vulnerability scanning pipeline|
---
