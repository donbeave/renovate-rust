# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/manager/maven/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/maven/index.spec.ts
**Total tests:** 8 | **Ported:** 5 | **Actionable:** 5 | **Status:** partial

### `updateDependency`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should update an existing dependency | 26 | ported | `extractors/maven.rs` | `maven_index_update_existing_dependency` | — |
| should update existing dependency defined via properties | 43 | not-applicable | — | — | `resolveParents` multi-POM inheritance not implemented in Rust |
| should not touch content if new and old versions are equal | 67 | ported | `extractors/maven.rs` | `maven_index_no_touch_when_equal` | — |
| should update to version of the latest dep in implicit group | 79 | not-applicable | — | — | Implicit group detection via shared properties not implemented |
| should return null for ungrouped deps if content was updated outside | 135 | not-applicable | — | — | Implicit group detection not implemented |
| should return null if current versions in content and upgrade are not same | 150 | ported | `extractors/maven.rs` | `maven_index_returns_none_when_current_mismatch` | — |
| should update ranges | 162 | ported | `extractors/maven.rs` | `maven_index_update_ranges` | — |
| should preserve ranges | 181 | ported | `extractors/maven.rs` | `maven_index_preserve_ranges` | — |

---
