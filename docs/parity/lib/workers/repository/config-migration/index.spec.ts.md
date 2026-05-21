# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/config-migration/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/config-migration/index.spec.ts
**Total tests:** 6 | **Ported:** 0 | **Actionable:** 6 | **Status:** not-applicable

### `workers/repository/config-migration/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does nothing when in silent mode | 32 | not-applicable | — | — | tests config migration orchestration via git/platform; out of scope |
| skips pr creation when migration is not needed | 40 | not-applicable | — | — | tests config migration orchestration via git/platform; out of scope |
| creates migration pr if needed | 49 | not-applicable | — | — | tests config migration orchestration via git/platform; out of scope |
| returns add-checkbox if migration pr exists but is created by another user | 64 | not-applicable | — | — | tests config migration orchestration via git/platform; out of scope |
| returns pr-modified incase the migration pr has been modified | 77 | not-applicable | — | — | tests config migration orchestration via git/platform; out of scope |
| returns add-checkbox if migration is needed but not demanded | 94 | not-applicable | — | — | tests config migration orchestration via git/platform; out of scope |

---

