# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/modules/manager/npm/extract/yarnrc.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/npm/extract/yarnrc.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 0 | **Status:** done

### `modules/manager/npm/extract/yarnrc › resolveRegistryUrl()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| considers default registry | 10 | ported | `npm.rs` | `yarnrc_resolve_registry_url_considers_default_registry` | — |
| chooses matching scoped registry over default registry | 17 | ported | `npm.rs` | `yarnrc_resolve_registry_url_prefers_matching_scope` | — |
| ignores non matching scoped registry | 29 | ported | `npm.rs` | `yarnrc_resolve_registry_url_ignores_non_matching_scope` | — |
| ignores partial scope match | 40 | ported | `npm.rs` | `yarnrc_resolve_registry_url_ignores_partial_scope_match` | — |
| ignores missing scope registryServer | 51 | ported | `npm.rs` | `yarnrc_resolve_registry_url_ignores_missing_scope_registry_server` | — |

### `modules/manager/npm/extract/yarnrc › loadConfigFromYarnrcYml()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| produces expected config (%s) | 63 | ported | `npm.rs` | `load_config_from_yarnrc_yml_produces_expected_config` | — |

### `modules/manager/npm/extract/yarnrc › loadConfigFromLegacyYarnrc()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| produces expected config (%s) | 117 | ported | `npm.rs` | `load_config_from_legacy_yarnrc_produces_expected_config` | — |

---

