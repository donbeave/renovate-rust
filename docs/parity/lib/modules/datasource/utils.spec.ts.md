# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/utils.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/utils.spec.ts
**Total tests:** 6 | **Ported:** 2 | **Actionable:** 0 | **Status:** done

### `modules/datasource/utils`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is artifactory server invalid | 10 | ported | `util.rs` | `test_is_artifactory_server_invalid` | — |
| is artifactory server valid | 19 | ported | `util.rs` | `test_is_artifactory_server_valid` | — |
| retrieves a Google Access token | 28 | not-applicable | — | — | TS-library-specific; uses vi.mock('google-auth-library') to mock Google auth; Rust would use google-auth2 or similar with different API |
| no Google Access token results in null | 42 | not-applicable | — | — | TS-library-specific; mocks google-auth-library; Rust handles GCP auth differently |
| Google Access token error throws an exception | 56 | not-applicable | — | — | TS-library-specific; mocks google-auth-library error paths |
| Google Access token could not load default credentials | 70 | not-applicable | — | — | TS-library-specific; mocks google-auth-library credential loading |

---

