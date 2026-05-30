# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/util/cache/package/backend.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/package/backend.spec.ts
**Total tests:** 10 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable-applicable

### `util/cache/package/backend`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns undefined when not initialized | 47 | not-applicable | — | — | Cache backend abstraction (redis/sqlite not implemented) |
| silently ignores set when not initialized | 52 | not-applicable | — | — | Cache backend abstraction (redis/sqlite not implemented) |
| silently ignores destroy when not initialized | 58 | not-applicable | — | — | Cache backend abstraction (redis/sqlite not implemented) |
| initializes file backend | 62 | not-applicable | — | — | Cache backend abstraction (redis/sqlite not implemented) |
| initializes redis backend | 69 | not-applicable | — | — | Cache backend abstraction (redis/sqlite not implemented) |
| initializes sqlite backend | 79 | not-applicable | — | — | Cache backend abstraction (redis/sqlite not implemented) |
| delegates get and set to backend instance | 88 | not-applicable | — | — | Cache backend abstraction (redis/sqlite not implemented) |
| re-init destroys previous backend | 105 | not-applicable | — | — | Cache backend abstraction (redis/sqlite not implemented) |
| clears backend when re-init has no config | 121 | not-applicable | — | — | Cache backend abstraction (redis/sqlite not implemented) |
| destroys backend and clears state | 132 | not-applicable | — | — | Cache backend abstraction (redis/sqlite not implemented) |

---

