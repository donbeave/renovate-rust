# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/github/api-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/github/api-cache.spec.ts
**Total tests:** 15 | **Ported:** 0 | **Actionable:** 15 | **Status:** pending

### `tests`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| stores and retrieves items | 12 | pending | — | — | — |

### `getItems`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| maps items | 29 | pending | — | — | — |
| resets cache on item update | 46 | pending | — | — | — |
| resets cache on page reconcile | 69 | pending | — | — | — |

### `getLastModified`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined when no lastModified in cache | 94 | pending | — | — | — |
| returns stored value when present | 100 | pending | — | — | — |
| returns updated value after reconcile | 106 | pending | — | — | — |

### `updateLastModified`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| sets lastModified when not present | 116 | pending | — | — | — |
| advances lastModified to newer timestamp | 124 | pending | — | — | — |
| does not regress lastModified to older timestamp | 132 | pending | — | — | — |

### `reconcile`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false for empty page | 142 | pending | — | — | — |
| appends new items | 152 | pending | — | — | — |
| handles updated items | 175 | pending | — | — | — |
| ignores page overlap | 199 | pending | — | — | — |
| does not require new page if all items are old | 226 | pending | — | — | — |

---

