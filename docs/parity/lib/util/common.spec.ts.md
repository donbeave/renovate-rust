# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/common.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/common.spec.ts
**Total tests:** 22 | **Ported:** 8 | **Actionable:** 22 | **Status:** partial

### `util/common › detectPlatform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ("$url") === $hostType | 46 | ported | `util.rs` | `test_detect_platform` | — |
| uses host rules | 67 | ported | `util.rs` | `test_detect_platform_uses_host_rules` | — |

### `util/common › parseJson`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null | 119 | ported | `util.rs` | `test_parse_json_null_for_empty` | — |
| returns parsed json | 123 | ported | `util.rs` | `test_parse_json_valid` | — |
| supports jsonc | 131 | ported | `util.rs` | `test_parse_json_jsonc` | — |
| throws error for invalid json | 149 | ported | `util.rs` | `test_parse_json_invalid` | — |
| catches and warns if content parsing failed with JSONC.parse but not with JSON5.parse | 153 | pending | — | — | — |
| does not warn if filename ends with .jsonc | 167 | pending | — | — | — |
| does not warn if filename ends with .json5 | 172 | pending | — | — | — |

### `util/common › parseJsonc`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns parsed jsonc | 179 | ported | `util.rs` | `test_parse_json_jsonc` | — |
| throws error for invalid jsonc | 187 | ported | `util.rs` | `test_parse_json_invalid` | — |

### `util/common › getInheritedOrGlobal`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined if not set | 198 | pending | — | — | — |
| returns inherited value if only inherited value is set | 202 | pending | — | — | — |
| returns global value if only global value is set | 209 | pending | — | — | — |
| returns inherited value - when both global + inherited are set | 216 | pending | — | — | — |
| handles null inherited values | 227 | pending | — | — | — |
| handles undefined inherited values | 238 | pending | — | — | — |

### `util/common › getInheritedOrGlobal › when requesting onboardingAutoCloseAge, do not allow inherit config to override global config`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns inherited value when inherited < global | 249 | pending | — | — | — |
| returns global value when inherited > global value | 259 | pending | — | — | — |
| returns inherited value when inherited == global | 269 | pending | — | — | — |
| returns inherited value when global value is not set | 279 | pending | — | — | — |
| returns global value when inherited value is not set | 289 | pending | — | — | — |

---
