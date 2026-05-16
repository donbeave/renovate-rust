# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/package-rules/dep-names.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/dep-names.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 4 | **Status:** ported

### `util/package-rules/dep-names › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return false if packageFile is not defined | 7 | ported | `package_rule.rs` | `dep_name_matcher_returns_false_if_dep_name_is_empty` | Rust `DepContext` carries a string dep name; empty string covers the missing depName case |
| should return false if depName is excluded prefix | 19 | ported | `package_rule.rs` | `dep_name_matcher_returns_false_if_dep_name_is_excluded_prefix` | — |
| should return true if depName is included prefix | 42 | ported | `package_rule.rs` | `dep_name_matcher_returns_true_if_dep_name_is_included_prefix` | — |
| should return false if for wrong prefix | 65 | ported | `package_rule.rs` | `dep_name_matcher_returns_false_for_wrong_prefix` | — |

---

