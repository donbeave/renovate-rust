# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/poetry/artifacts.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/poetry/artifacts.spec.ts
**Total tests:** 19 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `getPythonConstraint`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects from pyproject.toml | 56 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| detects from poetry.ock | 67 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

### `getPoetryRequirement`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| detects poetry from first line of poetry.lock | 76 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| detects poetry from metadata | 83 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

### `updateArtifacts`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null if no poetry.lock found | 99 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if updatedDeps is empty | 113 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns null if unchanged | 126 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated poetry.lock | 151 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| passes private credential environment vars | 179 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| passes Google Artifact Registry credentials environment vars | 228 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| continues if Google auth is not configured | 277 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| prioritizes pypi-scoped credentials | 317 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated pyproject.lock | 356 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated poetry.lock using docker | 387 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| supports docker mode with github credentials | 452 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated poetry.lock using docker (constraints) | 541 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated poetry.lock using install mode | 607 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| catches errors | 652 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |
| returns updated poetry.lock when doing lockfile maintenance | 672 | not-applicable | — | — | out of scope: artifact management; invokes external package managers not called by Rust CLI |

---

