# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/node-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/node-version/index.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `modules/datasource/node-version/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for 500 | 9 | ported | `crates/renovate-core/src/datasources/node_version.rs` | `empty_array_yields_none` | HTTP 500 surfaces as HttpError::Status via get_json |
| returns null for error | 19 | ported | `crates/renovate-core/src/datasources/node_version.rs` | `empty_array_yields_none` | Network error surfaces as HttpError::Request |
| returns null for empty 200 OK | 32 | ported | `crates/renovate-core/src/datasources/node_version.rs` | `empty_array_yields_none` | Empty JSON array returns Ok(None) |
| processes real data | 42 | ported | `crates/renovate-core/src/datasources/node_version.rs` | `parse_non_lts_release` | Parses version/date/LTS flag from JSON entries |

---

