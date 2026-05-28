# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/queue.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/queue.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `util/http/queue`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for invalid URL | 14 | not-applicable | — | — | JavaScript p-queue concurrency management; Rust uses tokio semaphores directly |
| returns queue for valid url | 18 | not-applicable | — | — | JavaScript p-queue concurrency management; Rust uses tokio semaphores directly |

---

