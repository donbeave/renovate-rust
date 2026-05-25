# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/process/lookup/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/lookup/index.spec.ts
**Total tests:** 169 | **Ported:** 0 | **Actionable:** 169 | **Status:** pending

### `workers/repository/process/lookup/index › .lookupUpdates()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if invalid currentValue | 100 | pending | — | — | — |
| returns null if unknown datasource | 111 | pending | — | — | — |
| handles error result from getPkgReleasesWithResult | 122 | pending | — | — | — |
| returns rollback for pinned version | 134 | pending | — | — | — |
| returns rollback for ranged version | 173 | pending | — | — | — |
| supports minor and major upgrades for tilde ranges | 199 | pending | — | — | — |
| supports lock file updates mixed with regular updates | 249 | pending | — | — | — |
| returns multiple updates if grouping but separateMajorMinor=true | 309 | pending | — | — | — |
| returns additional update if grouping but separateMinorPatch=true | 351 | pending | — | — | — |
| returns one update if grouping and separateMajorMinor=false | 407 | pending | — | — | — |
| returns both updates if automerging minor | 437 | pending | — | — | — |
| enforces allowedVersions | 488 | pending | — | — | — |
| enforces allowedVersions with regex | 516 | pending | — | — | — |
| enforces allowedVersions with negative regex | 544 | pending | — | — | — |
| falls back to semver syntax allowedVersions | 572 | pending | — | — | — |
| falls back to pep440 syntax allowedVersions | 601 | pending | — | — | — |
| skips invalid allowedVersions | 630 | pending | — | — | — |
| returns patch update even if separate patches not configured | 642 | pending | — | — | — |
| returns minor update if automerging both patch and minor | 683 | pending | — | — | — |
| returns patch update if separateMinorPatch | 730 | pending | — | — | — |
| returns patch minor and major | 772 | pending | — | — | — |
| disables major release separation (major) | 827 | pending | — | — | — |
| disables major release separation (minor) | 864 | pending | — | — | — |
| uses minimum version for vulnerabilityAlerts | 893 | pending | — | — | — |
| uses highest available version for vulnerabilityAlerts when vulnerabilityFixStrategy=highest | 921 | pending | — | — | — |
| uses vulnerabilityFixVersion when a version | 950 | pending | — | — | — |
| takes a later release when vulnerabilityFixVersion does not exist | 979 | pending | — | — | — |
| uses vulnerabilityFixVersion when a range | 1008 | pending | — | — | — |
| takes highest available version when using vulnerabilityFixStrategy=highest with vulnerabilityFixVersion | 1037 | pending | — | — | — |
| ignores vulnerabilityFixVersion if not a version | 1067 | pending | — | — | — |
| returns no results if vulnerabilityFixVersion is too high | 1096 | pending | — | — | — |
| supports minor and major upgrades for ranged versions | 1111 | pending | — | — | — |
| supports for x-range-all for replaceStrategy = pin (with lockfile) abcd | 1161 | pending | — | — | — |
| doesnt offer updates for x-range-all (with lockfile) when replaceStrategy = $strategy | 1184 | pending | — | — | — |
| supports pinning for x-range-all (no lockfile) | 1207 | pending | — | — | — |
| covers pinning an unsupported x-range-all value | 1229 | pending | — | — | — |
| doesnt offer updates for x-range-all (no lockfile) when replaceStrategy = $strategy | 1243 | pending | — | — | — |
| ignores pinning for ranges when other upgrade exists | 1266 | pending | — | — | — |
| upgrades minor ranged versions | 1302 | pending | — | — | — |
| handles update-lockfile | 1338 | pending | — | — | — |
| handles the in-range-only strategy and updates lockfile within range | 1369 | pending | — | — | — |
| handles the in-range-only strategy and discards changes not within range | 1400 | pending | — | — | — |
| handles unconstrainedValue values | 1431 | pending | — | — | — |
| handles unconstrainedValue values with rangeStrategy !== update-lockfile and isVulnerabilityAlert | 1461 | pending | — | — | — |
| widens minor ranged versions if configured | 1491 | pending | — | — | — |
| replaces minor complex ranged versions if configured | 1520 | pending | — | — | — |
| widens major ranged versions if configured | 1549 | pending | — | — | — |
| replaces major complex ranged versions if configured | 1581 | pending | — | — | — |
| pins minor ranged versions | 1613 | pending | — | — | — |
| uses the locked version for pinning | 1635 | pending | — | — | — |
| ignores minor ranged versions when not pinning | 1658 | pending | — | — | — |
| ignores minor ranged versions when locked | 1672 | pending | — | — | — |
| upgrades tilde ranges | 1687 | pending | — | — | — |
| upgrades .x minor ranges | 1723 | pending | — | — | — |
| upgrades tilde ranges without pinning | 1759 | pending | — | — | — |
| upgrades .x major ranges without pinning | 1788 | pending | — | — | — |
| upgrades .x minor ranges without pinning | 1817 | pending | — | — | — |
| upgrades .x complex minor ranges without pinning | 1846 | pending | — | — | — |
| upgrades shorthand major ranges without pinning | 1875 | pending | — | — | — |
| upgrades shorthand minor ranges without pinning | 1904 | pending | — | — | — |
| upgrades multiple tilde ranges without pinning | 1933 | pending | — | — | — |
| upgrades multiple caret ranges without pinning | 1976 | pending | — | — | — |
| supports complex ranges | 2019 | pending | — | — | — |
| supports complex major ranges | 2062 | pending | — | — | — |
| supports complex major hyphen ranges | 2094 | pending | — | — | — |
| widens .x OR ranges | 2126 | pending | — | — | — |
| widens stanndalone major OR ranges | 2158 | pending | — | — | — |
| supports complex tilde ranges | 2190 | pending | — | — | — |
| returns nothing for greater than ranges | 2219 | pending | — | — | — |
| upgrades less than equal ranges without pinning | 2233 | pending | — | — | — |
| upgrades less than ranges without pinning | 2276 | pending | — | — | — |
| upgrades less than major ranges | 2319 | pending | — | — | — |
| upgrades less than equal minor ranges | 2348 | pending | — | — | — |
| upgrades equal minor ranges | 2377 | pending | — | — | — |
| upgrades less than equal major ranges | 2406 | pending | — | — | — |
| upgrades major less than equal ranges | 2436 | pending | — | — | — |
| upgrades major less than ranges without pinning | 2465 | pending | — | — | — |
| upgrades major greater than less than ranges without pinning | 2494 | pending | — | — | — |
| upgrades minor greater than less than ranges without pinning | 2523 | pending | — | — | — |
| upgrades minor greater than less than equals ranges without pinning | 2566 | pending | — | — | — |
| rejects reverse ordered less than greater than | 2609 | pending | — | — | — |
| supports > latest versions if configured | 2623 | pending | — | — | — |
| should ignore unstable versions if the current version is stable | 2651 | pending | — | — | — |
| should ignore unstable versions from datasource | 2664 | pending | — | — | — |
| should allow unstable versions in same major for node | 2695 | pending | — | — | — |
| should return pendingChecks | 2727 | pending | — | — | — |
| should return pendingVersions | 2773 | pending | — | — | — |
| should allow unstable versions if the ignoreUnstable=false | 2819 | pending | — | — | — |
| should allow unstable versions if the current version is unstable | 2848 | pending | — | — | — |
| should not jump unstable versions | 2878 | pending | — | — | — |
| should update pinned versions if updatePinnedDependencies=true | 2908 | pending | — | — | — |
| should not update pinned versions if updatePinnedDependencies=false | 2939 | pending | — | — | — |
| should follow dist-tag even if newer version exists | 2956 | pending | — | — | — |
| should roll back to dist-tag if current version is higher | 2987 | pending | — | — | — |
| should jump unstable versions if followTag | 3019 | pending | — | — | — |
| should update nothing if current version is dist-tag | 3050 | pending | — | — | — |
| should warn if no version matches dist-tag | 3067 | pending | — | — | — |
| should warn if no digest could be found but there is a current digest | 3090 | pending | — | — | — |

