# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/hash.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/hash.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `util/hash`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| hashes data with sha256 | 6 | ported | `util.rs` | `test_hash_sha256` | — |
| hashes data with sha512 | 15 | ported | `util.rs` | `test_hash_sha512` | — |
| correctly hashes the content of a readable stream | 21 | ported | `util.rs` | `test_hash_stream_sha256` | — |
| uses sha512 if no algorithm is specified | 38 | ported | `util.rs` | `test_hash_stream_default_sha512` | — |

---

