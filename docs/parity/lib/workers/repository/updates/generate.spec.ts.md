# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/updates/generate.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/updates/generate.spec.ts
**Total tests:** 55 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/updates/generate › generateBranchConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not group single upgrade | 31 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| handles lockFileMaintenance | 52 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| sets minimumGroupSize based on upgrades | 77 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| handles lockFileUpdate | 106 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| does not group same upgrades | 147 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| groups multiple upgrades same version | 175 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| groups major updates with different versions but same newValue, no recreateWhen | 246 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| groups multiple digest updates immortally | 280 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| recreates grouped pin & pinDigest | 310 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| does not recreate grouped pin & pinDigest when closed if recreateWhen=never | 332 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| recreates grouped pin | 356 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| recreates grouped pinDigest | 383 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| skips appending baseBranch and updateType to prTitle when prTitleStrict is true | 409 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| groups multiple upgrades different version | 462 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| groups multiple upgrades different version but same value | 513 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| groups multiple upgrades different value but same version | 553 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| groups multiple digest updates | 593 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| pins digest to table | 629 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| fixes different messages | 647 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| uses semantic commits | 684 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| calculates the highest priority semanticCommitType | 711 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| scopes monorepo commits | 759 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| scopes monorepo commits with nested package files using parent directory | 786 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| scopes monorepo commits with nested package files using base directory | 816 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| use prettyVersion in pr title when there is a v | 845 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| use prettyVersion in pr title there is no v | 872 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| use newMajor in pr title with v | 899 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| Default commitMessageExtra pr title | 924 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| adds commit message body | 950 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| supports manual prTitle | 968 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| handles @types specially | 984 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| handles @types specially (reversed) | 1049 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| handles upgrades | 1110 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| combines prBodyColumns | 1257 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| sorts upgrades, without position first | 1274 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| passes through pendingChecks | 1315 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| filters pendingChecks | 1339 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| displays pending versions | 1362 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| merge excludeCommitPaths if appears in upgrade | 1396 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| generates pretty version name properly | 1429 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| prevents issue with duplicating "v" character | 1453 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| apply semanticCommits and commitMessagePrefix together | 1466 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| dedupes duplicate table rows | 1486 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| using commitMessagePrefix without separator | 1549 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| merges additionalReviewers | 1566 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| merges depTypes | 1590 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| depTypes is available on each branch upgrade object | 1615 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| allows upgrades in commitMessage | 1645 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| allows upgrades in commitMessage (group) | 1670 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| sets skipArtifactsUpdate to false when no upgrades specify a value | 1725 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| sets skipArtifactsUpdate to true when all upgrades specify true | 1773 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| sets skipArtifactsUpdate to false when not all upgrades specify true and first is $0 | 1824 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| uses prettyDepType when already set | 1888 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| falls back to depType when prettyDepType is not set | 1902 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |
| defaults prettyDepType to dependency when neither prettyDepType nor depType is set | 1915 | not-applicable | — | — | tests generateBranchConfig commit message generation using Handlebars template engine; needs template infrastructure |

---
