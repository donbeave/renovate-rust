# `lib/modules/manager/pip-compile/artifacts.spec.ts`

[← `manager/pip-compile`](../../../../_by-module/manager/pip-compile.md) · [all modules](../../../../README.md)

**0/34 ported** (34 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 85 | returns if no requirements.txt found | pending | — |
| 101 | returns null if all unchanged | pending | — |
| 122 | returns null if no config.lockfiles | pending | — |
| 143 | returns updated requirements.txt | pending | — |
| 170 | supports docker mode | pending | — |
| 224 | supports install mode | pending | — |
| 261 | installs python version according to the lock file | pending | — |
| 300 | installs python version according to the uv option | pending | — |
| 343 | install uv tools without constraints | pending | — |
| 384 | installs latest python version if no constraints and not in header | pending | — |
| 432 | catches errors | pending | — |
| 454 | returns updated requirements.txt when doing lockfile maintenance | pending | — |
| 477 | uses --upgrade-package only for islockfileupdate | pending | — |
| 505 | uses pip-compile version from config | pending | — |
| 566 | throws for garbage | pending | — |
| 577 | returns extracted common arguments (like those featured in the readme) | pending | — |
| 590 | returns extracted arguments for uv | pending | — |
| 600 | does not add --no-emit-index-url when pip_index_url has no credentials | pending | — |
| 609 | returns --no-emit-index-url when credentials are found in pip_index_url | pending | — |
| 618 | returns --no-emit-index-url when credentials are found in pip_extra_index_url | pending | — |
| 628 | returns --no-emit-index-url when only a username is found in pip_index_url | pending | — |
| 637 | returns --no-emit-index-url when only a username is found in pip_extra_index_url | pending | — |
| 646 | returns --no-emit-index-url when only a password is found in pip_index_url | pending | — |
| 655 | returns --no-emit-index-url when only a password is found in pip_extra_index_url | pending | — |
| 664 | returns --no-emit-index-url when pip_index_url is invalid | pending | — |
| 673 | returns --no-emit-index-url pip_extra_index_url is invalid | pending | — |
| 682 | returns --no-emit-index-url only once when its in the header and credentials are present in the environment | pending | — |
| 697 | allow explicit --emit-index-url | pending | — |
| 709 | throws on unknown arguments | pending | — |
| 720 | throws on custom command | pending | — |
| 731 | add --upgrade-package to command if upgrade[] passed | pending | — |
| 750 | skips source file package registry extraction when source file is not pip_requirements | pending | — |
| 778 | skips source file when readlocalfile returns null | pending | — |
| 804 | reports errors when a lock file is unchanged | pending | — |

