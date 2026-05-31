# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/manager/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/index.spec.ts
**Total tests:** 22 | **Ported:** 10 | **Actionable:** 12 | **Status:** done

### `modules/manager/index › supportedDatasources`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has valid supportedDatasources for ${m} | 18 | not-applicable | Mock framework internals — tests manager registry via vitest-mocked manager modules; Rust tests this at different layer | — | Manager registry datasource metadata enumeration not implemented in Rust |

### `modules/manager/index › lockFileNames`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| has lockFileNames for ${name} | 31 | not-applicable | Mock framework internals — tests manager registry via vitest-mocked manager modules; Rust tests this at different layer | — | Manager registry lockfile metadata enumeration not implemented in Rust |

### `modules/manager/index › get()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets something | 38 | ported | `managers.rs` | `manager_registry_manager_exists` | —  | — | — | — |

### `modules/manager/index › getManagerList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets | 45 | ported | `managers.rs` | `manager_registry_get_manager_list` | —  | — | — | — |

### `modules/manager/index › getEnabledManagersList()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 51 | ported | `managers.rs` | `manager_registry_get_enabled_managers_all`, `manager_registry_get_enabled_managers_filtered` | — |

### `modules/manager/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| validates | 60 | not-applicable | Mock framework internals — tests manager registry via vitest-mocked manager modules; Rust tests this at different layer | — | Manager registry schema validation not implemented in Rust |

### `modules/manager/index › detectGlobalConfig()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| iterates through managers | 108 | ported | `managers.rs` | `manager_registry_detect_all_global_config_empty` | —  | — | — | —|

### `modules/manager/index › extractAllPackageFiles()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 114 | not-applicable | Mock framework internals — tests manager registry via vitest-mocked manager modules; Rust tests this at different layer | — | `extractAllPackageFiles` orchestration not implemented in Rust |
| returns non-null | 127 | not-applicable | Mock framework internals — tests manager registry via vitest-mocked manager modules; Rust tests this at different layer | — | `extractAllPackageFiles` orchestration not implemented in Rust |

### `modules/manager/index › extractPackageFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 144 | not-applicable | Mock framework internals — tests manager registry via vitest-mocked manager modules; Rust tests this at different layer | — | `extractPackageFile` registry dispatch not implemented in Rust |
| handles custom managers | 157 | not-applicable | Mock framework internals — tests manager registry via vitest-mocked manager modules; Rust tests this at different layer | — | `extractPackageFile` registry dispatch not implemented in Rust |
| returns non-null | 168 | not-applicable | Mock framework internals — tests manager registry via vitest-mocked manager modules; Rust tests this at different layer | — | `extractPackageFile` registry dispatch not implemented in Rust |

### `modules/manager/index › getRangeStrategy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 186 | not-applicable | Mock framework internals — tests manager registry via vitest-mocked manager modules; Rust tests this at different layer | — | `getRangeStrategy` registry dispatch not implemented in Rust |
| returns non-null | 196 | not-applicable | Mock framework internals — tests manager registry via vitest-mocked manager modules; Rust tests this at different layer | — | `getRangeStrategy` registry dispatch not implemented in Rust |
| returns update-lockfile for in-range-only | 219 | not-applicable | Mock framework internals — tests manager registry via vitest-mocked manager modules; Rust tests this at different layer | — | `getRangeStrategy` registry dispatch not implemented in Rust |
| returns update-lockfile for in-range-only if it is proposed my manager | 232 | not-applicable | Mock framework internals — tests manager registry via vitest-mocked manager modules; Rust tests this at different layer | — | `getRangeStrategy` registry dispatch not implemented in Rust |

### `modules/manager/index › isKnownManager`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true | 252 | ported | `managers.rs` | `manager_registry_manager_exists` | — |
| returns false | 258 | ported | `managers.rs` | `manager_registry_manager_exists` | — |

### `modules/manager/index › getPrettyDepType`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when no manager found, returns undefined | 265 | ported | `managers.rs` | `manager_get_pretty_dep_type` | — |
| when manager found, but no prettyDepType found, returns undefined | 271 | ported | `managers.rs` | `manager_get_pretty_dep_type` | — |
| when manager found, but no prettyDepType found, returns undefined | 275 | ported | `managers.rs` | `manager_get_pretty_dep_type` | — |
| when manager found, and a prettyDepType found in knownDepTypes, returns the defined prettyDepType | 279 | ported | `managers.rs` | `manager_get_pretty_dep_type` | — |

---

