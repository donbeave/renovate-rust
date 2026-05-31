# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/code-owners.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/code-owners.spec.ts
**Total tests:** 31 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `workers/repository/update/pr/code-owners › codeOwnersForPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns global code owner  | 28 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| returns global code owner for commit with sha set  | 35 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| respects orphan files  | 43 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| does not return any owners if PR has no changes  | 55 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| returns more specific code owners  | 62 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |

### `workers/repository/update/pr/code-owners › codeOwnersForPr › returns more specific code owners in monorepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not assign changes for yarn.lock  | 91 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| assigns root changes to @john (*)  | 98 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| assigns changes in package A to @maria (a), @john (*)  | 105 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| assigns changes in package B to @jimmy (b), @john (*)  | 115 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| assigns changes in package C to @dan (c), @john (*)  | 125 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| assigns changes in package D to @maria (d), @jimmy (d), @john (*)  | 135 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| assigns changes in package A and B to @maria (a), @jimmy (b), @john (*)  | 145 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| assigns changes in package A, B and C to @john, @maria (a), @jimmy (b), @dan (c), @john (*)  | 156 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| assigns changes in package C and D to @dan (c), @maria (d), @jimmy (e), @john (*)  | 168 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| assigns changes in package D and E to @jimmy (d, e), @maria (d), @john (*)  | 179 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |

### `workers/repository/update/pr/code-owners › codeOwnersForPr › supports Gitlab sections`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns section code owner  | 199 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| returns code owners of multiple sections  | 232 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| returns default owners when none is explicitly set  | 245 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| parses only sections that start at the beginning of a line  | 260 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| returns code owners for optional sections  | 271 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |

### `workers/repository/update/pr/code-owners › codeOwnersForPr › Bitbucket Server CODEOWNERS integration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns code owners for matching file using escaped spaces  | 297 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| returns code owners from reviewer group with random selection  | 308 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| does not return owners when an empty rule overrides a broader rule  | 322 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| matches the most specific rule (bottom takes precedence)  | 336 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| handles multiple owners with mix of usernames and groups  | 350 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| does not require all files to match a single rule, regression test for #12611  | 386 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| ignores comments and leading/trailing whitespace  | 429 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| returns empty array when no code owners set  | 444 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| returns empty array when no code owners match  | 451 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| returns empty array when error occurs  | 460 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |
| detects code owner file at '${codeOwnerFilePath}'  | 473 | not-applicable | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer | — | Mock framework internals — tests PR code-owners via vitest-mocked platform/HTTP; Rust tests this at different layer |

---
