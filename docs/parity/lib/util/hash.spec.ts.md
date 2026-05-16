# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/hash.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/hash.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/hash`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| hashes data with sha256 | 6 | not-applicable | — | — | Renovate's generic string hash helper is not implemented as a shared Rust API; Rust hashing is local to call sites such as branch naming. |
| hashes data with sha512 | 15 | not-applicable | — | — | Renovate's generic string hash helper is not implemented as a shared Rust API; Rust hashing is local to call sites such as branch naming. |
| correctly hashes the content of a readable stream | 21 | not-applicable | — | — | Renovate's Node readable-stream hashing helper has no Rust API equivalent. |
| uses sha512 if no algorithm is specified | 38 | not-applicable | — | — | Renovate's Node readable-stream hashing helper has no Rust API equivalent. |

---

