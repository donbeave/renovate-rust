# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/process/lookup/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/lookup/index.spec.ts
**Total tests:** 169 | **Ported:** 0 | **Actionable:** 169 | **Status:** pending-applicable

### `workers/repository/process/lookup/index › .lookupUpdates()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if invalid currentValue  | 100 | pending | — | — | No corresponding Rust source|
| returns null if unknown datasource  | 111 | pending | — | — | No corresponding Rust source|
| handles error result from getPkgReleasesWithResult  | 122 | pending | — | — | No corresponding Rust source|
| returns rollback for pinned version  | 134 | pending | — | — | No corresponding Rust source|
| returns rollback for ranged version  | 173 | pending | — | — | No corresponding Rust source|
| supports minor and major upgrades for tilde ranges  | 199 | pending | — | — | No corresponding Rust source|
| supports lock file updates mixed with regular updates  | 249 | pending | — | — | No corresponding Rust source|
| returns multiple updates if grouping but separateMajorMinor=true  | 309 | pending | — | — | No corresponding Rust source|
| returns additional update if grouping but separateMinorPatch=true  | 351 | pending | — | — | No corresponding Rust source|
| returns one update if grouping and separateMajorMinor=false  | 407 | pending | — | — | No corresponding Rust source|
| returns both updates if automerging minor  | 437 | pending | — | — | No corresponding Rust source|
| enforces allowedVersions  | 488 | pending | — | — | No corresponding Rust source|
| enforces allowedVersions with regex  | 516 | pending | — | — | No corresponding Rust source|
| enforces allowedVersions with negative regex  | 544 | pending | — | — | No corresponding Rust source|
| falls back to semver syntax allowedVersions  | 572 | pending | — | — | No corresponding Rust source|
| falls back to pep440 syntax allowedVersions  | 601 | pending | — | — | No corresponding Rust source|
| skips invalid allowedVersions  | 630 | pending | — | — | No corresponding Rust source|
| returns patch update even if separate patches not configured  | 642 | pending | — | — | No corresponding Rust source|
| returns minor update if automerging both patch and minor  | 683 | pending | — | — | No corresponding Rust source|
| returns patch update if separateMinorPatch  | 730 | pending | — | — | No corresponding Rust source|
| returns patch minor and major  | 772 | pending | — | — | No corresponding Rust source|
| disables major release separation (major)  | 827 | pending | — | — | No corresponding Rust source|
| disables major release separation (minor)  | 864 | pending | — | — | No corresponding Rust source|
| uses minimum version for vulnerabilityAlerts  | 893 | pending | — | — | No corresponding Rust source|
| uses highest available version for vulnerabilityAlerts when vulnerabilityFixStrategy=highest  | 921 | pending | — | — | No corresponding Rust source|
| uses vulnerabilityFixVersion when a version  | 950 | pending | — | — | No corresponding Rust source|
| takes a later release when vulnerabilityFixVersion does not exist  | 979 | pending | — | — | No corresponding Rust source|
| uses vulnerabilityFixVersion when a range  | 1008 | pending | — | — | No corresponding Rust source|
| takes highest available version when using vulnerabilityFixStrategy=highest with vulnerabilityFixVersion  | 1037 | pending | — | — | No corresponding Rust source|
| ignores vulnerabilityFixVersion if not a version  | 1067 | pending | — | — | No corresponding Rust source|
| returns no results if vulnerabilityFixVersion is too high  | 1096 | pending | — | — | No corresponding Rust source|
| supports minor and major upgrades for ranged versions  | 1111 | pending | — | — | No corresponding Rust source|
| supports for x-range-all for replaceStrategy = pin (with lockfile) abcd  | 1161 | pending | — | — | No corresponding Rust source|
| doesnt offer updates for x-range-all (with lockfile) when replaceStrategy = $strategy  | 1184 | pending | — | — | No corresponding Rust source|
| supports pinning for x-range-all (no lockfile)  | 1207 | pending | — | — | No corresponding Rust source|
| covers pinning an unsupported x-range-all value  | 1229 | pending | — | — | No corresponding Rust source|
| doesnt offer updates for x-range-all (no lockfile) when replaceStrategy = $strategy  | 1243 | pending | — | — | No corresponding Rust source|
| ignores pinning for ranges when other upgrade exists  | 1266 | pending | — | — | No corresponding Rust source|
| upgrades minor ranged versions  | 1302 | pending | — | — | No corresponding Rust source|
| handles update-lockfile  | 1338 | pending | — | — | No corresponding Rust source|
| handles the in-range-only strategy and updates lockfile within range  | 1369 | pending | — | — | No corresponding Rust source|
| handles the in-range-only strategy and discards changes not within range  | 1400 | pending | — | — | No corresponding Rust source|
| handles unconstrainedValue values  | 1431 | pending | — | — | No corresponding Rust source|
| handles unconstrainedValue values with rangeStrategy !== update-lockfile and isVulnerabilityAlert  | 1461 | pending | — | — | No corresponding Rust source|
| widens minor ranged versions if configured  | 1491 | pending | — | — | No corresponding Rust source|
| replaces minor complex ranged versions if configured  | 1520 | pending | — | — | No corresponding Rust source|
| widens major ranged versions if configured  | 1549 | pending | — | — | No corresponding Rust source|
| replaces major complex ranged versions if configured  | 1581 | pending | — | — | No corresponding Rust source|
| pins minor ranged versions  | 1613 | pending | — | — | No corresponding Rust source|
| uses the locked version for pinning  | 1635 | pending | — | — | No corresponding Rust source|
| ignores minor ranged versions when not pinning  | 1658 | pending | — | — | No corresponding Rust source|
| ignores minor ranged versions when locked  | 1672 | pending | — | — | No corresponding Rust source|
| upgrades tilde ranges  | 1687 | pending | — | — | No corresponding Rust source|
| upgrades .x minor ranges  | 1723 | pending | — | — | No corresponding Rust source|
| upgrades tilde ranges without pinning  | 1759 | pending | — | — | No corresponding Rust source|
| upgrades .x major ranges without pinning  | 1788 | pending | — | — | No corresponding Rust source|
| upgrades .x minor ranges without pinning  | 1817 | pending | — | — | No corresponding Rust source|
| upgrades .x complex minor ranges without pinning  | 1846 | pending | — | — | No corresponding Rust source|
| upgrades shorthand major ranges without pinning  | 1875 | pending | — | — | No corresponding Rust source|
| upgrades shorthand minor ranges without pinning  | 1904 | pending | — | — | No corresponding Rust source|
| upgrades multiple tilde ranges without pinning  | 1933 | pending | — | — | No corresponding Rust source|
| upgrades multiple caret ranges without pinning  | 1976 | pending | — | — | No corresponding Rust source|
| supports complex ranges  | 2019 | pending | — | — | No corresponding Rust source|
| supports complex major ranges  | 2062 | pending | — | — | No corresponding Rust source|
| supports complex major hyphen ranges  | 2094 | pending | — | — | No corresponding Rust source|
| widens .x OR ranges  | 2126 | pending | — | — | No corresponding Rust source|
| widens stanndalone major OR ranges  | 2158 | pending | — | — | No corresponding Rust source|
| supports complex tilde ranges  | 2190 | pending | — | — | No corresponding Rust source|
| returns nothing for greater than ranges  | 2219 | pending | — | — | No corresponding Rust source|
| upgrades less than equal ranges without pinning  | 2233 | pending | — | — | No corresponding Rust source|
| upgrades less than ranges without pinning  | 2276 | pending | — | — | No corresponding Rust source|
| upgrades less than major ranges  | 2319 | pending | — | — | No corresponding Rust source|
| upgrades less than equal minor ranges  | 2348 | pending | — | — | No corresponding Rust source|
| upgrades equal minor ranges  | 2377 | pending | — | — | No corresponding Rust source|
| upgrades less than equal major ranges  | 2406 | pending | — | — | No corresponding Rust source|
| upgrades major less than equal ranges  | 2436 | pending | — | — | No corresponding Rust source|
| upgrades major less than ranges without pinning  | 2465 | pending | — | — | No corresponding Rust source|
| upgrades major greater than less than ranges without pinning  | 2494 | pending | — | — | No corresponding Rust source|
| upgrades minor greater than less than ranges without pinning  | 2523 | pending | — | — | No corresponding Rust source|
| upgrades minor greater than less than equals ranges without pinning  | 2566 | pending | — | — | No corresponding Rust source|
| rejects reverse ordered less than greater than  | 2609 | pending | — | — | No corresponding Rust source|
| supports > latest versions if configured  | 2623 | pending | — | — | No corresponding Rust source|
| should ignore unstable versions if the current version is stable  | 2651 | pending | — | — | No corresponding Rust source|
| should ignore unstable versions from datasource  | 2664 | pending | — | — | No corresponding Rust source|
| should allow unstable versions in same major for node  | 2695 | pending | — | — | No corresponding Rust source|
| should return pendingChecks  | 2727 | pending | — | — | No corresponding Rust source|
| should return pendingVersions  | 2773 | pending | — | — | No corresponding Rust source|
| should allow unstable versions if the ignoreUnstable=false  | 2819 | pending | — | — | No corresponding Rust source|
| should allow unstable versions if the current version is unstable  | 2848 | pending | — | — | No corresponding Rust source|
| should not jump unstable versions  | 2878 | pending | — | — | No corresponding Rust source|
| should update pinned versions if updatePinnedDependencies=true  | 2908 | pending | — | — | No corresponding Rust source|
| should not update pinned versions if updatePinnedDependencies=false  | 2939 | pending | — | — | No corresponding Rust source|
| should follow dist-tag even if newer version exists  | 2956 | pending | — | — | No corresponding Rust source|
| should roll back to dist-tag if current version is higher  | 2987 | pending | — | — | No corresponding Rust source|
| should jump unstable versions if followTag  | 3019 | pending | — | — | No corresponding Rust source|
| should update nothing if current version is dist-tag  | 3050 | pending | — | — | No corresponding Rust source|
| should warn if no version matches dist-tag  | 3067 | pending | — | — | No corresponding Rust source|
| should warn if no digest could be found but there is a current digest  | 3090 | pending | — | — | No corresponding Rust source|

