# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/poetry/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/poetry/artifacts.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 19 | **Status:** pending

### `getPythonConstraint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects from pyproject.toml | 56 | pending | — | — | — |
| detects from poetry.ock | 67 | pending | — | — | — |

### `getPoetryRequirement`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects poetry from first line of poetry.lock | 76 | pending | — | — | — |
| detects poetry from metadata | 83 | pending | — | — | — |

### `updateArtifacts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no poetry.lock found | 99 | pending | — | — | — |
| returns null if updatedDeps is empty | 113 | pending | — | — | — |
| returns null if unchanged | 126 | pending | — | — | — |
| returns updated poetry.lock | 151 | pending | — | — | — |
| passes private credential environment vars | 179 | pending | — | — | — |
| passes Google Artifact Registry credentials environment vars | 228 | pending | — | — | — |
| continues if Google auth is not configured | 277 | pending | — | — | — |
| prioritizes pypi-scoped credentials | 317 | pending | — | — | — |
| returns updated pyproject.lock | 356 | pending | — | — | — |
| returns updated poetry.lock using docker | 387 | pending | — | — | — |
| supports docker mode with github credentials | 452 | pending | — | — | — |
| returns updated poetry.lock using docker (constraints) | 541 | pending | — | — | — |
| returns updated poetry.lock using install mode | 607 | pending | — | — | — |
| catches errors | 652 | pending | — | — | — |
| returns updated poetry.lock when doing lockfile maintenance | 672 | pending | — | — | — |

---

