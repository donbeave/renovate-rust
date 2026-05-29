# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/swift/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/swift/index.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `modules/versioning/swift/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| isVersion("$version") === $expected | 14 | ported | `versioning/swift.rs` | `swift_is_version` | — |
| isValid("$version") === $expected | 24 | ported | `versioning/swift.rs` | `swift_is_valid` | — |
| minSatisfyingVersion($versions, "$range") === "$expected" | 62 | ported | `versioning/swift.rs` | `swift_min_satisfying_version` | — |
| getSatisfyingVersion($versions, "$range") === "$expected" | 74 | ported | `versioning/swift.rs` | `swift_get_satisfying_version` | — |
| isLessThanRange("$version", "$range") === "$expected" | 87 | ported | `versioning/swift.rs` | `swift_is_less_than_range` | — |
| matches("$version", "$range") === "$expected" | 101 | ported | `versioning/swift.rs` | `swift_matches` | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 117 | ported | `versioning/swift.rs` | `swift_get_new_value` | — |

---
