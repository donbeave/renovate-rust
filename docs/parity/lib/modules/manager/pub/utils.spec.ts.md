# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/pub/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/pub/utils.spec.ts
**Total tests:** 6 | **Ported:** 6 | **Actionable:** 6 | **Status:** ported

### `parsePubspec`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| load and parse successfully | 14 | ported | `pubspec.rs` | `parse_pubspec_loads_valid_yaml` | — |
| invalid yaml | 32 | ported | `pubspec.rs` | `parse_pubspec_invalid_yaml_returns_none` | — |
| invalid schema | 37 | ported | `pubspec.rs` | `parse_pubspec_invalid_schema_returns_none` | — |

### `parsePubspeckLock`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| load and parse successfully | 44 | ported | `pubspec.rs` | `parse_pubspec_lock_loads_valid_yaml` | — |
| invalid yaml | 56 | ported | `pubspec.rs` | `parse_pubspec_lock_invalid_yaml_returns_none` | — |
| invalid schema | 61 | ported | `pubspec.rs` | `parse_pubspec_lock_invalid_schema_returns_none` | — |

---