### `workers/repository/process/lookup/index › .lookupUpdates() › pinning enabled but no existing digest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should not warn if no new digest could be found  | 3125 | pending | — | — | No corresponding Rust source|
| should use registry of update to determine digest  | 3154 | pending | — | — | No corresponding Rust source|
| should treat zero zero tilde ranges as 0.0.x  | 3199 | pending | — | — | No corresponding Rust source|
| should treat zero zero caret ranges as pinned  | 3216 | pending | — | — | No corresponding Rust source|
| should downgrade from missing versions  | 3248 | pending | — | — | No corresponding Rust source|
| should upgrade to only one major  | 3279 | pending | — | — | No corresponding Rust source|
| should upgrade to two majors  | 3322 | pending | — | — | No corresponding Rust source|
| should upgrade to 16 minors  | 3380 | pending | — | — | No corresponding Rust source|
| does not jump  major unstable  | 3395 | pending | — | — | No corresponding Rust source|
| supports in-range caret updates  | 3409 | pending | — | — | No corresponding Rust source|
| supports in-range tilde updates  | 3439 | pending | — | — | No corresponding Rust source|
| supports in-range tilde patch updates  | 3484 | pending | — | — | No corresponding Rust source|
| supports in-range gte updates  | 3529 | pending | — | — | No corresponding Rust source|
| supports majorgte updates  | 3559 | pending | — | — | No corresponding Rust source|
| rejects in-range unsupported operator  | 3590 | pending | — | — | No corresponding Rust source|
| rejects non-fully specified in-range updates  | 3604 | pending | — | — | No corresponding Rust source|
| rejects complex range in-range updates  | 3618 | pending | — | — | No corresponding Rust source|
| replaces non-range in-range updates  | 3632 | pending | — | — | No corresponding Rust source|
| handles github 404  | 3661 | pending | — | — | No corresponding Rust source|
| handles pypi 404  | 3675 | pending | — | — | No corresponding Rust source|
| handles packagist  | 3692 | pending | — | — | No corresponding Rust source|
| handles unknown datasource  | 3712 | pending | — | — | No corresponding Rust source|
| handles PEP440  | 3725 | pending | — | — | No corresponding Rust source|
| returns complex object  | 3767 | pending | — | — | No corresponding Rust source|
| prefers lockedVersion  | 3803 | pending | — | — | No corresponding Rust source|
| ignores deprecated when it is not the latest  | 3820 | pending | — | — | No corresponding Rust source|
| treats all versions as deprecated if latest is deprecated  | 3873 | pending | — | — | No corresponding Rust source|
| skips unsupported values  | 3925 | pending | — | — | No corresponding Rust source|
| skips undefined values  | 3942 | pending | — | — | No corresponding Rust source|
| handles digest pin  | 3958 | pending | — | — | No corresponding Rust source|
| skips uncompatible versions for 8.1.0  | 4010 | pending | — | — | No corresponding Rust source|
| skips uncompatible versions for 8.1  | 4056 | pending | — | — | No corresponding Rust source|
| skips uncompatible versions for 8  | 4114 | pending | — | — | No corresponding Rust source|
| applies versionCompatibility for 18.10.0  | 4160 | pending | — | — | No corresponding Rust source|
| applies versionCompatibility for maven  | 4232 | pending | — | — | No corresponding Rust source|
| handles versionCompatibility mismatch  | 4272 | pending | — | — | No corresponding Rust source|
| applies versionCompatibility for debian codenames with suffix  | 4299 | pending | — | — | No corresponding Rust source|
| handles digest pin for up to date version  | 4340 | pending | — | — | No corresponding Rust source|
| handles no fitting version and no version in lock file  | 4379 | pending | — | — | No corresponding Rust source|
| handles digest pin for non-version  | 4408 | pending | — | — | No corresponding Rust source|
| handles digest lookup failure  | 4446 | pending | — | — | No corresponding Rust source|
| handles digest update  | 4473 | pending | — | — | No corresponding Rust source|
| handles digest update for custom datasource  | 4525 | pending | — | — | No corresponding Rust source|
| handles digest update for non-version  | 4552 | pending | — | — | No corresponding Rust source|
| handles git submodule update  | 4590 | pending | — | — | No corresponding Rust source|
| handles sourceUrl packageRules with version restrictions  | 4613 | pending | — | — | No corresponding Rust source|
| handles current age packageRules with version restrictions  | 4655 | pending | — | — | No corresponding Rust source|
| does not apply package rules for matchCurrentAge if packageRules doesn not have a current age matcher  | 4703 | pending | — | — | No corresponding Rust source|
| does not apply package rules for matchCurrentAge if the releaseTimestamp for current version is missing  | 4762 | pending | — | — | No corresponding Rust source|
| handles replacements - name only without pinDigests enabled  | 4815 | pending | — | — | No corresponding Rust source|
| handles replacements - name only with pinDigests enabled  | 4857 | pending | — | — | No corresponding Rust source|
| handles replacements - name only no version/tag  | 4912 | pending | — | — | No corresponding Rust source|
| handles replacements - Digest configured and validating getDigest funtion call  | 4934 | pending | — | — | No corresponding Rust source|
| handles replacements - Digest configured with replacementNameTemplate and validating getDigest function call  | 4994 | pending | — | — | No corresponding Rust source|
| handles replacements - skips if package and replacement names match  | 5083 | pending | — | — | No corresponding Rust source|
| handles replacements - name and version  | 5096 | pending | — | — | No corresponding Rust source|
| handles replacements - can template replacement name without a replacement version  | 5117 | pending | — | — | No corresponding Rust source|
| handles replacements - can template replacement name with a replacement version  | 5156 | pending | — | — | No corresponding Rust source|
| handles replacements - replacementName takes precedence over replacementNameTemplate  | 5196 | pending | — | — | No corresponding Rust source|
| handles replacements - can template replacement version without a replacement name  | 5236 | pending | — | — | No corresponding Rust source|
| handles replacements - can template replacement version with a replacement name  | 5275 | pending | — | — | No corresponding Rust source|
| handles replacements - can template replacement version with a template replacement name  | 5315 | pending | — | — | No corresponding Rust source|
| handles replacements - replacementVersion takes precedence over replacementVersionTemplate  | 5355 | pending | — | — | No corresponding Rust source|
| handles replacements - can perform replacement even for invalid versioning  | 5395 | pending | — | — | No corresponding Rust source|
| handles replacements - from datasource  | 5422 | pending | — | — | No corresponding Rust source|
| rollback for invalid version to last stable version  | 5445 | pending | — | — | No corresponding Rust source|

### `workers/repository/process/lookup/index › .lookupUpdates() › handles merge confidence`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets a merge confidence level for a given update when corresponding packageRule is in use  | 5493 | pending | — | — | No corresponding Rust source|
| does not get a merge confidence level when no packageRule is set  | 5536 | pending | — | — | No corresponding Rust source|
| does not set merge confidence value when API is not in use  | 5567 | pending | — | — | No corresponding Rust source|
| detects gomod updates and uses updateType=digest when appropriate  | 5588 | pending | — | — | No corresponding Rust source|
| handles changelog with content  | 5629 | pending | — | — | No corresponding Rust source|

---
