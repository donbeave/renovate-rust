# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/mutex.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/mutex.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 3 | **Status:** not-applicable-applicable-applicable

### `util/mutex › getMutex`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns mutex with default namespace | 7 | not-applicable | — | — | TS-library-specific; tests async-mutex npm package via getMutex/acquireLock API; Rust uses tokio::sync::Mutex directly|

### `util/mutex › acquireLock`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return lock function with default namespace | 17 | not-applicable | — | — | TS-library-specific; tests async-mutex npm package via getMutex/acquireLock API; Rust uses tokio::sync::Mutex directly|
| should lock if already used | 21 | not-applicable | — | — | TS-library-specific; tests async-mutex npm package via getMutex/acquireLock API; Rust uses tokio::sync::Mutex directly|

---

