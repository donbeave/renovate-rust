# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/labels.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/labels.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `workers/repository/update/pr/labels › prepareLabels(config)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty array if no labels are configured | 11 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |
| only labels | 16 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |
| only addLabels | 22 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |
| merge labels and addLabels | 30 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |
| deduplicate merged labels and addLabels | 39 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |
| empty labels ignored | 48 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |
| null labels ignored | 57 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |
| template labels | 68 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |
| template labels with empty datasource | 77 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |

### `workers/repository/update/pr/labels › prepareLabels(config) › trim labels that go over the max char limit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| github | 94 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |
| gitlab | 102 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |
| gitea | 115 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |

### `workers/repository/update/pr/labels › getChangedLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds new labels | 126 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |
| removes old labels | 133 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |

### `workers/repository/update/pr/labels › areLabelsModified`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true | 142 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |
| returns false | 146 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |

### `workers/repository/update/pr/labels › shouldUpdateLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true | 153 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |
| returns false if no labels found in debugData | 163 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |
| returns false if labels have been modified by user | 169 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |
| returns false if labels are not changed | 173 | not-applicable | — | — | tests PR label management via platform API; platform interactions out of scope |

---
