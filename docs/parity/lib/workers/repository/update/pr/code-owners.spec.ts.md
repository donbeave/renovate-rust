# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/code-owners.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/code-owners.spec.ts
**Total tests:** 31 | **Ported:** 0 | **Actionable:** 31 | **Status:** not-applicable

### `workers/repository/update/pr/code-owners › codeOwnersForPr`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns global code owner | 28 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| returns global code owner for commit with sha set | 35 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| respects orphan files | 43 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| does not return any owners if PR has no changes | 55 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| returns more specific code owners | 62 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |

### `workers/repository/update/pr/code-owners › codeOwnersForPr › returns more specific code owners in monorepos`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not assign changes for yarn.lock | 91 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| assigns root changes to @john (*) | 98 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| assigns changes in package A to @maria (a), @john (*) | 105 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| assigns changes in package B to @jimmy (b), @john (*) | 115 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| assigns changes in package C to @dan (c), @john (*) | 125 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| assigns changes in package D to @maria (d), @jimmy (d), @john (*) | 135 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| assigns changes in package A and B to @maria (a), @jimmy (b), @john (*) | 145 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| assigns changes in package A, B and C to @john, @maria (a), @jimmy (b), @dan (c), @john (*) | 156 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| assigns changes in package C and D to @dan (c), @maria (d), @jimmy (e), @john (*) | 168 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| assigns changes in package D and E to @jimmy (d, e), @maria (d), @john (*) | 179 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |

### `workers/repository/update/pr/code-owners › codeOwnersForPr › supports Gitlab sections`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns section code owner | 199 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| returns code owners of multiple sections | 232 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| returns default owners when none is explicitly set | 245 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| parses only sections that start at the beginning of a line | 260 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| returns code owners for optional sections | 271 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |

### `workers/repository/update/pr/code-owners › codeOwnersForPr › Bitbucket Server CODEOWNERS integration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns code owners for matching file using escaped spaces | 297 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| returns code owners from reviewer group with random selection | 308 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| does not return owners when an empty rule overrides a broader rule | 322 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| matches the most specific rule (bottom takes precedence) | 336 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| handles multiple owners with mix of usernames and groups | 350 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| does not require all files to match a single rule, regression test for #12611 | 386 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| ignores comments and leading/trailing whitespace | 429 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| returns empty array when no code owners set | 444 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| returns empty array when no code owners match | 451 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| returns empty array when error occurs | 460 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |
| detects code owner file at '${codeOwnerFilePath}' | 473 | not-applicable | — | — | Uses fs.readLocalFile mock + git.getBranchFiles mock + platform mock; fs/git/platform mock infrastructure not portable to Rust |

---
