# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/labels.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/labels.spec.ts
**Total tests:** 20 | **Ported:** 14 | **Actionable:** 20 | **Status:** partial-applicable

### `workers/repository/update/pr/labels › prepareLabels(config)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty array if no labels are configured | 11 | ported | `util.rs` | `test_prepare_labels_empty` | — |
| only labels | 16 | ported | `util.rs` | `test_prepare_labels_only_labels` | — |
| only addLabels | 22 | ported | `util.rs` | `test_prepare_labels_only_add_labels` | — |
| merge labels and addLabels | 30 | ported | `util.rs` | `test_prepare_labels_merge` | — |
| deduplicate merged labels and addLabels | 39 | ported | `util.rs` | `test_prepare_labels_deduplicate` | — |
| empty labels ignored | 48 | ported | `util.rs` | `test_prepare_labels_empty_strings_ignored` | — |
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
| adds new labels | 126 | ported | `util.rs` | `test_get_changed_labels_add` | — |
| removes old labels | 133 | ported | `util.rs` | `test_get_changed_labels_remove` | — |

### `workers/repository/update/pr/labels › areLabelsModified`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true | 142 | ported | `util.rs` | `test_are_labels_modified_true` | — |
| returns false | 146 | ported | `util.rs` | `test_are_labels_modified_false` | — |

### `workers/repository/update/pr/labels › shouldUpdateLabels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true | 153 | ported | `util.rs` | `test_should_update_labels_true` | — |
| returns false if no labels found in debugData | 163 | ported | `util.rs` | `test_should_update_labels_false_no_initial` | — |
| returns false if labels have been modified by user | 169 | ported | `util.rs` | `test_should_update_labels_false_user_modified` | — |
| returns false if labels are not changed | 173 | ported | `util.rs` | `test_should_update_labels_false_unchanged` | — |

---

