# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/poetry/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/poetry/artifacts.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `getPythonConstraint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects from pyproject.toml | 56 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| detects from poetry.ock | 67 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `getPoetryRequirement`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects poetry from first line of poetry.lock | 76 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| detects poetry from metadata | 83 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

### `updateArtifacts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no poetry.lock found | 99 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if updatedDeps is empty | 113 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns null if unchanged | 126 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns updated poetry.lock | 151 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| passes private credential environment vars | 179 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| passes Google Artifact Registry credentials environment vars | 228 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| continues if Google auth is not configured | 277 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| prioritizes pypi-scoped credentials | 317 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns updated pyproject.lock | 356 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns updated poetry.lock using docker | 387 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| supports docker mode with github credentials | 452 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns updated poetry.lock using docker (constraints) | 541 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns updated poetry.lock using install mode | 607 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| catches errors | 652 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|
| returns updated poetry.lock when doing lockfile maintenance | 672 | not-applicable | Mock framework internals — tests poetry artifacts via vitest-mocked fs/exec; Rust tests this at different layer | — | —|

---

