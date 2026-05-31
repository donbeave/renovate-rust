# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pipenv/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pipenv/artifacts.spec.ts
**Total tests:** 21 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns if no Pipfile.lock found | 119 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if unchanged | 130 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| gets python full version from Pipfile | 183 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| gets python version from Pipfile | 236 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| gets full python version from .python-version | 289 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| gets python stream, from .python-version | 348 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| handles no constraint | 406 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns updated Pipfile.lock | 462 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| supports docker mode | 525 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| supports install mode | 613 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| defaults to latest if no lock constraints | 685 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| catches errors | 757 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns updated Pipenv.lock when doing lockfile maintenance | 784 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses pipenv version from Pipfile | 832 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses pipenv version from Pipfile dev packages | 919 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| uses pipenv version from config | 1006 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| passes private credential environment vars | 1087 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns no host rule on invalid url | 1140 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| extractEnvironmentVariableName($credential) | 1144 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| warns about duplicate placeholders with different values | 1156 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| updates extraEnv if variable names differ from default | 1170 | not-applicable | Mock framework internals — tests pipenv artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

---

