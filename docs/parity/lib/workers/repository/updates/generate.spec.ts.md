# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/updates/generate.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/updates/generate.spec.ts
**Total tests:** 55 | **Ported:** 0 | **Actionable:** 55 | **Status:** pending

### `workers/repository/updates/generate › generateBranchConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not group single upgrade  | 31 | pending | — | — | — |
| handles lockFileMaintenance  | 52 | pending | — | — | — |
| sets minimumGroupSize based on upgrades  | 77 | pending | — | — | No corresponding Rust source|
| handles lockFileUpdate  | 106 | pending | — | — | — |
| does not group same upgrades  | 147 | pending | — | — | — |
| groups multiple upgrades same version  | 175 | pending | — | — | — |
| groups major updates with different versions but same newValue, no recreateWhen  | 246 | pending | — | — | — |
| groups multiple digest updates immortally  | 280 | pending | — | — | — |
| recreates grouped pin & pinDigest  | 310 | pending | — | — | — |
| does not recreate grouped pin & pinDigest when closed if recreateWhen=never  | 332 | pending | — | — | — |
| recreates grouped pin  | 356 | pending | — | — | — |
| recreates grouped pinDigest  | 383 | pending | — | — | — |
| skips appending baseBranch and updateType to prTitle when prTitleStrict is true  | 409 | pending | — | — | — |
| groups multiple upgrades different version  | 462 | pending | — | — | — |
| groups multiple upgrades different version but same value  | 513 | pending | — | — | — |
| groups multiple upgrades different value but same version  | 553 | pending | — | — | — |
| groups multiple digest updates  | 593 | pending | — | — | — |
| pins digest to table  | 629 | pending | — | — | — |
| fixes different messages  | 647 | pending | — | — | — |
| uses semantic commits  | 684 | pending | — | — | — |
| calculates the highest priority semanticCommitType  | 711 | pending | — | — | — |
| scopes monorepo commits  | 759 | pending | — | — | — |
| scopes monorepo commits with nested package files using parent directory  | 786 | pending | — | — | — |
| scopes monorepo commits with nested package files using base directory  | 816 | pending | — | — | — |
| use prettyVersion in pr title when there is a v  | 845 | pending | — | — | — |
| use prettyVersion in pr title there is no v  | 872 | pending | — | — | — |
| use newMajor in pr title with v  | 899 | pending | — | — | — |
| Default commitMessageExtra pr title  | 924 | pending | — | — | — |
| adds commit message body  | 950 | pending | — | — | — |
| supports manual prTitle  | 968 | pending | — | — | — |
| handles @types specially  | 984 | pending | — | — | — |
| handles @types specially (reversed)  | 1049 | pending | — | — | — |
| handles upgrades  | 1110 | pending | — | — | — |
| combines prBodyColumns  | 1257 | pending | — | — | — |
| sorts upgrades, without position first  | 1274 | pending | — | — | — |
| passes through pendingChecks  | 1315 | pending | — | — | — |
| filters pendingChecks  | 1339 | pending | — | — | — |
| displays pending versions  | 1362 | pending | — | — | — |
| merge excludeCommitPaths if appears in upgrade  | 1396 | pending | — | — | — |
| generates pretty version name properly  | 1429 | pending | — | — | — |
| prevents issue with duplicating "v" character  | 1453 | pending | — | — | — |
| apply semanticCommits and commitMessagePrefix together  | 1466 | pending | — | — | — |
| dedupes duplicate table rows  | 1486 | pending | — | — | — |
| using commitMessagePrefix without separator  | 1549 | pending | — | — | — |
| merges additionalReviewers  | 1566 | pending | — | — | — |
| merges depTypes  | 1590 | pending | — | — | — |
| depTypes is available on each branch upgrade object  | 1615 | pending | — | — | — |
| allows upgrades in commitMessage  | 1645 | pending | — | — | — |
| allows upgrades in commitMessage (group)  | 1670 | pending | — | — | — |
| sets skipArtifactsUpdate to false when no upgrades specify a value  | 1725 | pending | — | — | — |
| sets skipArtifactsUpdate to true when all upgrades specify true  | 1773 | pending | — | — | — |
| sets skipArtifactsUpdate to false when not all upgrades specify true and first is $0  | 1824 | pending | — | — | No corresponding Rust source|
| uses prettyDepType when already set  | 1888 | pending | — | — | — |
| falls back to depType when prettyDepType is not set  | 1902 | pending | — | — | — |
| defaults prettyDepType to dependency when neither prettyDepType nor depType is set  | 1915 | pending | — | — | — |

---
