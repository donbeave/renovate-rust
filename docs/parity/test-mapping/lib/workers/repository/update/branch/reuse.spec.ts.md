# `lib/workers/repository/update/branch/reuse.spec.ts`

[← `worker/repository`](../../../../../_by-module/worker/repository.md) · [all modules](../../../../../README.md)

**0/26 in-scope tests ported** (26 pending, 0 opt-out) · status: pending

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 28 | returns false if branch does not exist | pending | — |
| 34 | returns true if no pr | pending | — |
| 41 | returns true if does not need rebasing | pending | — |
| 49 | returns false if does not need rebasing but has upgrades that need lockfile maintenance along with upgrades that do not | pending | — |
| 77 | returns true if does not need rebasing and lockfile update is on different packages | pending | — |
| 99 | returns true if unmergeable and cannot rebase | pending | — |
| 108 | returns true if unmergeable and can rebase, but rebasewhen is never | pending | — |
| 118 | returns false if unmergeable and can rebase | pending | — |
| 127 | returns true if automerge branch and not stale | pending | — |
| 135 | returns false if automerge branch and stale | pending | — |
| 145 | returns true if rebasewhen=behind-base-branch but cannot rebase | pending | — |
| 156 | returns false if automerge pr and stale | pending | — |
| 166 | returns false if getbranchforcerebase and stale | pending | — |
| 175 | returns true if automerge, rebasewhen=never and stale | pending | — |
| 185 | returns true if automerge, rebasewhen=conflicted and stale | pending | — |
| 194 | returns false if rebasewhen=never, keepupdatedlabel and stale | pending | — |
| 204 | returns false if rebasewhen=conflicted, keepupdatedlabel and modified | pending | — |
| 216 | returns true if rebasewhen=never, miss-match keepupdatedlabel and stale | pending | — |
| 226 | converts rebasewhen=auto to behind-base-branch if automerge | pending | — |
| 236 | converts rebasewhen=auto to behind-base-branch if getbranchforcerebase | pending | — |
| 246 | converts rebasewhen=auto to behind-base-branch if keepupdatedlabel | pending | — |
| 257 | converts rebasewhen=auto to conflicted | pending | — |
| 266 | converts rebasewhen=automerging to behind-base-branch | pending | — |
| 278 | converts rebasewhen=automerging to behind-base-branch if keep-updated | pending | — |
| 292 | converts rebasewhen=automerging to never | pending | — |
| 303 | converts rebasewhen=auto to behind-base-branch if automerge is true and branch is new | pending | — |

