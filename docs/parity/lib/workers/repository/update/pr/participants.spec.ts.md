# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/participants.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/participants.spec.ts
**Total tests:** 18 | **Ported:** 0 | **Actionable:** 18 | **Status:** not-applicable

### `workers/repository/update/pr/participants › assignees`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not assignees when there are none | 31 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| adds assignees | 36 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| filters assignees | 45 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| expands group code owners assignees | 56 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| does not expand group code owners assignees when assigneesFromCodeOwners disabled | 91 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| does not expand group code owners assignees when expandCodeOwnersGroups disabled | 106 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| supports assigneesSampleSize | 125 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| handles add assignee errors | 134 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| supports dry run assignee adding | 139 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| supports assigneesFromCodeOwners | 145 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |

### `workers/repository/update/pr/participants › reviewers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not assignees when there are none | 160 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| adds reviewers | 165 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| handles add assignee errors | 174 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| supports reviewersSampleSize | 179 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| supports dry run assignee adding | 188 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| supports reviewersFromCodeOwners | 194 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| filters out bare @ from malformed CODEOWNERS entries | 207 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |
| supports additionalReviewers | 223 | not-applicable | — | — | Uses platform mock + vi.mock(sample/codeOwners) + GlobalConfig; platform mock infrastructure not portable to Rust |

---
