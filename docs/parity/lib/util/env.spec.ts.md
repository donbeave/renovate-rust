# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/env.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/env.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `util/env › getEnv`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return combined env | 11 | not-applicable | — | — | Uses JavaScript process.env + memory cache state; Rust manages env via std::env without layered JS-specific state |
| maintains precendence | 26 | not-applicable | — | — | Uses JavaScript process.env + memory cache state; Rust manages env via std::env without layered JS-specific state |

---

