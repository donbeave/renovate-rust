# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/process/lookup/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/lookup/index.spec.ts
**Total tests:** 169 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/process/lookup/index › .lookupUpdates()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if invalid currentValue | 100 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| returns null if unknown datasource | 111 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles error result from getPkgReleasesWithResult | 122 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| returns rollback for pinned version | 134 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| returns rollback for ranged version | 173 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| supports minor and major upgrades for tilde ranges | 199 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| supports lock file updates mixed with regular updates | 249 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| returns multiple updates if grouping but separateMajorMinor=true | 309 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| returns additional update if grouping but separateMinorPatch=true | 351 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| returns one update if grouping and separateMajorMinor=false | 407 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| returns both updates if automerging minor | 437 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| enforces allowedVersions | 488 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| enforces allowedVersions with regex | 516 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| enforces allowedVersions with negative regex | 544 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| falls back to semver syntax allowedVersions | 572 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| falls back to pep440 syntax allowedVersions | 601 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| skips invalid allowedVersions | 630 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| returns patch update even if separate patches not configured | 642 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| returns minor update if automerging both patch and minor | 683 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| returns patch update if separateMinorPatch | 730 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| returns patch minor and major | 772 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| disables major release separation (major) | 827 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| disables major release separation (minor) | 864 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| uses minimum version for vulnerabilityAlerts | 893 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| uses highest available version for vulnerabilityAlerts when vulnerabilityFixStrategy=highest | 921 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| uses vulnerabilityFixVersion when a version | 950 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| takes a later release when vulnerabilityFixVersion does not exist | 979 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| uses vulnerabilityFixVersion when a range | 1008 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| takes highest available version when using vulnerabilityFixStrategy=highest with vulnerabilityFixVersion | 1037 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| ignores vulnerabilityFixVersion if not a version | 1067 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| returns no results if vulnerabilityFixVersion is too high | 1096 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| supports minor and major upgrades for ranged versions | 1111 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| supports for x-range-all for replaceStrategy = pin (with lockfile) abcd | 1161 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| doesnt offer updates for x-range-all (with lockfile) when replaceStrategy = $strategy | 1184 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| supports pinning for x-range-all (no lockfile) | 1207 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| covers pinning an unsupported x-range-all value | 1229 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| doesnt offer updates for x-range-all (no lockfile) when replaceStrategy = $strategy | 1243 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| ignores pinning for ranges when other upgrade exists | 1266 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades minor ranged versions | 1302 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles update-lockfile | 1338 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles the in-range-only strategy and updates lockfile within range | 1369 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles the in-range-only strategy and discards changes not within range | 1400 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles unconstrainedValue values | 1431 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles unconstrainedValue values with rangeStrategy !== update-lockfile and isVulnerabilityAlert | 1461 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| widens minor ranged versions if configured | 1491 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| replaces minor complex ranged versions if configured | 1520 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| widens major ranged versions if configured | 1549 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| replaces major complex ranged versions if configured | 1581 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| pins minor ranged versions | 1613 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| uses the locked version for pinning | 1635 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| ignores minor ranged versions when not pinning | 1658 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| ignores minor ranged versions when locked | 1672 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades tilde ranges | 1687 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades .x minor ranges | 1723 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades tilde ranges without pinning | 1759 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades .x major ranges without pinning | 1788 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades .x minor ranges without pinning | 1817 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades .x complex minor ranges without pinning | 1846 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades shorthand major ranges without pinning | 1875 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades shorthand minor ranges without pinning | 1904 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades multiple tilde ranges without pinning | 1933 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades multiple caret ranges without pinning | 1976 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| supports complex ranges | 2019 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| supports complex major ranges | 2062 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| supports complex major hyphen ranges | 2094 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| widens .x OR ranges | 2126 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| widens stanndalone major OR ranges | 2158 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| supports complex tilde ranges | 2190 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| returns nothing for greater than ranges | 2219 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades less than equal ranges without pinning | 2233 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades less than ranges without pinning | 2276 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades less than major ranges | 2319 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades less than equal minor ranges | 2348 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades equal minor ranges | 2377 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades less than equal major ranges | 2406 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades major less than equal ranges | 2436 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades major less than ranges without pinning | 2465 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades major greater than less than ranges without pinning | 2494 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades minor greater than less than ranges without pinning | 2523 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| upgrades minor greater than less than equals ranges without pinning | 2566 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| rejects reverse ordered less than greater than | 2609 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| supports > latest versions if configured | 2623 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should ignore unstable versions if the current version is stable | 2651 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should ignore unstable versions from datasource | 2664 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should allow unstable versions in same major for node | 2695 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should return pendingChecks | 2727 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should return pendingVersions | 2773 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should allow unstable versions if the ignoreUnstable=false | 2819 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should allow unstable versions if the current version is unstable | 2848 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should not jump unstable versions | 2878 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should update pinned versions if updatePinnedDependencies=true | 2908 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should not update pinned versions if updatePinnedDependencies=false | 2939 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should follow dist-tag even if newer version exists | 2956 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should roll back to dist-tag if current version is higher | 2987 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should jump unstable versions if followTag | 3019 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should update nothing if current version is dist-tag | 3050 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should warn if no version matches dist-tag | 3067 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should warn if no digest could be found but there is a current digest | 3090 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |

