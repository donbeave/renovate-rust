# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/auto-replace.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/auto-replace.spec.ts
**Total tests:** 70 | **Ported:** 0 | **Actionable:** 70 | **Status:** pending

### `workers/repository/update/branch/auto-replace › doAutoReplace`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rebases if the deps list has changed | 36 | pending | — | — | — |
| rebases if the deps to update has changed | 47 | pending | — | — | — |
| uses depName or packageName | 56 | pending | — | — | — |
| updates version only | 78 | pending | — | — | — |
| handles a double attempt | 93 | pending | — | — | — |
| handles already updated | 107 | pending | — | — | — |
| handles no work | 128 | pending | — | — | — |
| returns existing content if replaceString mismatch | 144 | pending | — | — | — |
| updates version and integrity | 163 | pending | — | — | — |
| updates with autoReplaceNewString | 182 | pending | — | — | — |
| succeeds when using autoReplaceStringTemplate to update depName when using regex | 204 | pending | — | — | — |
| fails with oldversion in depName | 244 | pending | — | — | — |
| fails with digest mismatch | 266 | pending | — | — | — |
| updates with docker replacement | 284 | pending | — | — | — |
| handles already replaced | 298 | pending | — | — | — |
| handles replacement with depName===newName when replaceString exists | 309 | pending | — | — | — |
| updates with terraform replacement | 333 | pending | — | — | — |
| updates with ansible replacement | 350 | pending | — | — | — |
| updates with ansible-galaxy roles replacement | 376 | pending | — | — | — |
| updates with azure-pipeline image replacement | 398 | pending | — | — | — |
| updates with batect image replacement | 422 | pending | — | — | — |
| updates with bitbucket-pipelines image replacement | 445 | pending | — | — | — |
| updates with buildkite plugin replacement | 464 | pending | — | — | — |
| updates with bundler gem replacement | 487 | pending | — | — | — |
| updates with cake #addin replacement | 509 | pending | — | — | — |
| updates with cargo dependency replacement | 528 | pending | — | — | — |
| updates with cloudbuild replacement | 549 | pending | — | — | — |
| updates with podfile pod replacement | 572 | pending | — | — | — |
| updates with composer require replacement | 590 | pending | — | — | — |
| updates with edn deps replacement | 614 | pending | — | — | — |
| updates with docker-compose image replacement | 637 | pending | — | — | — |
| updates with Dockerfile image replacement | 660 | pending | — | — | — |
| updates with Dockerfile image replacement with digest | 679 | pending | — | — | — |
| updates with droneci image replacement | 701 | pending | — | — | — |
| updates with gitlabci image replacement | 724 | pending | — | — | — |
| updates with helm value image/repository replacement | 743 | pending | — | — | — |
| updates with helm value image/repository replacement with digest | 767 | pending | — | — | — |
| updates with helm value image/repository wrong version | 794 | pending | — | — | — |
| updates with helm value image/repository prefix replacement | 816 | pending | — | — | — |
| updates with helm value image/repository version prefix replacement | 840 | pending | — | — | — |
| updates with jenkins plugin replacement | 864 | pending | — | — | — |
| updates with meteor npm.depends replacement | 882 | pending | — | — | — |
| checks for replaceWithoutReplaceString double update | 908 | pending | — | — | — |
| updates with mix deps replacement | 942 | pending | — | — | — |
| updates with nuget tools replacement | 983 | pending | — | — | — |
| updates with pre-commit repo replacement | 1011 | pending | — | — | — |
| updates with terraform image replacement | 1033 | pending | — | — | — |
| updates with terraform module replacement | 1056 | pending | — | — | — |
| updates with setup-cfg replacement | 1079 | pending | — | — | — |
| updates with nvm version replacement | 1100 | pending | — | — | — |
| updates with multiple same name replacement without replaceString | 1118 | pending | — | — | — |
| updates with multiple same name replacement without replaceString 2 | 1142 | pending | — | — | — |
| updates with multiple same version replacement without replaceString | 1166 | pending | — | — | — |
| updates with multiple same digest replacement without replaceString | 1190 | pending | — | — | — |
| docker: updates with pinDigest enabled but no currentDigest value | 1216 | pending | — | — | — |
| docker: updates with pinDigest enabled and a currentDigest value | 1240 | pending | — | — | — |
| autoReplaceGlobalMatch: throws error when globally replacing recurring values across version and digests | 1264 | pending | — | — | — |
| autoReplaceGlobalMatch: updates when replacing first match only of recurring values across version and digests | 1284 | pending | — | — | — |
| regex: updates with pinDigest enabled but no currentDigest value | 1309 | pending | — | — | — |
| regex: updates with pinDigest enabled and a currentDigest value | 1333 | pending | — | — | — |
| jsonata: update currentValue | 1361 | pending | — | — | — |
| jsonata: update currentDigest | 1385 | pending | — | — | — |
| jsonata: update currentValue and currentDigest | 1408 | pending | — | — | — |
| github-actions: updates with newValue only | 1433 | pending | — | — | — |
| github-actions: updates with newValue and newDigest | 1470 | pending | — | — | — |
| github-actions: updates with pinDigest enabled but no currentDigest value | 1507 | pending | — | — | — |
| github-actions: updates with pinDigest enabled and a currentDigest value | 1546 | pending | — | — | — |
| github-actions: failes to update currentDigestShort | 1584 | pending | — | — | — |
| docker: replacement with same digest should not corrupt digest via currentDigestShort | 1622 | pending | — | — | — |
| updates only digest | 1653 | pending | — | — | — |

---
