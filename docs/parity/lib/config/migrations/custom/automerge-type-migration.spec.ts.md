# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/automerge-type-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/automerge-type-migration.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `config/migrations/custom/automerge-type-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate string like "branch-" to "branch" | 4 | not-applicable | — | — | Rust preserves typed `automergeType` values and does not expose Renovate's raw automergeType string cleanup migration output |
| should not migrate another string value | 14 | not-applicable | — | — | Rust preserves typed `automergeType` values and does not expose Renovate's raw automergeType string cleanup migration output |
| should not migrate non string value | 25 | not-applicable | — | — | Rust typed config parsing ignores non-string `automergeType` values instead of exposing raw migration output |

---