### `workers/repository/process/lookup/index › .lookupUpdates() › pinning enabled but no existing digest`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should not warn if no new digest could be found | 3125 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should use registry of update to determine digest | 3154 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should treat zero zero tilde ranges as 0.0.x | 3199 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should treat zero zero caret ranges as pinned | 3216 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should downgrade from missing versions | 3248 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should upgrade to only one major | 3279 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should upgrade to two majors | 3322 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| should upgrade to 16 minors | 3380 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| does not jump  major unstable | 3395 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| supports in-range caret updates | 3409 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| supports in-range tilde updates | 3439 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| supports in-range tilde patch updates | 3484 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| supports in-range gte updates | 3529 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| supports majorgte updates | 3559 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| rejects in-range unsupported operator | 3590 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| rejects non-fully specified in-range updates | 3604 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| rejects complex range in-range updates | 3618 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| replaces non-range in-range updates | 3632 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles github 404 | 3661 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles pypi 404 | 3675 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles packagist | 3692 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles unknown datasource | 3712 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles PEP440 | 3725 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| returns complex object | 3767 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| prefers lockedVersion | 3803 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| ignores deprecated when it is not the latest | 3820 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| treats all versions as deprecated if latest is deprecated | 3873 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| skips unsupported values | 3925 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| skips undefined values | 3942 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles digest pin | 3958 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| skips uncompatible versions for 8.1.0 | 4010 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| skips uncompatible versions for 8.1 | 4056 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| skips uncompatible versions for 8 | 4114 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| applies versionCompatibility for 18.10.0 | 4160 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| applies versionCompatibility for maven | 4232 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles versionCompatibility mismatch | 4272 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| applies versionCompatibility for debian codenames with suffix | 4299 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles digest pin for up to date version | 4340 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles no fitting version and no version in lock file | 4379 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles digest pin for non-version | 4408 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles digest lookup failure | 4446 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles digest update | 4473 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles digest update for custom datasource | 4525 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles digest update for non-version | 4552 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles git submodule update | 4590 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles sourceUrl packageRules with version restrictions | 4613 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles current age packageRules with version restrictions | 4655 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| does not apply package rules for matchCurrentAge if packageRules doesn not have a current age matcher | 4703 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| does not apply package rules for matchCurrentAge if the releaseTimestamp for current version is missing | 4762 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - name only without pinDigests enabled | 4815 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - name only with pinDigests enabled | 4857 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - name only no version/tag | 4912 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - Digest configured and validating getDigest funtion call | 4934 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - Digest configured with replacementNameTemplate and validating getDigest function call | 4994 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - skips if package and replacement names match | 5083 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - name and version | 5096 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - can template replacement name without a replacement version | 5117 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - can template replacement name with a replacement version | 5156 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - replacementName takes precedence over replacementNameTemplate | 5196 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - can template replacement version without a replacement name | 5236 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - can template replacement version with a replacement name | 5275 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - can template replacement version with a template replacement name | 5315 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - replacementVersion takes precedence over replacementVersionTemplate | 5355 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - can perform replacement even for invalid versioning | 5395 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles replacements - from datasource | 5422 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| rollback for invalid version to last stable version | 5445 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |

### `workers/repository/process/lookup/index › .lookupUpdates() › handles merge confidence`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets a merge confidence level for a given update when corresponding packageRule is in use | 5493 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| does not get a merge confidence level when no packageRule is set | 5536 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| does not set merge confidence value when API is not in use | 5567 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| detects gomod updates and uses updateType=digest when appropriate | 5588 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |
| handles changelog with content | 5629 | not-applicable | — | — | tests version lookup pipeline via httpMock datasource calls; Rust has no equivalent datasource HTTP layer yet |

---
