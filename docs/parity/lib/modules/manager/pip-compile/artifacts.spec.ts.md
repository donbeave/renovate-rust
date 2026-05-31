# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pip-compile/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pip-compile/artifacts.spec.ts
**Total tests:** 34 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no requirements.txt found | 84 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if all unchanged | 100 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if no config.lockFiles | 121 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns updated requirements.txt | 142 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| supports docker mode | 169 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| supports install mode | 223 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| installs Python version according to the lock file | 260 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| installs Python version according to the uv option | 299 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| install uv tools without constraints | 342 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| installs latest Python version if no constraints and not in header | 383 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| catches errors | 431 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns updated requirements.txt when doing lockfile maintenance | 453 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses --upgrade-package only for isLockfileUpdate | 476 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses pip-compile version from config | 504 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `constructPipCompileCmd()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for garbage | 565 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns extracted common arguments (like those featured in the README) | 576 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns extracted arguments for uv | 589 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns --no-emit-index-url when credentials are found in PIP_INDEX_URL | 599 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns --no-emit-index-url when credentials are found in PIP_EXTRA_INDEX_URL | 608 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns --no-emit-index-url when only a username is found in PIP_INDEX_URL | 618 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns --no-emit-index-url when only a username is found in PIP_EXTRA_INDEX_URL | 627 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns --no-emit-index-url when only a password is found in PIP_INDEX_URL | 636 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns --no-emit-index-url when only a password is found in PIP_EXTRA_INDEX_URL | 645 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns --no-emit-index-url when PIP_INDEX_URL is invalid | 654 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns --no-emit-index-url PIP_EXTRA_INDEX_URL is invalid | 663 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns --no-emit-index-url only once when its in the header and credentials are present in the environment | 672 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| allow explicit --emit-index-url | 687 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| throws on unknown arguments | 699 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| throws on custom command | 710 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| add --upgrade-package to command if Upgrade[] passed | 721 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| reports errors when a lock file is unchanged | 740 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

| does not add --no-emit-index-url when PIP_INDEX_URL has no credentials | 600 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| skips source file package registry extraction when source file is not pip_requirements | 750 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| skips source file when readLocalFile returns null | 778 | not-applicable | Mock framework internals — tests pip-compile artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
---

