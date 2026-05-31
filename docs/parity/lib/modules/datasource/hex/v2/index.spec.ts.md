# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/datasource/hex/v2/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/hex/v2/index.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** done

### `modules/datasource/hex/v2/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| roundtrip | 34 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | Hex.pm v2 protobuf API (`Signed`/`Package` types) not implemented; Rust hex datasource uses v1 REST API only |
| roundtrip | 55 | not-applicable | Mock framework internals — tests hex datasource via nock HTTP mocks; Rust tests this at different layer | — | Same reason as line 34 |

---

