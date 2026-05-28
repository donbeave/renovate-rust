# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/utils.spec.ts
**Total tests:** 6 | **Ported:** 2 | **Actionable:** 6 | **Status:** done

### `modules/datasource/utils`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is artifactory server invalid | 10 | ported | `util.rs` | `test_is_artifactory_server_invalid` | — |
| is artifactory server valid | 19 | ported | `util.rs` | `test_is_artifactory_server_valid` | — |
| retrieves a Google Access token | 28 | not-applicable | — | — | Uses vi.mock(google-auth-library) mock infrastructure; not portable |
| no Google Access token results in null | 42 | not-applicable | — | — | Uses vi.mock(google-auth-library) mock infrastructure; not portable |
| Google Access token error throws an exception | 56 | not-applicable | — | — | Uses vi.mock(google-auth-library) mock infrastructure; not portable |
| Google Access token could not load default credentials | 70 | not-applicable | — | — | Uses vi.mock(google-auth-library) mock infrastructure; not portable |

---

