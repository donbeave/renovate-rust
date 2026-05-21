# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/util/cache/package/impl/file.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/package/impl/file.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/cache/package/impl/file › basic operations`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets and gets | 26 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |
| stores payload with value and expiry | 34 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |

### `util/cache/package/impl/file › get`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined on cache miss | 47 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |
| expires cached entries | 53 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |
| returns undefined for null cached value | 65 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |
| returns undefined for invalid JSON | 73 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |
| returns undefined for corrupted cache payload | 81 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |
| returns undefined for missing expiry | 93 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |
| returns undefined for invalid expiry | 102 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |
| retrieves value from cache payload | 114 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |

### `util/cache/package/impl/file › destroy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| removes expired and invalid entries | 127 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |
| keeps entries without expiry field | 148 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |
| removes entries with invalid expiry | 158 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |
| continues on cleanup errors | 171 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |
| skips disk read for entry written this run | 183 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |
| skips disk read for expired entry written this run | 197 | not-applicable | — | — | tests file-based package cache backend via Node.js fs; Rust would use own cache impl |

---

