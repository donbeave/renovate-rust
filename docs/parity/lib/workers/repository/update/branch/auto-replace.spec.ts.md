# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/auto-replace.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/auto-replace.spec.ts
**Total tests:** 70 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/update/branch/auto-replace › doAutoReplace`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rebases if the deps list has changed | 36 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| rebases if the deps to update has changed | 47 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| uses depName or packageName | 56 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates version only | 78 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| handles a double attempt | 93 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| handles already updated | 107 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| handles no work | 128 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| returns existing content if replaceString mismatch | 144 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates version and integrity | 163 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with autoReplaceNewString | 182 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| succeeds when using autoReplaceStringTemplate to update depName when using regex | 204 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| fails with oldversion in depName | 244 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| fails with digest mismatch | 266 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with docker replacement | 284 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| handles already replaced | 298 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| handles replacement with depName===newName when replaceString exists | 309 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with terraform replacement | 333 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with ansible replacement | 350 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with ansible-galaxy roles replacement | 376 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with azure-pipeline image replacement | 398 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with batect image replacement | 422 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with bitbucket-pipelines image replacement | 445 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with buildkite plugin replacement | 464 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with bundler gem replacement | 487 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with cake #addin replacement | 509 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with cargo dependency replacement | 528 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with cloudbuild replacement | 549 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with podfile pod replacement | 572 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with composer require replacement | 590 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with edn deps replacement | 614 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with docker-compose image replacement | 637 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with Dockerfile image replacement | 660 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with Dockerfile image replacement with digest | 679 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with droneci image replacement | 701 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with gitlabci image replacement | 724 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with helm value image/repository replacement | 743 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with helm value image/repository replacement with digest | 767 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with helm value image/repository wrong version | 794 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with helm value image/repository prefix replacement | 816 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with helm value image/repository version prefix replacement | 840 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with jenkins plugin replacement | 864 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with meteor npm.depends replacement | 882 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| checks for replaceWithoutReplaceString double update | 908 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with mix deps replacement | 942 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with nuget tools replacement | 983 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with pre-commit repo replacement | 1011 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with terraform image replacement | 1033 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with terraform module replacement | 1056 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with setup-cfg replacement | 1079 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with nvm version replacement | 1100 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with multiple same name replacement without replaceString | 1118 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with multiple same name replacement without replaceString 2 | 1142 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with multiple same version replacement without replaceString | 1166 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates with multiple same digest replacement without replaceString | 1190 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| docker: updates with pinDigest enabled but no currentDigest value | 1216 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| docker: updates with pinDigest enabled and a currentDigest value | 1240 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| autoReplaceGlobalMatch: throws error when globally replacing recurring values across version and digests | 1264 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| autoReplaceGlobalMatch: updates when replacing first match only of recurring values across version and digests | 1284 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| regex: updates with pinDigest enabled but no currentDigest value | 1309 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| regex: updates with pinDigest enabled and a currentDigest value | 1333 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| jsonata: update currentValue | 1361 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| jsonata: update currentDigest | 1385 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| jsonata: update currentValue and currentDigest | 1408 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| github-actions: updates with newValue only | 1433 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| github-actions: updates with newValue and newDigest | 1470 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| github-actions: updates with pinDigest enabled but no currentDigest value | 1507 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| github-actions: updates with pinDigest enabled and a currentDigest value | 1546 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| github-actions: failes to update currentDigestShort | 1584 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| docker: replacement with same digest should not corrupt digest via currentDigestShort | 1622 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |
| updates only digest | 1653 | not-applicable | — | — | tests doAutoReplace file content rewriting via fs mocks and manager extractors; depends on Node.js fs infrastructure |

---
