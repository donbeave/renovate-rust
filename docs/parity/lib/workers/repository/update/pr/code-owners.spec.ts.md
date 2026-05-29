# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/code-owners.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/code-owners.spec.ts
**Total tests:** 31 | **Ported:** 0 | **Actionable:** 31 | **Status:** pending

### `workers/repository/update/pr/code-owners › codeOwnersForPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns global code owner | 28 | pending | — | — | —|
| returns global code owner for commit with sha set | 35 | pending | — | — | —|
| respects orphan files | 43 | pending | — | — | —|
| does not return any owners if PR has no changes | 55 | pending | — | — | —|
| returns more specific code owners | 62 | pending | — | — | —|

### `workers/repository/update/pr/code-owners › codeOwnersForPr › returns more specific code owners in monorepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not assign changes for yarn.lock | 91 | pending | — | — | —|
| assigns root changes to @john (*) | 98 | pending | — | — | —|
| assigns changes in package A to @maria (a), @john (*) | 105 | pending | — | — | —|
| assigns changes in package B to @jimmy (b), @john (*) | 115 | pending | — | — | —|
| assigns changes in package C to @dan (c), @john (*) | 125 | pending | — | — | —|
| assigns changes in package D to @maria (d), @jimmy (d), @john (*) | 135 | pending | — | — | —|
| assigns changes in package A and B to @maria (a), @jimmy (b), @john (*) | 145 | pending | — | — | —|
| assigns changes in package A, B and C to @john, @maria (a), @jimmy (b), @dan (c), @john (*) | 156 | pending | — | — | —|
| assigns changes in package C and D to @dan (c), @maria (d), @jimmy (e), @john (*) | 168 | pending | — | — | —|
| assigns changes in package D and E to @jimmy (d, e), @maria (d), @john (*) | 179 | pending | — | — | —|

### `workers/repository/update/pr/code-owners › codeOwnersForPr › supports Gitlab sections`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns section code owner | 199 | pending | — | — | —|
| returns code owners of multiple sections | 232 | pending | — | — | —|
| returns default owners when none is explicitly set | 245 | pending | — | — | —|
| parses only sections that start at the beginning of a line | 260 | pending | — | — | —|
| returns code owners for optional sections | 271 | pending | — | — | —|

### `workers/repository/update/pr/code-owners › codeOwnersForPr › Bitbucket Server CODEOWNERS integration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns code owners for matching file using escaped spaces | 297 | pending | — | — | —|
| returns code owners from reviewer group with random selection | 308 | pending | — | — | —|
| does not return owners when an empty rule overrides a broader rule | 322 | pending | — | — | —|
| matches the most specific rule (bottom takes precedence) | 336 | pending | — | — | —|
| handles multiple owners with mix of usernames and groups | 350 | pending | — | — | —|
| does not require all files to match a single rule, regression test for #12611 | 386 | pending | — | — | —|
| ignores comments and leading/trailing whitespace | 429 | pending | — | — | —|
| returns empty array when no code owners set | 444 | pending | — | — | —|
| returns empty array when no code owners match | 451 | pending | — | — | —|
| returns empty array when error occurs | 460 | pending | — | — | —|
| detects code owner file at '${codeOwnerFilePath}' | 473 | pending | — | — | —|

---
