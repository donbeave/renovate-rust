# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/ant/properties.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/ant/properties.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 0 | **Status:** done

### `modules/manager/ant/properties › parsePropertiesFile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses key=value pairs | 6 | ported | `ant.rs` | `properties_file_parses_key_value_pairs` | — |
| skips comments and blank lines | 28 | ported | `ant.rs` | `properties_file_skips_comments_and_blank_lines` | — |
| supports colon separator | 39 | ported | `ant.rs` | `properties_file_supports_colon_separator` | — |
| skips malformed lines without separators | 46 | ported | `ant.rs` | `properties_file_skips_malformed_lines_without_separators` | — |
| implements first-definition-wins | 57 | ported | `ant.rs` | `properties_file_implements_first_definition_wins` | — |
| respects pre-existing props (first-definition-wins across sources) | 64 | ported | `ant.rs` | `properties_file_respects_pre_existing_props_across_sources` | — |

---

