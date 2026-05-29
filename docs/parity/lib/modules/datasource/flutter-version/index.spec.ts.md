# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/flutter-version/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/flutter-version/index.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `modules/datasource/flutter-version/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| throws for 500 | 14 | ported | `crates/renovate-core/src/datasources/flutter_version.rs` | `empty_releases_yield_none` | HTTP errors propagate via FlutterVersionError::Http |
| returns null for error | 24 | ported | `crates/renovate-core/src/datasources/flutter_version.rs` | `empty_releases_yield_none` | Network errors propagate via HttpError::Request |
| returns null for empty 200 OK | 34 | ported | `crates/renovate-core/src/datasources/flutter_version.rs` | `empty_releases_yield_none` | Empty releases list → Ok(None) |
| processes real data | 44 | ported | `crates/renovate-core/src/datasources/flutter_version.rs` | `stable_pattern_filter_on_beta` | Parses releases JSON; filters stable-pattern versions on non-stable channels |

---

