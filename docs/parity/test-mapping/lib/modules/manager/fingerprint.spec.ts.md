# `lib/modules/manager/fingerprint.spec.ts`

[← `manager/_common`](../../../_by-module/manager/_common.md) · [all modules](../../../README.md)

**0/0 in-scope tests ported** (0 pending, 1 opt-out) · status: opt-out

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 5 | validate manager hash | opt-out | loops allManagersList and asserts every entry in the precomputed hashMap has a valid 64-hex SHA256 string; this is a data integrity check on a build-time/generated map of manager -> hash (likely of the manager's code/defaultConfig or for change detection). Rust has static manager registry (no equivalent hashMap of hex strings per manager name); the general fingerprint_json util is ported separately. Pure TS build/data validation test with no core manager extraction logic. |

