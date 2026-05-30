# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/datasource/datasource.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/datasource.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 2 | **Status:** done

### `modules/datasource/datasource`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should throw on 429 | 24 | not-applicable | — | — | TypeScript base class test — tests abstract Datasource class error handling via handleGenericErrors(); Rust datasources use HttpClient directly, error handling tested in individual datasource tests and HTTP layer (retry-after.spec.ts) |
| should throw on statusCode >=500 && <600 | 35 | not-applicable | — | — | TypeScript base class test — tests abstract Datasource class error handling via handleGenericErrors(); Rust datasources use HttpClient directly, error handling tested in individual datasource tests and HTTP layer (retry-after.spec.ts) |

---

