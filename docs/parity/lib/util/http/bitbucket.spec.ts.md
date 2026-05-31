# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/bitbucket.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/bitbucket.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable-applicable

### `util/http/bitbucket`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| posts | 25 | not-applicable | — | — | TS-library-specific schema internals; Bitbucket `got` wrapper pagination API is TypeScript-specific with no direct Rust equivalent |
| accepts custom baseUrl | 32 | not-applicable | — | — | TS-library-specific schema internals; Bitbucket `got` wrapper pagination API is TypeScript-specific with no direct Rust equivalent |
| paginates: adds default pagelen if non is present | 57 | not-applicable | — | — | TS-library-specific schema internals; Bitbucket `got` wrapper pagination API is TypeScript-specific with no direct Rust equivalent |
| paginates: respects pagelen if already set in path | 93 | not-applicable | — | — | TS-library-specific schema internals; Bitbucket `got` wrapper pagination API is TypeScript-specific with no direct Rust equivalent |
| paginates: respects pagelen if set in options | 129 | not-applicable | — | — | TS-library-specific schema internals; Bitbucket `got` wrapper pagination API is TypeScript-specific with no direct Rust equivalent |

---

