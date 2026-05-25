# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/buildpacks-registry/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/buildpacks-registry/index.spec.ts
**Total tests:** 3 | **Ported:** 3 | **Actionable:** 3 | **Status:** done

### `modules/datasource/buildpacks-registry/index › getReleases`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| processes real data | 9 | ported | `crates/renovate-core/src/datasources/buildpacks_registry.rs` | `processes_real_data` | versions sorted ascending; sourceUrl from latest.homepage |
| returns null on empty result | 48 | ported | `crates/renovate-core/src/datasources/buildpacks_registry.rs` | `returns_null_on_empty_result` | `{}` → None |
| handles not found | 57 | ported | `crates/renovate-core/src/datasources/buildpacks_registry.rs` | `handles_not_found` | 404 → None |

---

