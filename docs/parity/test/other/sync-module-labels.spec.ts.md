# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `test/other/sync-module-labels.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/test/other/sync-module-labels.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `other/sync-module-labels`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| creates module labels with the expected metadata | 11 | ported | `util.rs` | `test_create_module_label` | — |
| reports missing labels without flagging existing ones | 19 | ported | `util.rs` | `test_get_missing_module_labels` | — |
| renders stable label creation commands for missing labels | 36 | ported | `util.rs` | `test_format_create_label_commands` | — |
| includes labels for known runtime module ids | 62 | ported | `util.rs` | `test_get_expected_module_labels_contains_known_ids` | — |

---
