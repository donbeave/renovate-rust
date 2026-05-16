# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/manager/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/index.spec.ts
**Total tests:** 22 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/manager/index › supportedDatasources`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has valid supportedDatasources for ${m} | 18 | not-applicable | — | — | Renovate's TypeScript manager registry metadata validation is not implemented as a Rust API. |

### `modules/manager/index › lockFileNames`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has lockFileNames for ${name} | 31 | not-applicable | — | — | Renovate's TypeScript manager registry lock-file metadata validation is not implemented as a Rust API. |

### `modules/manager/index › get()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets something | 38 | not-applicable | — | — | Renovate's TypeScript manager registry lookup API is not implemented as a Rust API. |

### `modules/manager/index › getManagerList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets | 45 | not-applicable | — | — | Renovate's TypeScript manager registry list API is not implemented as a Rust API. |

### `modules/manager/index › getEnabledManagersList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 51 | not-applicable | — | — | Renovate's TypeScript enabled-manager list normalization helper is not implemented as a Rust API. |

### `modules/manager/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| validates | 60 | not-applicable | — | — | Renovate's TypeScript dynamic manager module registry validation is not implemented as a Rust API. |

### `modules/manager/index › detectGlobalConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| iterates through managers | 108 | not-applicable | — | — | Renovate's TypeScript manager global-config detection hook is not implemented as a Rust API. |

### `modules/manager/index › extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 114 | not-applicable | — | — | Renovate's TypeScript generic manager extraction dispatcher is not implemented as a Rust API; Rust invokes concrete extractor pipelines directly. |
| returns non-null | 127 | not-applicable | — | — | Renovate's TypeScript generic manager extraction dispatcher is not implemented as a Rust API; Rust invokes concrete extractor pipelines directly. |

### `modules/manager/index › extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 144 | not-applicable | — | — | Renovate's TypeScript generic manager extraction dispatcher is not implemented as a Rust API; Rust invokes concrete extractor pipelines directly. |
| handles custom managers | 157 | not-applicable | — | — | Renovate's TypeScript custom-manager registry dispatcher is not implemented as a Rust API. |
| returns non-null | 168 | not-applicable | — | — | Renovate's TypeScript generic manager extraction dispatcher is not implemented as a Rust API; Rust invokes concrete extractor pipelines directly. |

### `modules/manager/index › getRangeStrategy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 186 | not-applicable | — | — | Renovate's TypeScript manager range-strategy registry dispatch is not implemented as a Rust API. |
| returns non-null | 196 | not-applicable | — | — | Renovate's TypeScript manager range-strategy registry dispatch is not implemented as a Rust API. |
| returns update-lockfile for in-range-only | 219 | not-applicable | — | — | Renovate's TypeScript manager range-strategy registry dispatch is not implemented as a Rust API. |
| returns update-lockfile for in-range-only if it is proposed my manager | 232 | not-applicable | — | — | Renovate's TypeScript manager range-strategy registry dispatch is not implemented as a Rust API. |

### `modules/manager/index › isKnownManager`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true | 252 | not-applicable | — | — | Renovate's TypeScript manager registry predicate is not implemented as a Rust API. |
| returns false | 258 | not-applicable | — | — | Renovate's TypeScript manager registry predicate is not implemented as a Rust API. |

### `modules/manager/index › getPrettyDepType`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when no manager found, returns undefined | 265 | not-applicable | — | — | Renovate's TypeScript pretty dependency-type registry helper is not implemented as a Rust API. |
| when manager found, but no prettyDepType found, returns undefined | 271 | not-applicable | — | — | Renovate's TypeScript pretty dependency-type registry helper is not implemented as a Rust API. |
| when manager found, but no prettyDepType found, returns undefined | 275 | not-applicable | — | — | Renovate's TypeScript pretty dependency-type registry helper is not implemented as a Rust API. |
| when manager found, and a prettyDepType found in knownDepTypes, returns the defined prettyDepType | 279 | not-applicable | — | — | Renovate's TypeScript pretty dependency-type registry helper is not implemented as a Rust API. |

---

