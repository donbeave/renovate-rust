# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/composer/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/composer/artifacts.spec.ts
**Total tests:** 30 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no composer.lock found | 69 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if unchanged | 80 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| uses hostRules to set COMPOSER_AUTH | 114 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| git-tags hostRule for github.com set github-token in COMPOSER_AUTH | 191 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| Skip github application access token hostRules in COMPOSER_AUTH | 225 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| github hostRule for github.com with x-access-token set github-token in COMPOSER_AUTH | 263 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does set github COMPOSER_AUTH for github when only hostType git-tags artifactAuth does not include composer | 297 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does set github COMPOSER_AUTH for git-tags when only hostType github artifactAuth does not include composer | 336 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does not set github COMPOSER_AUTH when artifactAuth does not include composer, for both hostType github & git-tags | 375 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does not set gitlab COMPOSER_AUTH when artifactAuth does not include composer | 407 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does not set packagist COMPOSER_AUTH when artifactAuth does not include composer | 448 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does set gitlab COMPOSER_AUTH when artifactAuth does include composer | 509 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does set packagist COMPOSER_AUTH when artifactAuth does include composer | 553 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated composer.lock | 620 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports vendor directory update | 652 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| performs lockFileMaintenance | 713 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode | 749 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports install mode | 823 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports global mode | 881 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| catches errors | 914 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| catches unmet requirements errors | 938 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| throws for disk space | 957 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| disables ignorePlatformReqs | 976 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| adds all ignorePlatformReq items | 1011 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| installs before running the update when symfony flex is installed | 1046 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| installs before running the update when symfony flex is installed as dev | 1095 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| does not disable plugins when configured globally | 1144 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| disable plugins when configured locally | 1169 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| includes new dependency version in update command | 1194 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| uses --with-all-dependencies instead of --with-dependencies when composerUpdateAllDependencies is set in postUpdateOptions | 1216 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---

