# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/bazel/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/bazel/common.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** done

### `updateCode`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns input for invalid | 5 | ported | `extractors/bazel_parser.rs` | `update_code_returns_input_for_invalid` | — |
| replaces whole rule | 11 | ported | `extractors/bazel_parser.rs` | `update_code_replaces_whole_rule` | — |
| replaces rule key | 17 | ported | `extractors/bazel_parser.rs` | `update_code_replaces_rule_key` | — |
| returns input on wrong index | 23 | ported | `extractors/bazel_parser.rs` | `update_code_wrong_index` | — |
| returns input on wrong key | 29 | ported | `extractors/bazel_parser.rs` | `update_code_wrong_key` | — |
| replaces array values | 35 | ported | `extractors/bazel_parser.rs` | `update_code_replaces_array_value` | — |
| updates using function | 43 | ported | `extractors/bazel_parser.rs` | `update_code_updater_function_equivalent` | JS function → Rust closure simulation |

---
