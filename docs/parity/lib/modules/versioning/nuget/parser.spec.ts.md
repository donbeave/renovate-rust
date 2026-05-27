# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/nuget/parser.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/nuget/parser.spec.ts
**Total tests:** 15 | **Ported:** 15 | **Actionable:** 15 | **Status:** done

### `modules/versioning/nuget/parser › parseVersion`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid input | 13 | ported | `versioning/nuget.rs` | `parse_version_rejects_invalid_input` | — |
| parses version | 18 | ported | `versioning/nuget.rs` | `parse_version_parses_full_version` | — |

### `modules/versioning/nuget/parser › parseFloatingRange`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rejects invalid input | 32 | ported | `versioning/nuget.rs` | `parse_floating_range_rejects_invalid_input` | — |
| $input | 39 | ported | `versioning/nuget.rs` | `parse_floating_range_parametrized` | — |

### `modules/versioning/nuget/parser › getFloatingRangeLowerBound`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $input | 78 | ported | `versioning/nuget.rs` | `get_floating_range_lower_bound_parametrized` | — |

### `modules/versioning/nuget/parser › parseExactRange`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rejects invalid input | 115 | ported | `versioning/nuget.rs` | `parse_exact_range_rejects_invalid_input` | — |
| parses exact range | 123 | ported | `versioning/nuget.rs` | `parse_exact_range_parses` | — |

### `modules/versioning/nuget/parser › parseBracketRange`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| rejects invalid input | 137 | ported | `versioning/nuget.rs` | `parse_bracket_range_rejects_invalid_input` | — |
| parses range without lower bound | 147 | ported | `versioning/nuget.rs` | `parse_bracket_range_no_lower_bound` | — |
| parses range without upper bound | 157 | ported | `versioning/nuget.rs` | `parse_bracket_range_no_upper_bound` | — |
| $input | 168 | ported | `versioning/nuget.rs` | `parse_bracket_range_bounds_inclusivity` | — |
| handles whitespaces | 185 | ported | `versioning/nuget.rs` | `parse_bracket_range_handles_whitespace` | — |
| handles floating ranges as lower bounds | 195 | ported | `versioning/nuget.rs` | `parse_bracket_range_floating_lower_bound` | — |

### `modules/versioning/nuget/parser › versionToString`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $version | 224 | ported | `versioning/nuget.rs` | `version_to_string_roundtrip` | — |

### `modules/versioning/nuget/parser › rangeToString`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $version | 242 | ported | `versioning/nuget.rs` | `range_to_string_roundtrip` | — |

---
