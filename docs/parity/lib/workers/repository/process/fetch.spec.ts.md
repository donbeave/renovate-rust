# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/fetch.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/fetch.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 13 | **Status:** done

### `workers/repository/process/fetch › fetchUpdates()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty deps | 21 | not-applicable | — | — | Requires vi.mock datasource/lookup mock infrastructure |
| handles ignored, skipped and disabled | 31 | not-applicable | — | — | Requires vi.mock datasource/lookup mock infrastructure |
| fetches updates | 85 | not-applicable | — | — | Requires vi.mock datasource/lookup mock infrastructure |

### `workers/repository/process/fetch › fetchUpdates() › constraintsVersioning`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is merged from packageFile with config | 119 | not-applicable | — | — | Requires vi.mock datasource/lookup mock infrastructure |
| is set from packageFile if only set on packageFile | 147 | not-applicable | — | — | Requires vi.mock datasource/lookup mock infrastructure |
| is not set if neither config nor packageFile are set | 168 | not-applicable | — | — | Requires vi.mock datasource/lookup mock infrastructure |
| is set if config is set | 189 | not-applicable | — | — | Requires vi.mock datasource/lookup mock infrastructure |
| skips deps with empty names | 211 | not-applicable | — | — | Requires vi.mock datasource/lookup mock infrastructure |
| skips internal deps by default | 238 | not-applicable | — | — | Requires vi.mock datasource/lookup mock infrastructure |
| fetch updates for internal deps if updateInternalDeps is true | 261 | not-applicable | — | — | Requires vi.mock datasource/lookup mock infrastructure |
| throws lookup errors for onboarded repos | 283 | not-applicable | — | — | Requires vi.mock datasource/lookup mock infrastructure |
| throws lookup errors for not onboarded repos | 300 | not-applicable | — | — | Requires vi.mock datasource/lookup mock infrastructure |
| produces external host warnings for not onboarded repos | 317 | not-applicable | — | — | Requires vi.mock datasource/lookup mock infrastructure |

---

