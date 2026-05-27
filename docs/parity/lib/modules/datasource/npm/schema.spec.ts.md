# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/npm/schema.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/npm/schema.spec.ts
**Total tests:** 1 | **Ported:** 0 | **Actionable:** 1 | **Status:** done

### `modules/datasource/npm/schema`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| strips fields outside the cached packument shape | 4 | not-applicable | — | — | Rust `NpmPackument` deserializes only required fields (versions, dist-tags, time); no separate CachedPackument abstraction — serde skips unknown fields by default |

---