### `workers/repository/process/lookup/index › .lookupUpdates() › pinning enabled but no existing digest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should not warn if no new digest could be found | 3125 | pending | — | — | — |
| should use registry of update to determine digest | 3154 | pending | — | — | — |
| should treat zero zero tilde ranges as 0.0.x | 3199 | pending | — | — | — |
| should treat zero zero caret ranges as pinned | 3216 | pending | — | — | — |
| should downgrade from missing versions | 3248 | pending | — | — | — |
| should upgrade to only one major | 3279 | pending | — | — | — |
| should upgrade to two majors | 3322 | pending | — | — | — |
| should upgrade to 16 minors | 3380 | pending | — | — | — |
| does not jump  major unstable | 3395 | pending | — | — | — |
| supports in-range caret updates | 3409 | pending | — | — | — |
| supports in-range tilde updates | 3439 | pending | — | — | — |
| supports in-range tilde patch updates | 3484 | pending | — | — | — |
| supports in-range gte updates | 3529 | pending | — | — | — |
| supports majorgte updates | 3559 | pending | — | — | — |
| rejects in-range unsupported operator | 3590 | pending | — | — | — |
| rejects non-fully specified in-range updates | 3604 | pending | — | — | — |
| rejects complex range in-range updates | 3618 | pending | — | — | — |
| replaces non-range in-range updates | 3632 | pending | — | — | — |
| handles github 404 | 3661 | pending | — | — | — |
| handles pypi 404 | 3675 | pending | — | — | — |
| handles packagist | 3692 | pending | — | — | — |
| handles unknown datasource | 3712 | pending | — | — | — |
| handles PEP440 | 3725 | pending | — | — | — |
| returns complex object | 3767 | pending | — | — | — |
| prefers lockedVersion | 3803 | pending | — | — | — |
| ignores deprecated when it is not the latest | 3820 | pending | — | — | — |
| treats all versions as deprecated if latest is deprecated | 3873 | pending | — | — | — |
| skips unsupported values | 3925 | pending | — | — | — |
| skips undefined values | 3942 | pending | — | — | — |
| handles digest pin | 3958 | pending | — | — | — |
| skips uncompatible versions for 8.1.0 | 4010 | pending | — | — | — |
| skips uncompatible versions for 8.1 | 4056 | pending | — | — | — |
| skips uncompatible versions for 8 | 4114 | pending | — | — | — |
| applies versionCompatibility for 18.10.0 | 4160 | pending | — | — | — |
| applies versionCompatibility for maven | 4232 | pending | — | — | — |
| handles versionCompatibility mismatch | 4272 | pending | — | — | — |
| applies versionCompatibility for debian codenames with suffix | 4299 | pending | — | — | — |
| handles digest pin for up to date version | 4340 | pending | — | — | — |
| handles no fitting version and no version in lock file | 4379 | pending | — | — | — |
| handles digest pin for non-version | 4408 | pending | — | — | — |
| handles digest lookup failure | 4446 | pending | — | — | — |
| handles digest update | 4473 | pending | — | — | — |
| handles digest update for custom datasource | 4525 | pending | — | — | — |
| handles digest update for non-version | 4552 | pending | — | — | — |
| handles git submodule update | 4590 | pending | — | — | — |
| handles sourceUrl packageRules with version restrictions | 4613 | pending | — | — | — |
| handles current age packageRules with version restrictions | 4655 | pending | — | — | — |
| does not apply package rules for matchCurrentAge if packageRules doesn not have a current age matcher | 4703 | pending | — | — | — |
| does not apply package rules for matchCurrentAge if the releaseTimestamp for current version is missing | 4762 | pending | — | — | — |
| handles replacements - name only without pinDigests enabled | 4815 | pending | — | — | — |
| handles replacements - name only with pinDigests enabled | 4857 | pending | — | — | — |
| handles replacements - name only no version/tag | 4912 | pending | — | — | — |
| handles replacements - Digest configured and validating getDigest funtion call | 4934 | pending | — | — | — |
| handles replacements - Digest configured with replacementNameTemplate and validating getDigest function call | 4994 | pending | — | — | — |
| handles replacements - skips if package and replacement names match | 5083 | pending | — | — | — |
| handles replacements - name and version | 5096 | pending | — | — | — |
| handles replacements - can template replacement name without a replacement version | 5117 | pending | — | — | — |
| handles replacements - can template replacement name with a replacement version | 5156 | pending | — | — | — |
| handles replacements - replacementName takes precedence over replacementNameTemplate | 5196 | pending | — | — | — |
| handles replacements - can template replacement version without a replacement name | 5236 | pending | — | — | — |
| handles replacements - can template replacement version with a replacement name | 5275 | pending | — | — | — |
| handles replacements - can template replacement version with a template replacement name | 5315 | pending | — | — | — |
| handles replacements - replacementVersion takes precedence over replacementVersionTemplate | 5355 | pending | — | — | — |
| handles replacements - can perform replacement even for invalid versioning | 5395 | pending | — | — | — |
| handles replacements - from datasource | 5422 | pending | — | — | — |
| rollback for invalid version to last stable version | 5445 | pending | — | — | — |

### `workers/repository/process/lookup/index › .lookupUpdates() › handles merge confidence`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets a merge confidence level for a given update when corresponding packageRule is in use | 5493 | pending | — | — | — |
| does not get a merge confidence level when no packageRule is set | 5536 | pending | — | — | — |
| does not set merge confidence value when API is not in use | 5567 | pending | — | — | — |
| detects gomod updates and uses updateType=digest when appropriate | 5588 | pending | — | — | — |
| handles changelog with content | 5629 | pending | — | — | — |

---
