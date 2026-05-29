# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/versioning/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/versioning/index.spec.ts
**Total tests:** 4 | **Ported:** 3 | **Actionable:** 3 | **Status:** done

### `modules/versioning/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return versioning list | 8 | ported | `versioning.rs` | `versioning_registry_get_list` | — |
| should fallback to semver-coerced | 12 | ported | `versioning.rs` | `versioning_registry_fallback` | — |
| should accept config | 18 | ported | `versioning.rs` | `versioning_registry_accept_config` | — |
| matches the API contract | 22 | not-applicable | — | — | TypeScript module system test; uses Zod schema + dynamic import to validate versioning API contract; TypeScript-specific reflection |

---
