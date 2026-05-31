# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/sample.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/sample.spec.ts
**Total tests:** 7 | **Ported:** 5 | **Actionable:** 2 | **Status:** pending

### `util/sample › sampleSize`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns correct sized array | 7 | ported | `util.rs` | `test_sample_size_correct` | — |
| returns full array for undefined number | 12 | ported | `util.rs` | `test_sample_size_none_n` | — |
| returns full array for null number | 16 | pending | — | — | TypeScript type-system test; null vs undefined distinction has no Rust equivalent (both are None) |
| returns full array for 0 number | 20 | ported | `util.rs` | `test_sample_size_zero_n` | — |
| returns empty array for null array | 24 | pending | — | — | TypeScript type-system test; null vs undefined distinction has no Rust equivalent (both are None) |
| returns empty array for undefined array | 28 | ported | `util.rs` | `test_sample_size_empty_arr` | — |
| returns empty array for empty array | 32 | ported | `util.rs` | `test_sample_size_empty_arr` | — |

---

