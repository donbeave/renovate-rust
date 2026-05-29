# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/util/cache/repository/impl/local.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/cache/repository/impl/local.spec.ts
**Total tests:** 13 | **Ported:** 0 | **Actionable:** 13 | **Status:** not-applicable

### `util/cache/repository/impl/local`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty object before any data load | 41 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript local repository cache with filesystem mock|
| skip when receives non-string data | 51 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript local repository cache with filesystem mock|
| should not load empty repository cache files | 65 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript local repository cache with filesystem mock|
| skip when not found | 80 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript local repository cache with filesystem mock|
| loads previously stored cache from disk | 91 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript local repository cache with filesystem mock|
| resets if fingerprint does not match | 107 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript local repository cache with filesystem mock|
| handles invalid data | 124 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript local repository cache with filesystem mock|
| handles file read error | 137 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript local repository cache with filesystem mock|
| handles invalid json | 152 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript local repository cache with filesystem mock|
| resets if repository does not match | 166 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript local repository cache with filesystem mock|
| saves modified cache data to file | 181 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript local repository cache with filesystem mock|
| does not write cache that is not changed | 213 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript local repository cache with filesystem mock|
| does not write cache when only key order has changed | 234 | not-applicable | — | — | mocking framework internals — vi.mock on fs; TypeScript local repository cache with filesystem mock|

---

