# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/labels.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/labels.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 20 | **Status:** pending

### `workers/repository/update/pr/labels › prepareLabels(config)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty array if no labels are configured | 11 | pending | — | — | — |
| only labels | 16 | pending | — | — | — |
| only addLabels | 22 | pending | — | — | — |
| merge labels and addLabels | 30 | pending | — | — | — |
| deduplicate merged labels and addLabels | 39 | pending | — | — | — |
| empty labels ignored | 48 | pending | — | — | — |
| null labels ignored | 57 | pending | — | — | — |
| template labels | 68 | pending | — | — | — |
| template labels with empty datasource | 77 | pending | — | — | — |

### `workers/repository/update/pr/labels › prepareLabels(config) › trim labels that go over the max char limit`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| github | 94 | pending | — | — | — |
| gitlab | 102 | pending | — | — | — |
| gitea | 115 | pending | — | — | — |

### `workers/repository/update/pr/labels › getChangedLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| adds new labels | 126 | pending | — | — | — |
| removes old labels | 133 | pending | — | — | — |

### `workers/repository/update/pr/labels › areLabelsModified`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true | 142 | pending | — | — | — |
| returns false | 146 | pending | — | — | — |

### `workers/repository/update/pr/labels › shouldUpdateLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true | 153 | pending | — | — | — |
| returns false if no labels found in debugData | 163 | pending | — | — | — |
| returns false if labels have been modified by user | 169 | pending | — | — | — |
| returns false if labels are not changed | 173 | pending | — | — | — |

---

