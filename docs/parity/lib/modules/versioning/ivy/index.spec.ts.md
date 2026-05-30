# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/versioning/ivy/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/ivy/index.spec.ts
**Total tests:** 9 | **Ported:** 9 | **Actionable:** 0 | **Status:** done

### `modules/versioning/ivy/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parseDynamicRevision("$input") === { type: "$type", value: "$value" } | 10 | ported | `versioning/ivy.rs` | `ivy_parse_dynamic_revision_ok` | — |
| parseDynamicRevision("$input") === null | 33 | ported | `versioning/ivy.rs` | `ivy_parse_dynamic_revision_null` | — |
| isValid("$input") === $expected | 43 | ported | `versioning/ivy.rs` | `ivy_is_valid` | — |
| isVersion("$input") === $expected | 72 | ported | `versioning/ivy.rs` | `ivy_is_version` | — |
| matches("$version", "$range") === $expected | 100 | ported | `versioning/ivy.rs` | `ivy_matches` | — |
| getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected" | 143 | ported | `versioning/ivy.rs` | `ivy_get_new_value` | — |
| getSatisfyingVersion($versions, "$range") === $expected | 160 | ported | `versioning/ivy.rs` | `ivy_get_satisfying_version` | — |
| isCompatible("$version") === $expected | 170 | ported | `versioning/ivy.rs` | `ivy_is_compatible` | — |
| isSingleVersion("$version") === $expected | 177 | ported | `versioning/ivy.rs` | `ivy_is_single_version` | — |

---
