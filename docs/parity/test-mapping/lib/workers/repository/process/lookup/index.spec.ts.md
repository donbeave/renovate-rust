# `lib/workers/repository/process/lookup/index.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**0/171 in-scope tests ported** (171 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 101 | returns null if invalid currentvalue | pending | — |
| 112 | returns null if unknown datasource | pending | — |
| 123 | handles error result from getpkgreleaseswithresult | pending | — |
| 135 | returns rollback for pinned version | pending | — |
| 174 | returns rollback for ranged version | pending | — |
| 200 | supports minor and major upgrades for tilde ranges | pending | — |
| 250 | supports lock file updates mixed with regular updates | pending | — |
| 310 | returns multiple updates if grouping but separatemajorminor=true | pending | — |
| 352 | returns additional update if grouping but separateminorpatch=true | pending | — |
| 408 | returns one update if grouping and separatemajorminor=false | pending | — |
| 438 | returns both updates if automerging minor | pending | — |
| 489 | enforces allowedversions | pending | — |
| 517 | enforces allowedversions with regex | pending | — |
| 545 | enforces allowedversions with negative regex | pending | — |
| 573 | falls back to semver syntax allowedversions | pending | — |
| 602 | falls back to pep440 syntax allowedversions | pending | — |
| 631 | skips invalid allowedversions | pending | — |
| 643 | returns patch update even if separate patches not configured | pending | — |
| 684 | returns minor update if automerging both patch and minor | pending | — |
| 731 | returns patch update if separateminorpatch | pending | — |
| 773 | returns patch minor and major | pending | — |
| 828 | disables major release separation (major) | pending | — |
| 865 | disables major release separation (minor) | pending | — |
| 894 | uses minimum version for vulnerabilityalerts | pending | — |
| 922 | uses highest available version for vulnerabilityalerts when vulnerabilityfixstrategy=highest | pending | — |
| 951 | uses vulnerabilityfixversion when a version | pending | — |
| 980 | takes a later release when vulnerabilityfixversion does not exist | pending | — |
| 1009 | uses vulnerabilityfixversion when a range | pending | — |
| 1038 | takes highest available version when using vulnerabilityfixstrategy=highest with vulnerabilityfixversion | pending | — |
| 1068 | ignores vulnerabilityfixversion if not a version | pending | — |
| 1097 | returns no results if vulnerabilityfixversion is too high | pending | — |
| 1112 | supports minor and major upgrades for ranged versions | pending | — |
| 1162 | supports for x-range-all for replacestrategy = pin (with lockfile) abcd | pending | — |
| 1185 | _(it.each / template — verify manually)_ | ? | — |
| 1208 | supports pinning for x-range-all (no lockfile) | pending | — |
| 1230 | covers pinning an unsupported x-range-all value | pending | — |
| 1244 | _(it.each / template — verify manually)_ | ? | — |
| 1267 | ignores pinning for ranges when other upgrade exists | pending | — |
| 1303 | upgrades minor ranged versions | pending | — |
| 1339 | handles update-lockfile | pending | — |
| 1370 | handles the in-range-only strategy and updates lockfile within range | pending | — |
| 1401 | handles the in-range-only strategy and discards changes not within range | pending | — |
| 1432 | handles unconstrainedvalue values | pending | — |
| 1462 | handles unconstrainedvalue values with rangestrategy !== update-lockfile and isvulnerabilityalert | pending | — |
| 1492 | widens minor ranged versions if configured | pending | — |
| 1521 | replaces minor complex ranged versions if configured | pending | — |
| 1550 | widens major ranged versions if configured | pending | — |
| 1582 | replaces major complex ranged versions if configured | pending | — |
| 1614 | pins minor ranged versions | pending | — |
| 1636 | uses the locked version for pinning | pending | — |
| 1659 | ignores minor ranged versions when not pinning | pending | — |
| 1673 | ignores minor ranged versions when locked | pending | — |
| 1688 | upgrades tilde ranges | pending | — |
| 1724 | upgrades .x minor ranges | pending | — |
| 1760 | upgrades tilde ranges without pinning | pending | — |
| 1789 | upgrades .x major ranges without pinning | pending | — |
| 1818 | upgrades .x minor ranges without pinning | pending | — |
| 1847 | upgrades .x complex minor ranges without pinning | pending | — |
| 1876 | upgrades shorthand major ranges without pinning | pending | — |
| 1905 | upgrades shorthand minor ranges without pinning | pending | — |
| 1934 | upgrades multiple tilde ranges without pinning | pending | — |
| 1977 | upgrades multiple caret ranges without pinning | pending | — |
| 2020 | supports complex ranges | pending | — |
| 2063 | supports complex major ranges | pending | — |
| 2095 | supports complex major hyphen ranges | pending | — |
| 2127 | widens .x or ranges | pending | — |
| 2159 | widens stanndalone major or ranges | pending | — |
| 2191 | supports complex tilde ranges | pending | — |
| 2220 | returns nothing for greater than ranges | pending | — |
| 2234 | upgrades less than equal ranges without pinning | pending | — |
| 2277 | upgrades less than ranges without pinning | pending | — |
| 2320 | upgrades less than major ranges | pending | — |
| 2349 | upgrades less than equal minor ranges | pending | — |
| 2378 | upgrades equal minor ranges | pending | — |
| 2407 | upgrades less than equal major ranges | pending | — |
| 2437 | upgrades major less than equal ranges | pending | — |
| 2466 | upgrades major less than ranges without pinning | pending | — |
| 2495 | upgrades major greater than less than ranges without pinning | pending | — |
| 2524 | upgrades minor greater than less than ranges without pinning | pending | — |
| 2567 | upgrades minor greater than less than equals ranges without pinning | pending | — |
| 2610 | rejects reverse ordered less than greater than | pending | — |
| 2624 | supports > latest versions if configured | pending | — |
| 2652 | should ignore unstable versions if the current version is stable | pending | — |
| 2665 | should ignore unstable versions from datasource | pending | — |
| 2696 | should allow unstable versions in same major for node | pending | — |
| 2728 | should return pendingchecks | pending | — |
| 2774 | should return pendingversions | pending | — |
| 2820 | should allow unstable versions if the ignoreunstable=false | pending | — |
| 2849 | should allow unstable versions if the current version is unstable | pending | — |
| 2879 | should not jump unstable versions | pending | — |
| 2909 | should update pinned versions if updatepinneddependencies=true | pending | — |
| 2940 | should not update pinned versions if updatepinneddependencies=false | pending | — |
| 2957 | should follow dist-tag even if newer version exists | pending | — |
| 2988 | should roll back to dist-tag if current version is higher | pending | — |
| 3020 | should jump unstable versions if followtag | pending | — |
| 3051 | should update nothing if current version is dist-tag | pending | — |
| 3068 | should warn if no version matches dist-tag | pending | — |
| 3091 | should warn if no digest could be found but there is a current digest | pending | — |
| 3126 | should not warn if no new digest could be found | pending | — |
| 3155 | should use registry of update to determine digest | pending | — |
| 3200 | should treat zero zero tilde ranges as 0.0.x | pending | — |
| 3217 | should treat zero zero caret ranges as pinned | pending | — |
| 3249 | should downgrade from missing versions | pending | — |
| 3280 | should upgrade to only one major | pending | — |
| 3323 | should upgrade to two majors | pending | — |
| 3381 | should upgrade to 16 minors | pending | — |
| 3396 | does not jump major unstable | pending | — |
| 3410 | supports in-range caret updates | pending | — |
| 3440 | supports in-range tilde updates | pending | — |
| 3485 | supports in-range tilde patch updates | pending | — |
| 3530 | supports in-range gte updates | pending | — |
| 3560 | supports majorgte updates | pending | — |
| 3591 | rejects in-range unsupported operator | pending | — |
| 3605 | rejects non-fully specified in-range updates | pending | — |
| 3619 | rejects complex range in-range updates | pending | — |
| 3633 | replaces non-range in-range updates | pending | — |
| 3662 | handles github 404 | pending | — |
| 3676 | handles pypi 404 | pending | — |
| 3693 | handles packagist | pending | — |
| 3713 | handles unknown datasource | pending | — |
| 3726 | handles pep440 | pending | — |
| 3768 | returns complex object | pending | — |
| 3804 | prefers lockedversion | pending | — |
| 3821 | ignores deprecated when it is not the latest | pending | — |
| 3874 | treats all versions as deprecated if latest is deprecated | pending | — |
| 3926 | skips unsupported values | pending | — |
| 3943 | skips undefined values | pending | — |
| 3959 | handles digest pin | pending | — |
| 4011 | skips uncompatible versions for 8.1.0 | pending | — |
| 4057 | skips uncompatible versions for 8.1 | pending | — |
| 4115 | skips uncompatible versions for 8 | pending | — |
| 4161 | applies versioncompatibility for 18.10.0 | pending | — |
| 4233 | applies versioncompatibility for maven | pending | — |
| 4273 | handles versioncompatibility mismatch | pending | — |
| 4300 | applies versioncompatibility for debian codenames with suffix | pending | — |
| 4341 | handles digest pin for up to date version | pending | — |
| 4380 | handles pin for github actions | pending | — |
| 4411 | handles no fitting version and no version in lock file | pending | — |
| 4440 | handles digest pin for non-version | pending | — |
| 4478 | handles digest lookup failure | pending | — |
| 4505 | handles digest update | pending | — |
| 4557 | handles digest update for custom datasource | pending | — |
| 4584 | handles digest update for non-version | pending | — |
| 4622 | handles git submodule update | pending | — |
| 4645 | handles sourceurl packagerules with version restrictions | pending | — |
| 4687 | handles current age packagerules with version restrictions | pending | — |
| 4735 | does not apply package rules for matchcurrentage if packagerules doesn not have a current age matcher | pending | — |
| 4794 | does not apply package rules for matchcurrentage if the releasetimestamp for current version is missing | pending | — |
| 4847 | handles replacements - name only without pindigests enabled | pending | — |
| 4889 | handles replacements - name only with pindigests enabled | pending | — |
| 4944 | handles replacements - name only no version/tag | pending | — |
| 4966 | handles replacements - digest configured and validating getdigest funtion call | pending | — |
| 5026 | handles replacements - digest configured with replacementnametemplate and validating getdigest function call | pending | — |
| 5115 | handles replacements - skips if package and replacement names match | pending | — |
| 5128 | handles replacements - name and version | pending | — |
| 5149 | handles replacements - can template replacement name without a replacement version | pending | — |
| 5188 | handles replacements - can template replacement name with a replacement version | pending | — |
| 5228 | handles replacements - replacementname takes precedence over replacementnametemplate | pending | — |
| 5268 | handles replacements - can template replacement version without a replacement name | pending | — |
| 5307 | handles replacements - can template replacement version with a replacement name | pending | — |
| 5347 | handles replacements - can template replacement version with a template replacement name | pending | — |
| 5387 | handles replacements - replacementversion takes precedence over replacementversiontemplate | pending | — |
| 5427 | handles replacements - can perform replacement even for invalid versioning | pending | — |
| 5454 | handles replacements - from datasource | pending | — |
| 5477 | rollback for invalid version to last stable version | pending | — |
| 5525 | gets a merge confidence level for a given update when corresponding packagerule is in use | pending | — |
| 5568 | does not get a merge confidence level when no packagerule is set | pending | — |
| 5599 | does not set merge confidence value when api is not in use | pending | — |
| 5620 | detects gomod updates and uses updatetype=digest when appropriate | pending | — |
| 5661 | handles changelog with content | pending | — |
| 5708 | handles changelog with content for ranges | pending | — |

