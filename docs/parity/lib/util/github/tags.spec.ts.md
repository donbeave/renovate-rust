# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/github/tags.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/github/tags.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `util/github/tags › findCommitOfTag`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be able to find the hash of a Git tag | 11 | not-applicable | — | — | Mock framework internals — tests GitHub tag resolution via vitest-mocked GraphQL queries; Rust uses different architecture |
| should support passing a custom registry URL | 36 | not-applicable | — | — | Mock framework internals — tests GitHub tag resolution via vitest-mocked GraphQL queries; Rust uses different architecture |
| should return `null` if the tag does not exist | 55 | not-applicable | — | — | Mock framework internals — tests GitHub tag resolution via vitest-mocked GraphQL queries; Rust uses different architecture |
| should gracefully return `null` if tags cannot be queried | 67 | not-applicable | — | — | Mock framework internals — tests GitHub tag resolution via vitest-mocked GraphQL queries; Rust uses different architecture |

---

