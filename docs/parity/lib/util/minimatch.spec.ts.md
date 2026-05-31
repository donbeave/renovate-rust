# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/minimatch.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/minimatch.spec.ts
**Total tests:** 5 | **Ported:** 2 | **Actionable:** 3 | **Status:** pending

### `util/minimatch › minimatch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| caches minimatch | 5 | pending | — | — | TypeScript type-system test; checks JavaScript reference identity (toBe same Minimatch object) — object identity has no Rust equivalent |
| does not cache minimatch | 12 | pending | — | — | TypeScript type-system test; checks JavaScript reference identity (not.toBe same object when cache=false) — object identity has no Rust equivalent |
| matches | 20 | ported | `string_match.rs` | `minimatch_glob_path_matching` | — |

### `util/minimatch › minimatchFilter`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return a function | 32 | pending | — | — | TypeScript type-system test; checks that minimatchFilter returns a function type — TypeScript toBeFunction() check has no Rust equivalent |
| should correctly match filenames | 37 | ported | `string_match.rs` | `minimatch_filter_filename_matching` | — |

---
