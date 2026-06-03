# `lib/workers/repository/update/pr/code-owners.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**0/31 ported** (31 pending) · status: pending

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 28 | returns global code owner | pending | — |
| 35 | returns global code owner for commit with sha set | pending | — |
| 43 | respects orphan files | pending | — |
| 55 | does not return any owners if pr has no changes | pending | — |
| 62 | returns more specific code owners | pending | — |
| 91 | does not assign changes for yarn.lock | pending | — |
| 98 | assigns root changes to @john (*) | pending | — |
| 105 | assigns changes in package a to @maria (a), @john (*) | pending | — |
| 115 | assigns changes in package b to @jimmy (b), @john (*) | pending | — |
| 125 | assigns changes in package c to @dan (c), @john (*) | pending | — |
| 135 | assigns changes in package d to @maria (d), @jimmy (d), @john (*) | pending | — |
| 145 | assigns changes in package a and b to @maria (a), @jimmy (b), @john (*) | pending | — |
| 156 | assigns changes in package a, b and c to @john, @maria (a), @jimmy (b), @dan (c), @john (*) | pending | — |
| 168 | assigns changes in package c and d to @dan (c), @maria (d), @jimmy (e), @john (*) | pending | — |
| 179 | assigns changes in package d and e to @jimmy (d, e), @maria (d), @john (*) | pending | — |
| 199 | returns section code owner | pending | — |
| 232 | returns code owners of multiple sections | pending | — |
| 245 | returns default owners when none is explicitly set | pending | — |
| 260 | parses only sections that start at the beginning of a line | pending | — |
| 271 | returns code owners for optional sections | pending | — |
| 297 | returns code owners for matching file using escaped spaces | pending | — |
| 308 | returns code owners from reviewer group with random selection | pending | — |
| 322 | does not return owners when an empty rule overrides a broader rule | pending | — |
| 336 | matches the most specific rule (bottom takes precedence) | pending | — |
| 350 | handles multiple owners with mix of usernames and groups | pending | — |
| 386 | does not require all files to match a single rule, regression test for #12611 | pending | — |
| 429 | ignores comments and leading/trailing whitespace | pending | — |
| 444 | returns empty array when no code owners set | pending | — |
| 451 | returns empty array when no code owners match | pending | — |
| 460 | returns empty array when error occurs | pending | — |
| 473 | detects code owner file at '${codeownerfilepath}' | pending | — |

