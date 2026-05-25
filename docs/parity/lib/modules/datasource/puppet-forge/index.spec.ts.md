# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/puppet-forge/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/puppet-forge/index.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** done

### `modules/datasource/puppet-forge/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should use default forge if no other provided | 12 | ported | `crates/renovate-core/src/datasources/puppet_forge.rs` | `uses_default_forge` | puppetlabs/apache → 4 releases sorted ascending |
| parses real data | 34 | ported | `crates/renovate-core/src/datasources/puppet_forge.rs` | `parses_real_data` | created_at "YYYY-MM-DD HH:MM:SS ±HHMM" → ISO UTC |
| has a deprecated for reason | 79 | ported | `crates/renovate-core/src/datasources/puppet_forge.rs` | `has_deprecated_for_reason` | deprecated_for → deprecation_message |
| should return null if lookup fails 400 | 107 | ported | `crates/renovate-core/src/datasources/puppet_forge.rs` | `returns_null_for_400` | 400 → None |
| should return null if lookup fails | 123 | ported | `crates/renovate-core/src/datasources/puppet_forge.rs` | `returns_null_for_404` | 404 → None |
| should fetch package info from custom registry | 137 | ported | `crates/renovate-core/src/datasources/puppet_forge.rs` | `fetches_from_custom_registry` | custom registry URL works |
| load all possible null values | 182 | ported | `crates/renovate-core/src/datasources/puppet_forge.rs` | `loads_null_values` | null endorsement/deprecated_for handled |
| no releases available -> return null | 208 | ported | `crates/renovate-core/src/datasources/puppet_forge.rs` | `returns_null_for_no_releases` | empty releases array → None |

---
