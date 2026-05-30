# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/index.spec.ts
**Total tests:** 53 | **Ported:** 0 | **Actionable:** 53 | **Status:** pending

### `workers/repository/update/pr/index › ensurePr › Create`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates PR | 78 | pending | — | — | PR update runtime behavior is in scope |
| fetches changelogs for the "pr" stage | 98 | pending | — | — | PR update runtime behavior is in scope |
| aborts PR creation once limit is exceeded | 109 | pending | — | — | PR update runtime behavior is in scope |
| ignores PR limits on vulnerability alert | 122 | pending | — | — | PR update runtime behavior is in scope |
| creates rollback PR | 135 | pending | — | — | PR update runtime behavior is in scope |
| skips PR creation due to non-green branch check | 146 | pending | — | — | PR update runtime behavior is in scope |
| creates PR for green branch checks | 158 | pending | — | — | PR update runtime behavior is in scope |
| skips PR creation for unapproved dependencies | 169 | pending | — | — | PR update runtime behavior is in scope |
| skips PR creation before prNotPendingHours is hit | 181 | pending | — | — | PR update runtime behavior is in scope |
| skips PR creation due to stabilityStatus | 201 | pending | — | — | PR update runtime behavior is in scope |
| creates PR after prNotPendingHours is hit | 221 | pending | — | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › Create › Error handling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles unknown error | 240 | pending | — | — | PR update runtime behavior is in scope |
| handles error for PR that already exists | 250 | pending | — | — | PR update runtime behavior is in scope |
| deletes branch on 502 error | 268 | pending | — | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › Update`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates PR if labels have changed in config | 285 | pending | — | — | PR update runtime behavior is in scope |
| skips pr update if existing pr does not have labels in debugData | 339 | pending | — | — | PR update runtime behavior is in scope |
| skips pr update if pr labels have been modified by user | 366 | pending | — | — | PR update runtime behavior is in scope |
| updates PR due to title change | 418 | pending | — | — | PR update runtime behavior is in scope |
| updates PR due to body change | 435 | pending | — | — | PR update runtime behavior is in scope |
| updates PR target branch if base branch changed in config | 455 | pending | — | — | PR update runtime behavior is in scope |
| ignores reviewable content | 483 | pending | — | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › dry-run`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| dry-runs PR creation | 512 | pending | — | — | PR update runtime behavior is in scope |
| dry-runs PR update | 530 | pending | — | — | PR update runtime behavior is in scope |
| skips automerge failure comment | 545 | pending | — | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › Automerge`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles branch automerge | 563 | pending | — | — | PR update runtime behavior is in scope |
| forces PR on dashboard check | 579 | pending | — | — | PR update runtime behavior is in scope |
| adds assignees for PR automerge with red status | 596 | pending | — | — | PR update runtime behavior is in scope |
| adds reviewers for PR automerge with red status and existing ignorable reviewers that can be ignored | 616 | pending | — | — | PR update runtime behavior is in scope |
| skips branch automerge and forces PR creation due to artifact errors | 637 | pending | — | — | PR update runtime behavior is in scope |
| skips branch automerge and forces PR creation due to prNotPendingHours exceeded | 653 | pending | — | — | PR update runtime behavior is in scope |
| automerges branch when prNotPendingHours are not exceeded | 674 | pending | — | — | PR update runtime behavior is in scope |
| comments on automerge failure | 697 | pending | — | — | PR update runtime behavior is in scope |
| handles ensureComment error | 720 | pending | — | — | PR update runtime behavior is in scope |
| logs unknown error | 737 | pending | — | — | PR update runtime behavior is in scope |
| re-throws ExternalHostError | 761 | pending | — | — | PR update runtime behavior is in scope |
| re-throws error with specific message: "$message" | 782 | pending | — | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › Changelog`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| processes changelogs | 854 | pending | — | — | PR update runtime behavior is in scope |
| handles missing GitHub token | 880 | pending | — | — | PR update runtime behavior is in scope |
| removes duplicate changelogs | 902 | pending | — | — | PR update runtime behavior is in scope |
| remove duplicates release notes | 928 | pending | — | — | PR update runtime behavior is in scope |
| stricter de-deuplication of changelogs | 958 | pending | — | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › Warnings › Attestations › when attestation is not removed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not warn the user | 1026 | pending | — | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › Warnings › Attestations › when attestation is removed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| warns the user | 1056 | pending | — | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › Warnings › Attestations › when attestation is removed in an intermediate version`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not warn the user | 1104 | pending | — | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › prCache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds pr-cache when not present | 1128 | pending | — | — | PR update runtime behavior is in scope |
| does not update lastEdited pr-cache when pr fingerprint is same but pr was edited within 24hrs | 1144 | pending | — | — | PR update runtime behavior is in scope |
| updates pr-cache when pr fingerprint is different | 1171 | pending | — | — | PR update runtime behavior is in scope |
| skips fetching changelogs when cache is valid and pr was lastEdited before 24hrs | 1190 | pending | — | — | PR update runtime behavior is in scope |
| updates PR when rebase requested by user regardless of pr-cache state | 1215 | pending | — | — | PR update runtime behavior is in scope |
| logs when cache is enabled but pr-cache is absent | 1259 | pending | — | — | PR update runtime behavior is in scope |
| does not log when cache is disabled and pr-cache is absent | 1268 | pending | — | — | PR update runtime behavior is in scope |
| skips cache early return when autoApprove is set | 1278 | pending | — | — | PR update runtime behavior is in scope |

### `workers/repository/update/pr/index › ensurePr › autoApprove`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates PR when autoApprove is set even if PR does not need updating | 1300 | pending | — | — | PR update runtime behavior is in scope |

---
