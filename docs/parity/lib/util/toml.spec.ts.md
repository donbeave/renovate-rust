# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/toml.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/toml.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/toml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 5 | not-applicable | — | — | Renovate's TypeScript shared TOML parse wrapper is not implemented as a Rust API; Rust parses TOML inside individual extractors. |
| handles invalid toml | 24 | not-applicable | — | — | Renovate's TypeScript shared TOML parse wrapper error contract is not implemented as a Rust API; Rust parses TOML inside individual extractors. |
| handles templates | 32 | not-applicable | — | — | Renovate's TypeScript TOML template massaging helper is not implemented as a Rust API. |

---

