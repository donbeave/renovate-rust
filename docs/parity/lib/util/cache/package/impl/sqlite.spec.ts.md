# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/util/cache/package/impl/sqlite.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/package/impl/sqlite.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 12 | **Status:** pending

### `util/cache/package/impl/sqlite › get`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined on cache miss | 53 | pending | — | — | — |
| returns undefined for invalid compressed payload | 62 | pending | — | — | — |
| returns undefined for invalid JSON payload | 77 | pending | — | — | — |
| returns undefined when the read fails | 93 | pending | — | — | — |

### `util/cache/package/impl/sqlite › set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs a warning and continues when serialization fails | 120 | pending | — | — | —|
| logs a warning and continues when the write fails | 137 | pending | — | — | —|

### `util/cache/package/impl/sqlite › set and get`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| overwrites and returns latest value | 161 | pending | — | — | — |

### `util/cache/package/impl/sqlite › expiry`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined for immediately expired entry | 174 | pending | — | — | — |

### `util/cache/package/impl/sqlite › destroy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes expired entries and closes database | 185 | pending | — | — | — |
| resolves and still closes when cleanup throws | 204 | pending | — | — | — |
| resolves when close throws | 219 | pending | — | — | — |

### `util/cache/package/impl/sqlite › persistence`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| retrieves value from persistent storage after reopening | 245 | pending | — | — | — |

---

