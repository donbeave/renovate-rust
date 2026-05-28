# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/index.spec.ts
**Total tests:** 53 | **Ported:** 0 | **Actionable:** 53 | **Status:** not-applicable

### `workers/repository/update/pr/index › ensurePr › Create`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates PR | 78 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| fetches changelogs for the "pr" stage | 98 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| aborts PR creation once limit is exceeded | 109 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| ignores PR limits on vulnerability alert | 122 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| creates rollback PR | 135 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| skips PR creation due to non-green branch check | 146 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| creates PR for green branch checks | 158 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| skips PR creation for unapproved dependencies | 169 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| skips PR creation before prNotPendingHours is hit | 181 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| skips PR creation due to stabilityStatus | 201 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| creates PR after prNotPendingHours is hit | 221 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |

### `workers/repository/update/pr/index › ensurePr › Create › Error handling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles unknown error | 240 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| handles error for PR that already exists | 250 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| deletes branch on 502 error | 268 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |

### `workers/repository/update/pr/index › ensurePr › Update`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates PR if labels have changed in config | 285 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| skips pr update if existing pr does not have labels in debugData | 339 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| skips pr update if pr labels have been modified by user | 366 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| updates PR due to title change | 418 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| updates PR due to body change | 435 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| updates PR target branch if base branch changed in config | 455 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| ignores reviewable content | 483 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |

### `workers/repository/update/pr/index › ensurePr › dry-run`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| dry-runs PR creation | 512 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| dry-runs PR update | 530 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| skips automerge failure comment | 545 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |

### `workers/repository/update/pr/index › ensurePr › Automerge`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles branch automerge | 563 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| forces PR on dashboard check | 579 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| adds assignees for PR automerge with red status | 596 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| adds reviewers for PR automerge with red status and existing ignorable reviewers that can be ignored | 616 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| skips branch automerge and forces PR creation due to artifact errors | 637 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| skips branch automerge and forces PR creation due to prNotPendingHours exceeded | 653 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| automerges branch when prNotPendingHours are not exceeded | 674 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| comments on automerge failure | 697 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| handles ensureComment error | 720 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| logs unknown error | 737 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| re-throws ExternalHostError | 761 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| re-throws error with specific message: "$message" | 782 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |

### `workers/repository/update/pr/index › ensurePr › Changelog`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| processes changelogs | 854 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| handles missing GitHub token | 880 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| removes duplicate changelogs | 902 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| remove duplicates release notes | 928 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| stricter de-deuplication of changelogs | 958 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |

### `workers/repository/update/pr/index › ensurePr › Warnings › Attestations › when attestation is not removed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not warn the user | 1026 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |

### `workers/repository/update/pr/index › ensurePr › Warnings › Attestations › when attestation is removed`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| warns the user | 1056 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |

### `workers/repository/update/pr/index › ensurePr › Warnings › Attestations › when attestation is removed in an intermediate version`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not warn the user | 1104 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |

### `workers/repository/update/pr/index › ensurePr › prCache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds pr-cache when not present | 1128 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| does not update lastEdited pr-cache when pr fingerprint is same but pr was edited within 24hrs | 1144 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| updates pr-cache when pr fingerprint is different | 1171 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| skips fetching changelogs when cache is valid and pr was lastEdited before 24hrs | 1190 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| updates PR when rebase requested by user regardless of pr-cache state | 1215 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| logs when cache is enabled but pr-cache is absent | 1259 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| does not log when cache is disabled and pr-cache is absent | 1268 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |
| skips cache early return when autoApprove is set | 1278 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |

### `workers/repository/update/pr/index › ensurePr › autoApprove`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates PR when autoApprove is set even if PR does not need updating | 1300 | not-applicable | — | — | Requires vi.mock platform APIs + git + logger + GlobalConfig |

---
