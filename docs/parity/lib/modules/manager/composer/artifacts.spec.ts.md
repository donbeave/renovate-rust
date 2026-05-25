# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/composer/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/composer/artifacts.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 30 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no composer.lock found | 69 | pending | — | — | — |
| returns null if unchanged | 80 | pending | — | — | — |
| uses hostRules to set COMPOSER_AUTH | 114 | pending | — | — | — |
| git-tags hostRule for github.com set github-token in COMPOSER_AUTH | 191 | pending | — | — | — |
| Skip github application access token hostRules in COMPOSER_AUTH | 225 | pending | — | — | — |
| github hostRule for github.com with x-access-token set github-token in COMPOSER_AUTH | 263 | pending | — | — | — |
| does set github COMPOSER_AUTH for github when only hostType git-tags artifactAuth does not include composer | 297 | pending | — | — | — |
| does set github COMPOSER_AUTH for git-tags when only hostType github artifactAuth does not include composer | 336 | pending | — | — | — |
| does not set github COMPOSER_AUTH when artifactAuth does not include composer, for both hostType github & git-tags | 375 | pending | — | — | — |
| does not set gitlab COMPOSER_AUTH when artifactAuth does not include composer | 407 | pending | — | — | — |
| does not set packagist COMPOSER_AUTH when artifactAuth does not include composer | 448 | pending | — | — | — |
| does set gitlab COMPOSER_AUTH when artifactAuth does include composer | 509 | pending | — | — | — |
| does set packagist COMPOSER_AUTH when artifactAuth does include composer | 553 | pending | — | — | — |
| returns updated composer.lock | 620 | pending | — | — | — |
| supports vendor directory update | 652 | pending | — | — | — |
| performs lockFileMaintenance | 713 | pending | — | — | — |
| supports docker mode | 749 | pending | — | — | — |
| supports install mode | 823 | pending | — | — | — |
| supports global mode | 881 | pending | — | — | — |
| catches errors | 914 | pending | — | — | — |
| catches unmet requirements errors | 938 | pending | — | — | — |
| throws for disk space | 957 | pending | — | — | — |
| disables ignorePlatformReqs | 976 | pending | — | — | — |
| adds all ignorePlatformReq items | 1011 | pending | — | — | — |
| installs before running the update when symfony flex is installed | 1046 | pending | — | — | — |
| installs before running the update when symfony flex is installed as dev | 1095 | pending | — | — | — |
| does not disable plugins when configured globally | 1144 | pending | — | — | — |
| disable plugins when configured locally | 1169 | pending | — | — | — |
| includes new dependency version in update command | 1194 | pending | — | — | — |
| uses --with-all-dependencies instead of --with-dependencies when composerUpdateAllDependencies is set in postUpdateOptions | 1216 | pending | — | — | — |

---

