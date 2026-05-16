# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/util/cache/package/impl/file.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/package/impl/file.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 16 | **Status:** pending

### `util/cache/package/impl/file › basic operations`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets and gets | 26 | pending | — | — | — |
| stores payload with value and expiry | 34 | pending | — | — | — |

### `util/cache/package/impl/file › get`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined on cache miss | 47 | pending | — | — | — |
| expires cached entries | 53 | pending | — | — | — |
| returns undefined for null cached value | 65 | pending | — | — | — |
| returns undefined for invalid JSON | 73 | pending | — | — | — |
| returns undefined for corrupted cache payload | 81 | pending | — | — | — |
| returns undefined for missing expiry | 93 | pending | — | — | — |
| returns undefined for invalid expiry | 102 | pending | — | — | — |
| retrieves value from cache payload | 114 | pending | — | — | — |

### `util/cache/package/impl/file › destroy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| removes expired and invalid entries | 127 | pending | — | — | — |
| keeps entries without expiry field | 148 | pending | — | — | — |
| removes entries with invalid expiry | 158 | pending | — | — | — |
| continues on cleanup errors | 171 | pending | — | — | — |
| skips disk read for entry written this run | 183 | pending | — | — | — |
| skips disk read for expired entry written this run | 197 | pending | — | — | — |

---

