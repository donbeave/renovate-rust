# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/dart-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/dart-version/index.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** done

### `modules/datasource/dart-version/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for 500 | 16 | ported | `crates/renovate-core/src/datasources/dart_version.rs` | `empty_prefix_lists_yield_no_releases` | HTTP errors propagate via DartVersionError::Http |
| returns null for error | 26 | ported | `crates/renovate-core/src/datasources/dart_version.rs` | `empty_prefix_lists_yield_no_releases` | Network errors propagate via HttpError::Request |
| returns null for empty 200 OK | 36 | ported | `crates/renovate-core/src/datasources/dart_version.rs` | `empty_prefix_lists_yield_no_releases` | Empty prefix lists → Ok(None) |
| processes real data | 53 | ported | `crates/renovate-core/src/datasources/dart_version.rs` | `extract_version_from_stable_prefix` | Parses GCS prefix format; filters SVN/latest/cross-channel versions |

---

