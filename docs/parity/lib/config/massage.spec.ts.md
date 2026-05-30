# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/config/massage.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/massage.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 0 | **Status:** done

### `config/massage › massageConfig`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns empty | 6 | ported | `massage.rs` | `massage_config_returns_empty` | — |
| massages strings to array | 12 | ported | `massage.rs` | `massage_config_converts_allowed_string_to_array` | — |
| normalizes zero minimumReleaseAge to null | 20 | ported | `massage.rs` | `massage_config_normalizes_zero_minimum_release_age` | — |
| normalizes zero minimumReleaseAge in packageRules | 30 | ported | `massage.rs` | `massage_config_normalizes_zero_minimum_release_age_in_package_rules` | — |
| massages packageRules matchUpdateTypes | 58 | ported | `massage.rs` | `massage_config_expands_package_rule_update_types` | — |
| filters packageRules with only match/exclude | 95 | ported | `massage.rs` | `massage_config_filters_package_rules_with_only_match_or_exclude` | — |
| does not massage lockFileMaintenance | 110 | ported | `massage.rs` | `massage_config_does_not_expand_lock_file_maintenance` | — |

---

