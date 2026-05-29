# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/workers/repository/process/fetch.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/fetch.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 13 | **Status:** not-applicable

### `workers/repository/process/fetch › fetchUpdates()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| handles empty deps | 21 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| handles ignored, skipped and disabled | 31 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| fetches updates | 85 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|

### `workers/repository/process/fetch › fetchUpdates() › constraintsVersioning`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| is merged from packageFile with config | 119 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| is set from packageFile if only set on packageFile | 147 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| is not set if neither config nor packageFile are set | 168 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| is set if config is set | 189 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| skips deps with empty names | 211 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| skips internal deps by default | 238 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| fetch updates for internal deps if updateInternalDeps is true | 261 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| throws lookup errors for onboarded repos | 283 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| throws lookup errors for not onboarded repos | 300 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|
| produces external host warnings for not onboarded repos | 317 | not-applicable | — | — | mocking framework internals — vi.mock/vi.spyOn on TypeScript module subsystems; tests TypeScript pipeline coordination|

---

