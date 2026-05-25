# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/cpan/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/cpan/index.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** done

### `modules/datasource/cpan/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for empty result | 11 | ported | `crates/renovate-core/src/datasources/cpan.rs` | `returns_null_for_empty_result` | Empty hits array → None |
| returns null for 404 | 27 | ported | `crates/renovate-core/src/datasources/cpan.rs` | `returns_null_for_404` | 404 → None |
| throws for 5xx | 37 | ported | `crates/renovate-core/src/datasources/cpan.rs` | `throws_for_5xx` | 502 → Err |
| returns null for unknown error | 47 | ported | `crates/renovate-core/src/datasources/cpan.rs` | `returns_null_for_unknown_error` | Invalid JSON → parse error → None |
| processes real data | 57 | ported | `crates/renovate-core/src/datasources/cpan.rs` | `processes_real_data` | Plack.json fixture → 10 releases, changelog/homepage URLs, tags.latest |

---
