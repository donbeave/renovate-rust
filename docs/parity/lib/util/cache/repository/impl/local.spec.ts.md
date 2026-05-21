# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/util/cache/repository/impl/local.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/repository/impl/local.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/cache/repository/impl/local`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty object before any data load | 41 | not-applicable | — | — | tests local repository cache impl via Node.js fs; Rust would use own cache impl |
| skip when receives non-string data | 51 | not-applicable | — | — | tests local repository cache impl via Node.js fs; Rust would use own cache impl |
| should not load empty repository cache files | 65 | not-applicable | — | — | tests local repository cache impl via Node.js fs; Rust would use own cache impl |
| skip when not found | 80 | not-applicable | — | — | tests local repository cache impl via Node.js fs; Rust would use own cache impl |
| loads previously stored cache from disk | 91 | not-applicable | — | — | tests local repository cache impl via Node.js fs; Rust would use own cache impl |
| resets if fingerprint does not match | 107 | not-applicable | — | — | tests local repository cache impl via Node.js fs; Rust would use own cache impl |
| handles invalid data | 124 | not-applicable | — | — | tests local repository cache impl via Node.js fs; Rust would use own cache impl |
| handles file read error | 137 | not-applicable | — | — | tests local repository cache impl via Node.js fs; Rust would use own cache impl |
| handles invalid json | 152 | not-applicable | — | — | tests local repository cache impl via Node.js fs; Rust would use own cache impl |
| resets if repository does not match | 166 | not-applicable | — | — | tests local repository cache impl via Node.js fs; Rust would use own cache impl |
| saves modified cache data to file | 181 | not-applicable | — | — | tests local repository cache impl via Node.js fs; Rust would use own cache impl |
| does not write cache that is not changed | 213 | not-applicable | — | — | tests local repository cache impl via Node.js fs; Rust would use own cache impl |
| does not write cache when only key order has changed | 234 | not-applicable | — | — | tests local repository cache impl via Node.js fs; Rust would use own cache impl |

---

