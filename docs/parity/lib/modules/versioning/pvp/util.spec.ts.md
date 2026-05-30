# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/pvp/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/pvp/util.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `modules/versioning/pvp/util`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null when there are no numbers | 5 | ported | `pvp.rs` | `extract_all_parts_returns_none_when_there_are_no_numbers` | — |
| should parse 3.0 | 9 | ported | `pvp.rs` | `extract_all_parts_parses_numeric_components` | — |
| "0" is valid major version | 15 | ported | `pvp.rs` | `get_parts_accepts_zero_major_version` | — |
| returns null when no parts could be extracted | 19 | ported | `pvp.rs` | `get_parts_returns_none_when_no_parts_can_be_extracted` | — |

---

