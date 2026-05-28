# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/minimatch.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/minimatch.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 5 | **Status:** partial

### `util/minimatch › minimatch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| caches minimatch | 5 | not-applicable | — | — | JavaScript object identity (same Minimatch instance); Rust has no equivalent cached reference |
| does not cache minimatch | 12 | not-applicable | — | — | JavaScript object identity semantics |
| matches | 20 | not-applicable | — | — | `{/,}` brace expansion in minimatch (`/` or empty) differs from globset (`/` or `,`); semantic incompatibility |

### `util/minimatch › minimatchFilter`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return a function | 32 | not-applicable | — | — | JavaScript higher-order function semantics |
| should correctly match filenames | 37 | not-applicable | — | — | Covered by string_match.rs globset tests |

---
