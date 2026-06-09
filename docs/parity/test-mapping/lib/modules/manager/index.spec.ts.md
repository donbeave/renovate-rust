# `lib/modules/manager/index.spec.ts`

[← `manager/_common`](../../../_by-module/manager/_common.md) · [all modules](../../../README.md)

**9/10 in-scope tests ported** (1 pending, 12 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | has valid supporteddatasources for ${m} | opt-out | parametrized registry data check (loop over getManagerList, assert supportedDatasources non-empty and all known); TS runtime iteration over manager metadata. Rust static MANAGER_DEFS + list fns (core already ported in other tests); this exact form is TS-specific registry validation. |
| 31 | has lockfilenames for ${name} | opt-out | parametrized check for lock-supporting managers (filter supportsLockFileMaintenance, assert lockFileNames non-empty); similar TS registry data loop. Equivalent data in Rust artifact/locked paths (covered elsewhere); the loop in this spec is runtime validation detail. |
| 38 | gets something | ported | [`crates/renovate-core/src/managers.rs:2413`](../../../../../../crates/renovate-core/src/managers.rs#L2413) |
| 45 | gets | ported | [`crates/renovate-core/src/managers.rs:2369`](../../../../../../crates/renovate-core/src/managers.rs#L2369) |
| 51 | works | ported | [`crates/renovate-core/src/managers.rs:2379`](../../../../../../crates/renovate-core/src/managers.rs#L2379) |
| 60 | validates | opt-out | dynamic loadModules (TS require + dir scan of managers/custom) + custom validate fn (defaultConfig, extract* or updateDependency, no undefined exports) + key cross-match vs getManagers. Pure TS runtime module loading + reflection with no Rust static equivalent (Rust is compile-time registry; core get/exists behaviors ported). |
| 108 | iterates through managers | ported | [`crates/renovate-core/src/managers.rs:2429`](../../../../../../crates/renovate-core/src/managers.rs#L2429) |
| 114 | returns null | opt-out | multiple (extractAll, extractPackageFile, getRangeStrategy) using managers map monkey-patch with 'dummy' (no fn) then assert index facade returns null for unknown/missing-fn. TS mutable registry + injection for dispatch guard tests; no direct Rust (static); core unknown guard via manager_exists + per-manager none already exercised. |
| 127 | returns non-null | opt-out | sibling dummy-injection tests asserting non-null when dummy has the fn. Same TS facade mutation pattern for dispatch; core for known managers covered by existing ports. |
| 144 | returns null | opt-out | multiple (extractAll, extractPackageFile, getRangeStrategy) using managers map monkey-patch with 'dummy' (no fn) then assert index facade returns null for unknown/missing-fn. TS mutable registry + injection for dispatch guard tests; no direct Rust (static); core unknown guard via manager_exists + per-manager none already exercised. |
| 157 | handles custom managers | opt-out | customManager.getCustomManagers().set + index extractPackageFile assert non-null. Tests custom delegation via runtime map. Rust custom (regex etc.) via is_custom_manager + static wiring; behavior covered in other tests. |
| 168 | returns non-null | opt-out | sibling dummy-injection tests asserting non-null when dummy has the fn. Same TS facade mutation pattern for dispatch; core for known managers covered by existing ports. |
| 186 | returns null | opt-out | multiple (extractAll, extractPackageFile, getRangeStrategy) using managers map monkey-patch with 'dummy' (no fn) then assert index facade returns null for unknown/missing-fn. TS mutable registry + injection for dispatch guard tests; no direct Rust (static); core unknown guard via manager_exists + per-manager none already exercised. |
| 196 | returns non-null | opt-out | sibling dummy-injection tests asserting non-null when dummy has the fn. Same TS facade mutation pattern for dispatch; core for known managers covered by existing ports. |
| 219 | returns update-lockfile for in-range-only | opt-out | getRangeStrategy dummy test for in-range-only input yielding 'update-lockfile' (and when manager proposes it). The special case is in util::get_range_strategy (unconditional, prior ports); this is the TS index facade version using map mutation. |
| 232 | returns update-lockfile for in-range-only if it is proposed my manager | opt-out | paired in-range-only facade test. Same util logic already marked/port covered; exact it() here is the wrapper test. |
| 252 | returns true | ported | [`crates/renovate-core/src/managers.rs:2414`](../../../../../../crates/renovate-core/src/managers.rs#L2414) |
| 258 | returns false | ported | [`crates/renovate-core/src/managers.rs:2415`](../../../../../../crates/renovate-core/src/managers.rs#L2415) |
| 265 | when no manager found, returns undefined | ported | [`crates/renovate-core/src/managers.rs:2396`](../../../../../../crates/renovate-core/src/managers.rs#L2396) |
| 271 | when manager found, but no prettydeptype found, returns undefined | ported | [`crates/renovate-core/src/managers.rs:2397`](../../../../../../crates/renovate-core/src/managers.rs#L2397) |
| 275 | when manager found, but no prettydeptype found, returns undefined | ported | [`crates/renovate-core/src/managers.rs:2397`](../../../../../../crates/renovate-core/src/managers.rs#L2397) |
| 279 | when manager found, and a prettydeptype found in knowndeptypes, returns the defined prettydeptype | ported | [`crates/renovate-core/src/managers.rs:2398`](../../../../../../crates/renovate-core/src/managers.rs#L2398) |

