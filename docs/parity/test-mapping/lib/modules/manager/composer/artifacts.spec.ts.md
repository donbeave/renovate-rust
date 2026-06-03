# `lib/modules/manager/composer/artifacts.spec.ts`

[← `manager/composer`](../../../../_by-module/manager/composer.md) · [all modules](../../../../README.md)

**0/30 in-scope tests ported** (30 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 70 | returns if no composer.lock found | pending | — |
| 81 | returns null if unchanged | pending | — |
| 115 | uses hostrules to set composer_auth | pending | — |
| 192 | git-tags hostrule for github.com set github-token in composer_auth | pending | — |
| 226 | skip github application access token hostrules in composer_auth | pending | — |
| 264 | github hostrule for github.com with x-access-token set github-token in composer_auth | pending | — |
| 298 | does set github composer_auth for github when only hosttype git-tags artifactauth does not include composer | pending | — |
| 337 | does set github composer_auth for git-tags when only hosttype github artifactauth does not include composer | pending | — |
| 376 | does not set github composer_auth when artifactauth does not include composer, for both hosttype github & git-tags | pending | — |
| 408 | does not set gitlab composer_auth when artifactauth does not include composer | pending | — |
| 449 | does not set packagist composer_auth when artifactauth does not include composer | pending | — |
| 510 | does set gitlab composer_auth when artifactauth does include composer | pending | — |
| 554 | does set packagist composer_auth when artifactauth does include composer | pending | — |
| 621 | returns updated composer.lock | pending | — |
| 653 | supports vendor directory update | pending | — |
| 714 | performs lockfilemaintenance | pending | — |
| 750 | supports docker mode | pending | — |
| 824 | supports install mode | pending | — |
| 882 | supports global mode | pending | — |
| 915 | catches errors | pending | — |
| 939 | catches unmet requirements errors | pending | — |
| 958 | throws for disk space | pending | — |
| 977 | disables ignoreplatformreqs | pending | — |
| 1012 | adds all ignoreplatformreq items | pending | — |
| 1047 | installs before running the update when symfony flex is installed | pending | — |
| 1096 | installs before running the update when symfony flex is installed as dev | pending | — |
| 1145 | does not disable plugins when configured globally | pending | — |
| 1170 | disable plugins when configured locally | pending | — |
| 1195 | includes new dependency version in update command | pending | — |
| 1217 | uses --with-all-dependencies instead of --with-dependencies when composerupdatealldependencies is set in postupdateoptions | pending | — |

