# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/post-update/rules.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/post-update/rules.spec.ts
**Total tests:** 6 | **Ported:** 3 | **Actionable:** 6 | **Status:** partial

### `processHostRules()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty if no rules | 10 | ported | `extractors/npm.rs` | `process_host_rules_empty` | — |
| returns empty if no resolvedHost | 16 | ported | `extractors/npm.rs` | `process_host_rules_no_resolved_host` | — |
| returns rules content | 23 | ported | `extractors/npm.rs` | `process_host_rules_username_password` | — |
| returns mixed rules content | 64 | pending | — | — | — |
| uses rules without host type | 146 | pending | — | — | — |
| deduplicates host rules while prefering npm type ones | 167 | pending | — | — | — |

---

