# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/util/cache/package/impl/sqlite.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/package/impl/sqlite.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 12 | **Status:** not-applicable

### `util/cache/package/impl/sqlite › get`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined on cache miss | 53 | not-applicable | — | — | TS-library-specific; uses better-sqlite3 npm package; TypeScript SQLite cache implementation|
| returns undefined for invalid compressed payload | 62 | not-applicable | — | — | TS-library-specific; uses better-sqlite3 npm package; TypeScript SQLite cache implementation|
| returns undefined for invalid JSON payload | 77 | not-applicable | — | — | TS-library-specific; uses better-sqlite3 npm package; TypeScript SQLite cache implementation|
| returns undefined when the read fails | 93 | not-applicable | — | — | TS-library-specific; uses better-sqlite3 npm package; TypeScript SQLite cache implementation|

### `util/cache/package/impl/sqlite › set`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| logs a warning and continues when serialization fails | 120 | not-applicable | — | — | TS-library-specific; uses better-sqlite3 npm package; TypeScript SQLite cache implementation|
| logs a warning and continues when the write fails | 137 | not-applicable | — | — | TS-library-specific; uses better-sqlite3 npm package; TypeScript SQLite cache implementation|

### `util/cache/package/impl/sqlite › set and get`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| overwrites and returns latest value | 161 | not-applicable | — | — | TS-library-specific; uses better-sqlite3 npm package; TypeScript SQLite cache implementation|

### `util/cache/package/impl/sqlite › expiry`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined for immediately expired entry | 174 | not-applicable | — | — | TS-library-specific; uses better-sqlite3 npm package; TypeScript SQLite cache implementation|

### `util/cache/package/impl/sqlite › destroy`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes expired entries and closes database | 185 | not-applicable | — | — | TS-library-specific; uses better-sqlite3 npm package; TypeScript SQLite cache implementation|
| resolves and still closes when cleanup throws | 204 | not-applicable | — | — | TS-library-specific; uses better-sqlite3 npm package; TypeScript SQLite cache implementation|
| resolves when close throws | 219 | not-applicable | — | — | TS-library-specific; uses better-sqlite3 npm package; TypeScript SQLite cache implementation|

### `util/cache/package/impl/sqlite › persistence`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| retrieves value from persistent storage after reopening | 245 | not-applicable | — | — | TS-library-specific; uses better-sqlite3 npm package; TypeScript SQLite cache implementation|

---

