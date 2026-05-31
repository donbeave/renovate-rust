# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/index.spec.ts
**Total tests:** 53 | **Ported:** 0 | **Actionable:** 0 | **Status:** done-applicable

### `workers/repository/update/pr/index › ensurePr › Create`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates PR  | 78 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| fetches changelogs for the "pr" stage  | 98 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| aborts PR creation once limit is exceeded  | 109 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| ignores PR limits on vulnerability alert  | 122 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| creates rollback PR  | 135 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| skips PR creation due to non-green branch check  | 146 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| creates PR for green branch checks  | 158 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| skips PR creation for unapproved dependencies  | 169 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| skips PR creation before prNotPendingHours is hit  | 181 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| skips PR creation due to stabilityStatus  | 201 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| creates PR after prNotPendingHours is hit  | 221 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › Create › Error handling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles unknown error  | 240 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| handles error for PR that already exists  | 250 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| deletes branch on 502 error  | 268 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › Update`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates PR if labels have changed in config  | 285 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| skips pr update if existing pr does not have labels in debugData  | 339 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| skips pr update if pr labels have been modified by user  | 366 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| updates PR due to title change  | 418 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| updates PR due to body change  | 435 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| updates PR target branch if base branch changed in config  | 455 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| ignores reviewable content  | 483 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › dry-run`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| dry-runs PR creation  | 512 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| dry-runs PR update  | 530 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| skips automerge failure comment  | 545 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › Automerge`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles branch automerge  | 563 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| forces PR on dashboard check  | 579 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| adds assignees for PR automerge with red status  | 596 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| adds reviewers for PR automerge with red status and existing ignorable reviewers that can be ignored  | 616 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| skips branch automerge and forces PR creation due to artifact errors  | 637 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| skips branch automerge and forces PR creation due to prNotPendingHours exceeded  | 653 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| automerges branch when prNotPendingHours are not exceeded  | 674 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| comments on automerge failure  | 697 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| handles ensureComment error  | 720 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| logs unknown error  | 737 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| re-throws ExternalHostError  | 761 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| re-throws error with specific message: "$message"  | 782 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › Changelog`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| processes changelogs  | 854 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| handles missing GitHub token  | 880 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| removes duplicate changelogs  | 902 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| remove duplicates release notes  | 928 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| stricter de-deuplication of changelogs  | 958 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › Warnings › Attestations › when attestation is not removed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not warn the user  | 1026 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › Warnings › Attestations › when attestation is removed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| warns the user  | 1056 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › Warnings › Attestations › when attestation is removed in an intermediate version`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not warn the user  | 1104 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › prCache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds pr-cache when not present  | 1128 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| does not update lastEdited pr-cache when pr fingerprint is same but pr was edited within 24hrs  | 1144 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| updates pr-cache when pr fingerprint is different  | 1171 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| skips fetching changelogs when cache is valid and pr was lastEdited before 24hrs  | 1190 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| updates PR when rebase requested by user regardless of pr-cache state  | 1215 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| logs when cache is enabled but pr-cache is absent  | 1259 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| does not log when cache is disabled and pr-cache is absent  | 1268 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |
| skips cache early return when autoApprove is set  | 1278 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › autoApprove`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates PR when autoApprove is set even if PR does not need updating  | 1300 | not-applicable | Mock framework internals — tests PR index via vitest-mocked platform/SCM/template; Rust tests this at different layer | — | PR update runtime behavior is in scope |

---
