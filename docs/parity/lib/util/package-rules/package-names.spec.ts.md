# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/package-rules/package-names.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/package-names.spec.ts
**Total tests:** 4 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `util/package-rules/package-names › match`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return false if packageName is not defined | 7 | ported | `package_rule.rs` | `package_name_matcher_returns_false_if_package_name_is_empty` | Rust `PackageRule::name_matches` carries a string package name; empty string covers the missing packageName case |
| should return false if not matching | 19 | ported | `package_rule.rs` | `package_name_matcher_returns_false_if_not_matching` | — |
| should matchPackageName | 32 | ported | `package_rule.rs` | `package_name_matcher_matches_package_name` | — |
| should match pattern | 44 | ported | `package_rule.rs` | `package_name_matcher_matches_regex_pattern` | — |

---

