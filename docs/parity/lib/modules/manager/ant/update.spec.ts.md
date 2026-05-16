# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/ant/update.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ant/update.spec.ts
**Total tests:** 10 | **Ported:** 10 | **Actionable:** 10 | **Status:** ported

### `modules/manager/ant/update`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| updates inline XML version attribute | 4 | ported | `ant.rs` | `update_inline_xml_version_attribute` | — |
| updates single-quoted XML version attribute | 23 | ported | `ant.rs` | `update_single_quoted_xml_version_attribute` | — |
| updates .properties file value | 42 | ported | `ant.rs` | `update_properties_file_value` | — |
| updates .properties value at end of file without trailing newline | 58 | ported | `ant.rs` | `update_properties_value_at_eof_without_trailing_newline` | — |
| returns fileContent unchanged when already updated | 74 | ported | `ant.rs` | `update_returns_file_content_unchanged_when_already_updated` | — |
| updates when sharedVariableName is set even if currentValue differs | 91 | ported | `ant.rs` | `update_shared_variable_even_when_current_value_differs` | — |
| returns null when fileReplacePosition is undefined | 108 | ported | `ant.rs` | `update_returns_none_when_file_replace_position_is_missing` | — |
| updates version within coords attribute | 122 | ported | `ant.rs` | `update_version_within_coords_attribute` | — |
| updates version within 4-part coords attribute | 140 | ported | `ant.rs` | `update_version_within_four_part_coords_attribute` | — |
| returns null when value at position does not match | 158 | ported | `ant.rs` | `update_returns_none_when_value_at_position_does_not_match` | — |

---

