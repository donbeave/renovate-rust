# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pip-compile/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pip-compile/artifacts.spec.ts
**Total tests:** 34 | **Ported:** 0 | **Actionable:** 34 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no requirements.txt found | 84 | pending | — | — | —|
| returns null if all unchanged | 100 | pending | — | — | —|
| returns null if no config.lockFiles | 121 | pending | — | — | —|
| returns updated requirements.txt | 142 | pending | — | — | —|
| supports docker mode | 169 | pending | — | — | —|
| supports install mode | 223 | pending | — | — | —|
| installs Python version according to the lock file | 260 | pending | — | — | —|
| installs Python version according to the uv option | 299 | pending | — | — | —|
| install uv tools without constraints | 342 | pending | — | — | —|
| installs latest Python version if no constraints and not in header | 383 | pending | — | — | —|
| catches errors | 431 | pending | — | — | —|
| returns updated requirements.txt when doing lockfile maintenance | 453 | pending | — | — | —|
| uses --upgrade-package only for isLockfileUpdate | 476 | pending | — | — | —|
| uses pip-compile version from config | 504 | pending | — | — | —|

### `constructPipCompileCmd()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for garbage | 565 | pending | — | — | —|
| returns extracted common arguments (like those featured in the README) | 576 | pending | — | — | —|
| returns extracted arguments for uv | 589 | pending | — | — | —|
| returns --no-emit-index-url when credentials are found in PIP_INDEX_URL | 599 | pending | — | — | —|
| returns --no-emit-index-url when credentials are found in PIP_EXTRA_INDEX_URL | 608 | pending | — | — | —|
| returns --no-emit-index-url when only a username is found in PIP_INDEX_URL | 618 | pending | — | — | —|
| returns --no-emit-index-url when only a username is found in PIP_EXTRA_INDEX_URL | 627 | pending | — | — | —|
| returns --no-emit-index-url when only a password is found in PIP_INDEX_URL | 636 | pending | — | — | —|
| returns --no-emit-index-url when only a password is found in PIP_EXTRA_INDEX_URL | 645 | pending | — | — | —|
| returns --no-emit-index-url when PIP_INDEX_URL is invalid | 654 | pending | — | — | —|
| returns --no-emit-index-url PIP_EXTRA_INDEX_URL is invalid | 663 | pending | — | — | —|
| returns --no-emit-index-url only once when its in the header and credentials are present in the environment | 672 | pending | — | — | —|
| allow explicit --emit-index-url | 687 | pending | — | — | —|
| throws on unknown arguments | 699 | pending | — | — | —|
| throws on custom command | 710 | pending | — | — | —|
| add --upgrade-package to command if Upgrade[] passed | 721 | pending | — | — | —|
| reports errors when a lock file is unchanged | 740 | pending | — | — | —|

| does not add --no-emit-index-url when PIP_INDEX_URL has no credentials | 600 | pending | — | — | —|
| skips source file package registry extraction when source file is not pip_requirements | 750 | pending | — | — | —|
| skips source file when readLocalFile returns null | 778 | pending | — | — | —|
---

